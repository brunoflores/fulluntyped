(* Relocation information *)

open Const
open Buffcode

type info =
  | Reloc_literal of struct_constant  (** structured constant *)
  | Reloc_getglobal of qualified_ident  (** reference to a global *)
  | Reloc_setglobal of qualified_ident  (** definition of a global *)
  | Reloc_tag of qualified_ident * int  (** exception tag*)
  | Reloc_primitive of string  (** C primitive number *)

let reloc_info = ref ([] : (info * int) list)
let reset () = reloc_info := []
let enter info = reloc_info := (info, !out_position) :: !reloc_info

let slot_for_literal sc =
  enter (Reloc_literal sc);
  out_short 0

and slot_for_c_prim name =
  enter (Reloc_primitive name);
  out_short 0
