open Core

let set_memory8 mem addr value = mem.(addr) <- value land 0xFF; mem
let set_memory32 mem addr value =
  List.range 0 4
  |> List.fold_left ~init:mem ~f:(fun mem i -> set_memory8 mem (addr + i) (value lsr (i * 8)))
  |> (fun mem -> Printf.printf "set_memory32: value=%d, mem[addr]=%d\n" value mem.(addr); mem)

let set_register32 reg r_ix value =
  Register.update_register reg r_ix value

let calc_mem_address reg disp modrm =
  match Code.(modrm.md) with
  | 0 ->
     begin
       match modrm.rm with
       | 4 -> failwith "not implemented: modrm.md = 0 and modrm.rm = 4"
       | 5 -> disp
       | _ -> Register.get_register reg modrm.rm
     end
  | 1 ->
     begin
       match modrm.rm with
       | 4 -> failwith "not implemented: modrm.md = 1 and modrm.rm = 4"
       | _ -> (Register.get_register reg modrm.rm) + disp
     end
  | 2 ->
     begin
       match modrm.rm with
       | 4 -> failwith "not implemented: modrm.md = 2 and modrm.rm = 4"
       | _ -> (Register.get_register reg modrm.rm) + disp
     end
  | x -> failwith @@ Printf.sprintf "not implemented: modrm.md = %d" x

let set_rm32 (reg, mem) disp modrm value =
  if Code.(modrm.md) = 3 then
    (set_register32 reg modrm.rm value, mem)
  else
    let addr = calc_mem_address reg (Option.value_exn disp) modrm in
    (reg, set_memory32 mem addr value)

let set_r32 (reg, mem) modrm value =
  (set_register32 reg Code.(modrm.reg) value, mem)

let get_r32 reg modrm =
  Register.get_register reg Code.(modrm.reg)

let get_rm32 disp modrm (reg, mem) =
  if Code.(modrm.md) = 3 then Register.get_register reg modrm.rm
  else let addr = calc_mem_address reg (Option.value_exn disp) modrm in
       Memory.get_memory32 mem addr

let mov_r32_imm32 r_ix value (reg, mem): Register.t * Memory.t =
  (Register.(update_register reg r_ix value), mem)

let mov_rm32_imm32 disp modrm value (reg, mem): Register.t * Memory.t =
  set_rm32 (reg, mem) disp modrm value

let mov_rm32_r32 disp modrm (reg, mem): Register.t * Memory.t =
  let v = get_r32 reg modrm in
  set_rm32 (reg, mem) disp modrm v

let mov_r32_rm32 disp modrm (reg, mem): Register.t * Memory.t =
  let v = get_rm32 disp modrm (reg, mem) in
  set_r32 (reg, mem) modrm v

let short_jump diff (reg, mem): Register.t * Memory.t =
  (Register.inc_pc diff reg, mem)

let near_jump diff (reg, mem): Register.t * Memory.t =
  (Register.inc_pc diff reg, mem)

let add_rm32_r32 disp modrm (reg, mem): Register.t * Memory.t =
  let r32 = get_r32 reg modrm in
  let rm32 = get_rm32 disp modrm (reg, mem) in
  set_rm32 (reg, mem) disp modrm (rm32 + r32)

let sub_rm32_imm8 disp modrm value (reg, mem): Register.t * Memory.t =
  let rm32 = get_rm32 disp modrm (reg, mem) in
  set_rm32 (reg, mem) disp modrm (rm32 - value)

let inc_rm32 disp modrm (reg, mem): Register.t * Memory.t =
  let value = get_rm32 disp modrm (reg, mem) in
  set_rm32 (reg, mem) disp modrm (value + 1)

let push32 (reg, mem) value =
  let addr = (Register.get_register reg Const.n_esp) - 4 in
  (set_register32 reg Const.n_esp addr, set_memory32 mem addr value)

let pop32 (reg, mem) =
  let addr = Register.get_register reg Const.n_esp in
  let ret  = Memory.get_memory32 mem addr in
  set_register32 reg Const.n_esp (addr + 4), ret

let push_r32 r_ix (reg, mem): Register.t * Memory.t =
  let v = Register.get_register reg r_ix in
  push32 (reg, mem) v

let pop_r32 r_ix (reg, mem): Register.t * Memory.t =
  let reg', v = pop32 (reg, mem) in
  (set_register32 reg' r_ix v, mem)

let call_rel32 diff (reg, mem): Register.t * Memory.t =
  let reg, mem = push32 (reg, mem) (reg.eip) in
  (Register.inc_pc diff reg, mem)

let ret (reg, mem): Register.t * Memory.t =
  let reg', v = pop32 (reg, mem) in
  ({reg' with eip = v}, mem)

let leave (reg, mem): Register.t * Memory.t =
  let ebp = Register.get_register reg Const.n_ebp in
  let reg = set_register32 reg Const.n_esp ebp in
  let reg, v = pop32 (reg, mem) in
  let reg = set_register32 reg Const.n_ebp v in
  (reg, mem)

let of_code (code: Code.t) =
  match code.opcode with
  | Code.Short_jump -> short_jump (Option.value_exn code.imid)
  | Near_jump -> near_jump (Option.value_exn code.imid)
  | Mov_r32_imm32 r_ix -> mov_r32_imm32 r_ix (Option.value_exn code.imid)
  | Mov_rm32_imm32 ->
     mov_rm32_imm32 code.disp (Option.value_exn code.modrm) (Option.value_exn code.imid)
  | Mov_rm32_r32 -> mov_rm32_r32 code.disp (Option.value_exn code.modrm)
  | Mov_r32_rm32 -> mov_r32_rm32 code.disp (Option.value_exn code.modrm)
  | Add_rm32_r32 -> add_rm32_r32 code.disp (Option.value_exn code.modrm)
  | Sub_rm32_imm8 ->
     sub_rm32_imm8 code.disp (Option.value_exn code.modrm) (Option.value_exn code.imid)
  | Inc_rm32 -> inc_rm32 code.disp (Option.value_exn code.modrm)
  | Push_r32 r_ix -> push_r32 r_ix
  | Pop_r32 r_ix -> pop_r32 r_ix
  | Call_rel32 -> call_rel32 (Option.value_exn code.imid)
  | Ret -> ret
  | Leave -> leave
  | Dummy_code -> failwith "This may be bug (Dummy code is used)"
