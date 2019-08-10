open Core

let main machine filename =
  let (module Machine: Emulator.MachineType) = match machine with
    | Config.I386small -> (module I386.Instance: Emulator.MachineType)
    | PC98VM -> failwith "unimplemented"
  in
  let module Emulator = Emulator.Make(Machine) in
  (Emulator.mk_emulator ()
   |> Emulator.load_program filename
   |> Emulator.run) ()

let machine =
  Command.Arg_type.create (function
      | "i386s" -> Config.I386small | "pc98vm" -> PC98VM
      | arg -> failwith @@ Printf.sprintf "invalid machine type %s" arg)

let command =
  Command.basic
    ~summary: "X86 Emulator"
    ~readme:(fun () -> "X86 Emulator (more information)")
    Command.Let_syntax.(
    let open Command.Param in
    let%map
          machine = anon ("machine" %: machine)
    and filename = anon ("filename" %: string) in
    fun () -> main machine filename)

let () =
  Command.run ~version:"1.0" ~build_info:"YS" command
