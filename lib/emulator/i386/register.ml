open Util.Util32bit

type general = {eax: uint32; ebx: uint32; ecx: uint32;
                edx: uint32; esi: uint32; edi: uint32;
                esp: uint32; ebp: uint32;}
type eflags = uint32
type eip = uint32

type t = {general:general; eflags: eflags; eip:eip;}

let create eip esp =
  let general = {eax = 0; ebx = 0; ecx = 0;
                 edx = 0; esi = 0; edi = 0;
                 esp = esp; ebp = 0;} in
  let eflags = 0 in
  let eip = eip in
  {general=general; eflags=eflags; eip=eip}

let dump (reg: t) =
  Printf.printf
    ("EAX = 0x%08x\nECX = 0x%08x\nEDX = 0x%08x\n"
     ^^ "EBX = 0x%08x\nESP = 0x%08x\nEBP = 0x%08x\n"
     ^^ "ESI = 0x%08x\nEDI = 0x%08x\nEIP = 0x%08x\n")
    reg.general.eax reg.general.ecx reg.general.edx
    reg.general.ebx reg.general.esp reg.general.ebp
    reg.general.esi reg.general.edi reg.eip

let update_general (g: general) r_idx data =
  let open Const in
  if r_idx = n_eax then {g with eax = data}
  else if r_idx = n_ecx then {g with ecx = data}
  else if r_idx = n_edx then {g with edx = data}
  else if r_idx = n_ebx then {g with ebx = data}
  else if r_idx = n_esp then {g with esp = data}
  else if r_idx = n_ebp then {g with ebp = data}
  else if r_idx = n_esi then {g with esi = data}
  else if r_idx = n_edi then {g with edi = data}
  else failwith @@ Printf.sprintf "Register %x does not exist" r_idx

let update_register t r_idx data =
  {t with general = update_general t.general r_idx data}

let inc_pc diff (t:t) = {t with eip = t.eip+diff}
