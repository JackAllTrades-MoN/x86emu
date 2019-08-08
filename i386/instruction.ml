open Core
open Types

let to_signed8 x =
  let sign = x lsr 7 in
  let body = lnot (x - 1) |> (land) 0xFF in
  if sign = 0 then x else (-1) * body

let get_code8 emu ix = emu.memory.(emu.eip + ix)
let get_sign_code8 emu ix = emu.memory.(emu.eip + ix) |> to_signed8
let get_code32 emu ix =
  List.range 0 4
  |> List.fold ~init:0 ~f:(fun acc i -> (acc + get_code8 emu (ix + i) lsl (i * 8)))

let mov_r32_imm32 emu =
  (* opecode is of the form 0xb8 + r *)
  let reg = (get_code8 emu 0) - 0xB8 in
  let value = get_code32 emu 1 in
  {emu with registers = update_register emu.registers reg value;
            eip = emu.eip + 5}

let short_jump emu =
  let diff = get_sign_code8 emu 1 in
  {emu with eip = emu.eip + diff + 2}
