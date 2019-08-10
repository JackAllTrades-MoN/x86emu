open Core

let mov_r32_imm32 r_ix value (t: State.t): State.t =
  (* opecode is of the form 0xb8 + r *)
  let register' = Register.(update_register t.register r_ix value |> inc_pc 5) in
  {t with register = register'}

let short_jump (t: State.t): State.t =
  let mem, reg = t.memory, t.register in
  let diff = Code.get_sign_code8 mem reg 1 in
  {t with register = t.register |> Register.inc_pc (diff + 2)}

let of_code (code: Code.t) =
  match code.opcode with
  | Code.Short_jump -> short_jump
  | Mov_r32_imm32 r_ix ->
     match code.imid with
     | None -> failwith "unimplemented"
     | Some imid -> mov_r32_imm32 r_ix imid
