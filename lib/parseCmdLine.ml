let (>>) f1 f2 = fun x -> f2 (f1 x)
let (<<) f1 f2 = fun x -> f1 (f2 x)

let update_gconfig f = Global.config := f !Global.config

let args =
  let open Arg in
  [("-machine",
    String (Config.(machine_of_str >> update_machine) >> update_gconfig),
    "target machine [i386s/pc98vm] (default: i386s)");
   ("-enable-dbg-mode",
    Unit (Config.enable_dbg_mode >> update_gconfig),
    "enable debug mode")]

let useage_msg = "X86 Emulator"

let parse_and_run main () = Arg.parse args main useage_msg
