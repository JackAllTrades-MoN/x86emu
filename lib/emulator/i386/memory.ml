open Core
open Util.Util8bit

type t = uint8 array

let create memory_size = Array.create ~len:memory_size 0

let load_program (memory: t) ~offset:offset data =
  if (List.length data) + offset >= Array.length memory
  then failwith "Data is bigger than memory size";
  List.foldi data ~init:memory ~f:(fun i mem d -> mem.(i + offset) <- d; mem)

let get_memory8 (mem: t) addr = mem.(addr)

let get_memory32 (mem: t) addr =
  List.range 0 4
  |> List.fold_left ~init:0 ~f:(fun acc i -> acc lor (get_memory8 mem (addr + i) lsl (8 * i)))
  |> (fun res -> Printf.printf "get_memory32[%d]: val=%d\n" addr res; res)
