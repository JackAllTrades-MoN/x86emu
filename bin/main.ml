open Core

let main machine filename =
  let run = match machine with
    | Config.I386small -> I386.Emulator.(mk_emulator () |> load_program filename |> run)
    | PC98VM -> failwith "unimplemented"
  in run ()

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
  let open Hardware.Monitor in
(*  exec (); *)
  Command.run ~version:"1.0" ~build_info:"YS" command
