open Core

type t = State.t

let fetch_and_decode (t: t): Code.t =
  Code.get_code8 t.memory t.register 0
  |> function
    | 0xEB -> {Code.prefix=None; opcode=Short_jump; modrm=None; sib=None; disp=None; imid=None}
    | x when 0xB8 <= x && x < 0xB8 + Const.register_count ->
       let target = (Code.get_code8 t.memory t.register 0) - 0xB8 in
       let imid = Code.get_code32 t.memory t.register 1 in
       {Code.prefix=None; opcode=(Mov_r32_imm32 target); modrm=None; sib=None; disp=None; imid=(Some imid)}
    | byte -> failwith @@ Printf.sprintf "unimplemented code: %x" byte

let build () =
  let mem_size, eip, esp = 100, 0x0000, 0x7c00 in
  {State.register=Register.create eip esp; memory=Memory.create mem_size}

let load_program (bin:Binary.BinaryFile.t) (t: t) =
  if Array.length t.memory <= List.length bin then failwith "Given Program is too big"
  else
    let memory' = t.memory in
    List.iteri bin ~f:(fun i b -> memory'.(i) <- b);
    {t with memory = memory'}

let dump_information (t: t) = Register.dump t.register

let is_out_of_memory (t: t): bool = t.register.eip > Array.length t.memory

let exec (t: t): t =
  let code = fetch_and_decode t in
  let inst = Instruction.of_code code in
  inst t

let pc (t: t) = t.register.eip
