open Core

module type MachineType = sig
  type t
  val build: unit -> t
  val load_program: Binary.BinaryFile.t -> t -> t
  val dump_information: t -> unit
  val is_out_of_memory: t -> bool
  val exec: t -> t
  val pc: t -> int
end

module type EmulatorType = sig
  type t
  val mk_emulator: unit -> t
  val dump_information: t -> unit
  val load_program: string -> t -> t
  val run: t -> unit -> unit
end

module Make(Machine: MachineType): EmulatorType = struct
  type t = Machine.t

  let mk_emulator () = Machine.build ()

  let dump_information = Machine.dump_information

  let load_program filename (state: t) =
    let binary = Binary.BinaryFile.read filename in
    Machine.load_program binary state

  let run emu () =
    let rec main_loop (state: t) =
      if Machine.is_out_of_memory state then failwith "Out of Memory"
      else
        let state' = Machine.exec state in
        if Machine.pc state' = 0x00 then state' else main_loop state'
    in
    main_loop emu |> dump_information
end
