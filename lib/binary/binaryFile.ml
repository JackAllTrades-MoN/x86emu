open Core

type t = int list

let read filename =
  let binary = In_channel.create filename in
  let rec inner res =
    match In_channel.input_byte binary with
    | None -> res
    | Some b -> inner (b::res)
  in inner [] |> List.rev
