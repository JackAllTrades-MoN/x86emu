module type EmulatorType = sig
  type t
  val is_out_of_memory: t -> bool
  val fetch_and_decode: t -> t -> t
  val at_address_zero: t -> bool
  val create_emu: int -> int -> int -> t
  val dump_information: t -> unit
end

module Make(E: EmulatorType) = struct
  type t = E.t
  let run () =
    let rec main_loop (emu:t) =
      if E.is_out_of_memory emu then failwith "Out of Memory"
      else
        let inst = E.fetch_and_decode emu in
        let emu' = inst emu in
        if E.at_address_zero emu' then emu' else main_loop emu'
    in
    let memory_size, eip, esp = 100, 0x0000, 0x7c00 in
    main_loop (E.create_emu memory_size eip esp)
    |> E.dump_information
end
