open Core
open Util.Util8bit
open Register

type prefix = int
type opcode = Short_jump | Mov_r32_imm32 of int
type modrm  = int
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

let get_code8 mem reg ix = mem.(reg.eip + ix)
let get_sign_code8 mem reg ix = mem.(reg.eip + ix) |> to_signed8
let get_code32 mem reg ix =
  List.range 0 4
  |> List.fold ~init:0 ~f:(fun acc i -> (acc + get_code8 mem reg (ix + i) lsl (i * 8)))
