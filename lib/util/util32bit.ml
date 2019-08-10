type uint32 = int (* TODO: optimize *)

let to_signed32 x =
  let sign = x lsr 31 in
  let body = lnot (x - 1) |> (land) 0xFFFFFFFF in
  if sign = 0 then x else (-1) * body
