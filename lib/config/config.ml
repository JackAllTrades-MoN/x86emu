type t = {machine: machine; dbg_mode: bool;}
and
machine = I386small (* CPU: I386, MemorySize 100 * 8bit *)
        | PC98VM    (* CPU: V30/10MHz, MemorySize:   *)


let default = {
    machine  = I386small;
    dbg_mode = false;
  }

let str_of_machine = function I386small -> "i386s" | PC98VM -> "pc98vm"
let machine_of_str = function
  | "i386s" -> I386small | "pc98vm" -> PC98VM | x -> failwith @@ "Invalid machine type: " ^ x

let update_machine machine (conf: t) = {conf with machine=machine}

let enable_dbg_mode () (conf: t) = {conf with dbg_mode = true}
