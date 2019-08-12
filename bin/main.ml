open Core
open X86emulator

let main filename =
  let run = match !Global.config.machine with
    | Config.I386small -> I386.Emulator.(mk_emulator () |> load_program filename |> run)
    | PC98VM -> failwith "unimplemented"
  in run ()

let () = ParseCmdLine.parse_and_run main ()
