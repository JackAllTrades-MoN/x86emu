open Core

type m_state = Register.t * Memory.t
type 'a t = m_state -> 'a * m_state

let return x s = (x, s)
let get s = (s, s)
let put x _ = ((), x)
let run_state x s = x s
let (>>=) x f = fun s ->
  let a, new_state = x s in
  let g = f a in g new_state

let inc_pc x s =
  let s' = (fst s |> Register.inc_pc x, snd s) in ((), s')

let parse_modrm code =
  get >>= fun (reg, mem) ->
  let modrm_code = Code.get_code8 mem reg 0 in
  let md = (modrm_code land 0xc0) lsr 6 in
  let reg = (modrm_code land 0x38) lsr 3 in
  let rm = modrm_code land 0x07 in
  let code' = Code.update_modrm {md=md; reg=reg; rm=rm;} code in
  inc_pc 1 >>= fun () -> return code'

let update_sid code =
  get >>= fun (reg, mem) ->
  match Code.(code.modrm) with
  | None -> return code
  | Some modrm ->
     if modrm.md <> 3 && modrm.rm = 4 then
       let sid = Code.get_code8 mem reg 0 in
       inc_pc 1 >>= fun () ->
       return @@ Code.update_sib sid code
     else return code

let update_disp code =
  get >>= fun (reg, mem) ->
  match Code.(code.modrm) with
  | None -> return code
  | Some modrm ->
     if (modrm.md = 0 && modrm.rm = 5) || modrm.md = 2 then
       let disp = Code.get_sign_code32 mem reg 0 in
       inc_pc 4 >>= fun () -> return @@ Code.update_disp disp code
     else if modrm.md = 1 then
       let disp = Code.get_sign_code8 mem reg 0 in
       inc_pc 1 >>= fun () -> return @@ Code.update_disp disp code
     else return code

let update_modrm code = parse_modrm code >>= update_sid >>= update_disp

let fetch_and_decode =
  get >>= fun (reg, mem) ->
  match Code.get_code8 mem reg 0 with
  | 0x01 ->
     inc_pc 1 >>= fun () ->
     update_modrm Code.(update_opcode Add_rm32_r32 empty)
  | x when 0x50 <= x && x < 0x50 + Const.register_count ->
     let r_ix = x - 0x50 in
     inc_pc 1 >>= fun () ->
     return Code.(empty |> update_opcode (Push_r32 r_ix))
  | x when 0x58 <= x && x < 0x58 + Const.register_count ->
     let r_ix = x - 0x58 in
     inc_pc 1 >>= fun () ->
     return Code.(empty |> update_opcode (Pop_r32 r_ix))
  | 0x83 ->
     inc_pc 1 >>= fun () ->
     update_modrm Code.empty
     >>= fun code ->
     let modrm = Option.value_exn Code.(code.modrm) in
     if modrm.reg = 5 then
       get >>= fun (reg, mem) ->
       let imid = Code.get_sign_code8 mem reg 0 in
       inc_pc 1 >>= fun () ->
       return Code.(update_opcode Sub_rm32_imm8 code |> update_imid imid)
     else failwith @@ Printf.sprintf "not implemented: 83 /%d" modrm.reg
  | 0x89 ->
     inc_pc 1 >>= fun () ->
     update_modrm Code.(update_opcode Mov_rm32_r32 empty)
  | 0x8B ->
     inc_pc 1 >>= fun () ->
     update_modrm Code.(update_opcode Mov_r32_rm32 empty)
  | x when 0xB8 <= x && x < 0xB8 + Const.register_count ->
     let target = (Code.get_code8 mem reg 0) - 0xB8 in
     let imid = Code.get_code32 mem reg 1 in
     inc_pc 5 >>= fun () ->
     return {Code.prefix=None; opcode=(Mov_r32_imm32 target); modrm=None; sib=None; disp=None; imid=(Some imid)}
  | 0xC7 ->
     inc_pc 1 >>= fun () ->
     update_modrm Code.(update_opcode Mov_rm32_imm32 empty) >>= fun code ->
     get >>= fun (reg, mem) ->
     let value = Code.get_code32 mem reg 0 in
     inc_pc 4 >>= fun () ->
     return (Code.update_imid value code)
  | 0xC9 ->
     inc_pc 1 >>= fun () -> return Code.(empty |> update_opcode Leave)
  | 0xE8 ->
     let imid = Code.get_sign_code32 mem reg 1 in
     inc_pc 5 >>= fun () ->
     return Code.(empty |> update_opcode Call_rel32 |> update_imid imid)
  | 0xC3 -> return Code.(empty |> update_opcode Ret)
  | 0xE9 ->
     let imid = Code.get_sign_code32 mem reg 1 in
     inc_pc 5 >>= fun () -> return @@ Code.(empty |> update_opcode Near_jump |> update_imid imid)
  | 0xEB ->
     let imid = Code.get_sign_code8 mem reg 1 in
     inc_pc 2 >>= fun () ->
     return {Code.prefix=None; opcode=Short_jump; modrm=None; sib=None; disp=None; imid=Some imid}
  | 0xFF ->
     inc_pc 1 >>= fun () ->
     update_modrm Code.empty
     >>= fun code ->
     let modrm = Option.value_exn Code.(code.modrm) in
     if modrm.reg = 0 then
       return Code.(update_opcode Inc_rm32 code)
     else failwith @@ Printf.sprintf "not implemented: FF /%d" modrm.reg
  | byte -> failwith @@ Printf.sprintf "unimplemented code: %x" byte

let exec code =
  get >>= fun (reg, mem) ->
  let inst = Instruction.of_code code in
  put (inst (reg, mem))

let mk_emulator () =
  let mem_size, eip, esp = (1024 * 1024), 0x7c00, 0x7c00 in
  (Register.create eip esp, Memory.create mem_size)

let load_program filename (reg, mem) =
  let binary = Binary.BinaryFile.read filename in
  (reg, Memory.load_program ~offset:0x7c00 mem binary)

let dump_imformation (reg, _) = Register.dump reg

let run emu () =
  let rec main_loop () =
    get >>= fun (reg, _) ->
    print_endline @@ "pc: " ^ (string_of_int Register.(reg.eip));
    fetch_and_decode >>= fun code ->
    print_endline "code: ";
    Code.dump code;
    exec code
    >>= fun () ->
    get >>= fun (reg, _) ->
    print_endline @@ "pc': " ^ (string_of_int Register.(reg.eip));
    if Register.(reg.eip) = 0x00 then return ()
    else main_loop ()
  in
  run_state (main_loop ()) emu
  |> snd
  |> dump_imformation
