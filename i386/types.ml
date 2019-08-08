type uint32_t = int (* TODO: optimize *)
type uint8_t = int  (* TODO: optimize *)

type registers = {eax: uint32_t;
                  ebx: uint32_t;
                  ecx: uint32_t;
                  edx: uint32_t;
                  esi: uint32_t;
                  edi: uint32_t;
                  esp: uint32_t;
                  ebp: uint32_t;}
type eflags = uint32_t
type memory = uint8_t array
type eip = uint32_t (* program counter *)

type t = {registers: registers; eflags: eflags; memory: memory; eip: eip;}

type instruction = t -> t
