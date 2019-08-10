type uint8 = int (* TODO: optimize *)

let to_signed8 x =
  let sign = x lsr 7 in
  let body = lnot (x - 1) |> (land) 0xFF in
  if sign = 0 then x else (-1) * body
