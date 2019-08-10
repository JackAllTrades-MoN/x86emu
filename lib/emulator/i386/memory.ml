open Core
open Util.Util8bit

type t = uint8 array

let create memory_size = Array.create ~len:memory_size 0

let load_program (memory: t) data =
  if List.length data >= Array.length memory
  then failwith "Data is bigger than memory size";
  List.to_array data
