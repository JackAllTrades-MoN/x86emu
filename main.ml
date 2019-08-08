open Core

let memory_size = 100

let main filename =
  let emu = I386.Emulator.create_emu memory_size 0x0000 0x7c00 in
  let binary = BinaryFile.read filename in
  I386.Emulator.(update_memory emu binary |> run); ()

let filename_param = Command.Param.(anon ("filename" %: string))

let command =
  Command.basic
    ~summary: "X86 Emulator"
    ~readme:(fun () -> "X86 Emulator (more information)")
    (Command.Param.map filename_param ~f:(fun filename ->
       (fun () -> main filename)))

let () =
  Command.run ~version:"1.0" ~build_info:"YS" command
