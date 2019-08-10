open Core
open Util.Util8bit
open Util.Util32bit
open Register

type prefix = int
type opcode =
  | Dummy_code
  | Short_jump | Near_jump
  | Mov_r32_imm32 of int | Mov_rm32_imm32 | Mov_rm32_r32 | Mov_r32_rm32
  | Add_rm32_r32
  | Sub_rm32_imm8
  | Inc_rm32
type modrm  = {md: int; reg: int; rm: int;}
type sib    = int
type disp   = int
type imid   = int
type t = {prefix:prefix option;
          opcode:opcode;
          modrm: modrm option;
          sib: sib option;
          disp: disp option;
          imid: imid option;
         }

let empty = {prefix=None; opcode=Dummy_code; modrm=None; sib=None; disp=None; imid=None}
let update_opcode opcode t = {t with opcode = opcode}
let update_modrm modrm t = {t with modrm = Some modrm}
let update_sib sib t = {t with sib = Some sib}
let update_disp disp t = {t with disp = Some disp}
let update_imid imid t = {t with imid = Some imid}

let get_code8 mem reg ix = mem.(reg.eip + ix)
let get_sign_code8 mem reg ix = get_code8 mem reg ix |> to_signed8
let get_code32 mem reg ix =
  List.range 0 4
  |> List.fold ~init:0 ~f:(fun acc i -> (acc + get_code8 mem reg (ix + i) lsl (i * 8)))
let get_sign_code32 mem reg ix =
  get_code32 mem reg ix |> to_signed32

let str_of_opt f x = match x with None -> "None" | Some x -> f x
let str_of_prefix = string_of_int
let str_of_opcode = function
  | Dummy_code -> "Dummy_code" | Short_jump -> "Short_jump"
  | Near_jump -> "Near_jump" | Mov_r32_imm32 _ -> "Mov_r32_imm32"
  | Mov_rm32_imm32 -> "Mov_rm32_imm32" | Mov_rm32_r32 -> "Mov_rm32_r32"
  | Mov_r32_rm32 -> "Mov_r32_rm32" | Add_rm32_r32 -> "Add_rm32_r32"
  | Sub_rm32_imm8 -> "Sub_rm32_imm8" | Inc_rm32 -> "Inc_rm32"
let str_of_modrm modrm =
  Printf.sprintf "{md: %d; reg: %d; rm:%d}" modrm.md modrm.reg modrm.rm
let str_of_sib = string_of_int
let str_of_disp = string_of_int
let str_of_imid = string_of_int
let dump (code:t) =
  Printf.printf "{prefix: %s; opcode: %s; modrm: %s; sib: %s; disp: %s; imid: %s}\n"
    (str_of_opt str_of_prefix code.prefix)
    (str_of_opcode code.opcode)
    (str_of_opt str_of_modrm code.modrm)
    (str_of_opt str_of_sib code.sib)
    (str_of_opt str_of_disp code.disp)
    (str_of_opt str_of_imid code.imid)
