use std::fmt;

#[derive(Copy, Clone)]
pub enum Instruction {
  Constbyte,
  Constshort,
  Getglobal,
  Setglobal,
  Cur,
  Switch,
  Branch,
  Branchif,
  Branchifnot,
  Popbranchifnot,
  Branchifneqtag,
  Branchifeq,
  Branchifneq,
  Branchiflt,
  Branchifgt,
  Branchifle,
  Branchifge,
  Branchinterval,
  Ccall1,
  Ccall2,
  Ccall3,
  Ccall4,
  Ccall5,
  Ccalln,
  Makeblock,
  Makeblock1,
  Makeblock2,
  Makeblock3,
  Makeblock4,
  Tagof,
  Access,
  Acc0,
  Acc1,
  Acc2,
  Acc3,
  Acc4,
  Acc5,
  Atom,
  Atom0,
  Atom1,
  Atom2,
  Atom3,
  Atom4,
  Atom5,
  Atom6,
  Atom7,
  Atom8,
  Atom9,
  Getfield,
  Getfield0,
  Getfield1,
  Getfield2,
  Getfield3,
  Setfield,
  Setfield0,
  Setfield1,
  Setfield2,
  Setfield3,
  Stop,
  Checksignals,
  Apply,
  Return,
  Appterm,
  Grab,
  Let,
  Letrec1,
  Dummy,
  Update,
  Endlet,
  Endlet1,
  Pushtrap,
  Raise,
  Poptrap,
  Push,
  Pop,
  Pushmark,
  Pushgetglobalapply,
  Pushgetglobalappterm,
  Boolnot,
  Negint,
  Succint,
  Predint,
  Addint,
  Subint,
  Mulint,
  Divint,
  Modint,
  Andint,
  Orint,
  Xorint,
  Shiftleftint,
  Shiftrightintsigned,
  Shiftrightintunsigned,
  Eq,
  Neq,
  Ltint,
  Gtint,
  Leint,
  Geint,
  Incr,
  Decr,
  Floatop,
  Intoffloat,
  Eqfloat,
  Neqfloat,
  Ltfloat,
  Gtfloat,
  Lefloat,
  Gefloat,
  Stringlength,
  Getstringchar,
  Setstringchar,
  Eqstring,
  Neqstring,
  Ltstring,
  Gtstring,
  Lestring,
  Gestring,
  Makevector,
  Vectlength,
  Getvectitem,
  Setvectitem,
  Break,
  Floatofint,
  Negfloat,
  Addfloat,
  Subfloat,
  Mulfloat,
  Divfloat,
  Makestring,
}

// In Fish:
// cat instruct.h |
//   awk -F',' '/^  [A-Z]/ {
//     instr = substr(toupper($1), 3,1) substr(tolower($1), 4);
//     print "Instruction::" instr " => \"" instr "\","
//   }'
impl fmt::Display for Instruction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Instruction::Constbyte => "Constbyte",
        Instruction::Constshort => "Constshort",
        Instruction::Getglobal => "Getglobal",
        Instruction::Setglobal => "Setglobal",
        Instruction::Cur => "Cur",
        Instruction::Switch => "Switch",
        Instruction::Branch => "Branch",
        Instruction::Branchif => "Branchif",
        Instruction::Branchifnot => "Branchifnot",
        Instruction::Popbranchifnot => "Popbranchifnot",
        Instruction::Branchifneqtag => "Branchifneqtag",
        Instruction::Branchifeq => "Branchifeq",
        Instruction::Branchifneq => "Branchifneq",
        Instruction::Branchiflt => "Branchiflt",
        Instruction::Branchifgt => "Branchifgt",
        Instruction::Branchifle => "Branchifle",
        Instruction::Branchifge => "Branchifge",
        Instruction::Branchinterval => "Branchinterval",
        Instruction::Ccall1 => "Ccall1",
        Instruction::Ccall2 => "Ccall2",
        Instruction::Ccall3 => "Ccall3",
        Instruction::Ccall4 => "Ccall4",
        Instruction::Ccall5 => "Ccall5",
        Instruction::Ccalln => "Ccalln",
        Instruction::Makeblock => "Makeblock",
        Instruction::Makeblock1 => "Makeblock1",
        Instruction::Makeblock2 => "Makeblock2",
        Instruction::Makeblock3 => "Makeblock3",
        Instruction::Makeblock4 => "Makeblock4",
        Instruction::Tagof => "Tagof",
        Instruction::Access => "Access",
        Instruction::Acc0 => "Acc0",
        Instruction::Acc1 => "Acc1",
        Instruction::Acc2 => "Acc2",
        Instruction::Acc3 => "Acc3",
        Instruction::Acc4 => "Acc4",
        Instruction::Acc5 => "Acc5",
        Instruction::Atom => "Atom",
        Instruction::Atom0 => "Atom0",
        Instruction::Atom1 => "Atom1",
        Instruction::Atom2 => "Atom2",
        Instruction::Atom3 => "Atom3",
        Instruction::Atom4 => "Atom4",
        Instruction::Atom5 => "Atom5",
        Instruction::Atom6 => "Atom6",
        Instruction::Atom7 => "Atom7",
        Instruction::Atom8 => "Atom8",
        Instruction::Atom9 => "Atom9",
        Instruction::Getfield => "Getfield",
        Instruction::Getfield0 => "Getfield0",
        Instruction::Getfield1 => "Getfield1",
        Instruction::Getfield2 => "Getfield2",
        Instruction::Getfield3 => "Getfield3",
        Instruction::Setfield => "Setfield",
        Instruction::Setfield0 => "Setfield0",
        Instruction::Setfield1 => "Setfield1",
        Instruction::Setfield2 => "Setfield2",
        Instruction::Setfield3 => "Setfield3",
        Instruction::Stop => "Stop",
        Instruction::Checksignals => "Checksignals",
        Instruction::Apply => "Apply",
        Instruction::Return => "Return",
        Instruction::Appterm => "Appterm",
        Instruction::Grab => "Grab",
        Instruction::Let => "Let",
        Instruction::Letrec1 => "Letrec1",
        Instruction::Dummy => "Dummy",
        Instruction::Update => "Update",
        Instruction::Endlet => "Endlet",
        Instruction::Endlet1 => "Endlet1",
        Instruction::Pushtrap => "Pushtrap",
        Instruction::Raise => "Raise",
        Instruction::Poptrap => "Poptrap",
        Instruction::Push => "Push",
        Instruction::Pop => "Pop",
        Instruction::Pushmark => "Pushmark",
        Instruction::Pushgetglobalapply => "Pushgetglobalapply",
        Instruction::Pushgetglobalappterm => "Pushgetglobalappterm",
        Instruction::Boolnot => "Boolnot",
        Instruction::Negint => "Negint",
        Instruction::Succint => "Succint",
        Instruction::Predint => "Predint",
        Instruction::Addint => "Addint",
        Instruction::Subint => "Subint",
        Instruction::Mulint => "Mulint",
        Instruction::Divint => "Divint",
        Instruction::Modint => "Modint",
        Instruction::Andint => "Andint",
        Instruction::Orint => "Orint",
        Instruction::Xorint => "Xorint",
        Instruction::Shiftleftint => "Shiftleftint",
        Instruction::Shiftrightintsigned => "Shiftrightintsigned",
        Instruction::Shiftrightintunsigned => "Shiftrightintunsigned",
        Instruction::Eq => "Eq",
        Instruction::Neq => "Neq",
        Instruction::Ltint => "Ltint",
        Instruction::Gtint => "Gtint",
        Instruction::Leint => "Leint",
        Instruction::Geint => "Geint",
        Instruction::Incr => "Incr",
        Instruction::Decr => "Decr",
        Instruction::Floatop => "Floatop",
        Instruction::Intoffloat => "Intoffloat",
        Instruction::Eqfloat => "Eqfloat",
        Instruction::Neqfloat => "Neqfloat",
        Instruction::Ltfloat => "Ltfloat",
        Instruction::Gtfloat => "Gtfloat",
        Instruction::Lefloat => "Lefloat",
        Instruction::Gefloat => "Gefloat",
        Instruction::Stringlength => "Stringlength",
        Instruction::Getstringchar => "Getstringchar",
        Instruction::Setstringchar => "Setstringchar",
        Instruction::Eqstring => "Eqstring",
        Instruction::Neqstring => "Neqstring",
        Instruction::Ltstring => "Ltstring",
        Instruction::Gtstring => "Gtstring",
        Instruction::Lestring => "Lestring",
        Instruction::Gestring => "Gestring",
        Instruction::Makevector => "Makevector",
        Instruction::Vectlength => "Vectlength",
        Instruction::Getvectitem => "Getvectitem",
        Instruction::Setvectitem => "Setvectitem",
        Instruction::Break => "Break",
        Instruction::Floatofint => "Floatofint",
        Instruction::Negfloat => "Negfloat",
        Instruction::Addfloat => "Addfloat",
        Instruction::Subfloat => "Subfloat",
        Instruction::Mulfloat => "Mulfloat",
        Instruction::Divfloat => "Divfloat",
        Instruction::Makestring => "Makestring",
      }
    )
  }
}

// In Fish:
// cat instruct.h |
//   awk -F',' '/^  [A-Z]/ {
//     ++i;
//     print "Instruction::" substr(toupper($1), 3,1)
//     substr(tolower($1), 4) " => " i ","
//   }'
#[cfg(test)]
pub fn encode(i: &Instruction) -> i32 {
  match i {
    Instruction::Constbyte => 1,
    Instruction::Constshort => 2,
    Instruction::Getglobal => 3,
    Instruction::Setglobal => 4,
    Instruction::Cur => 5,
    Instruction::Switch => 6,
    Instruction::Branch => 7,
    Instruction::Branchif => 8,
    Instruction::Branchifnot => 9,
    Instruction::Popbranchifnot => 10,
    Instruction::Branchifneqtag => 11,
    Instruction::Branchifeq => 12,
    Instruction::Branchifneq => 13,
    Instruction::Branchiflt => 14,
    Instruction::Branchifgt => 15,
    Instruction::Branchifle => 16,
    Instruction::Branchifge => 17,
    Instruction::Branchinterval => 18,
    Instruction::Ccall1 => 19,
    Instruction::Ccall2 => 20,
    Instruction::Ccall3 => 21,
    Instruction::Ccall4 => 22,
    Instruction::Ccall5 => 23,
    Instruction::Ccalln => 24,
    Instruction::Makeblock => 25,
    Instruction::Makeblock1 => 26,
    Instruction::Makeblock2 => 27,
    Instruction::Makeblock3 => 28,
    Instruction::Makeblock4 => 29,
    Instruction::Tagof => 30,
    Instruction::Access => 31,
    Instruction::Acc0 => 32,
    Instruction::Acc1 => 33,
    Instruction::Acc2 => 34,
    Instruction::Acc3 => 35,
    Instruction::Acc4 => 36,
    Instruction::Acc5 => 37,
    Instruction::Atom => 38,
    Instruction::Atom0 => 39,
    Instruction::Atom1 => 40,
    Instruction::Atom2 => 41,
    Instruction::Atom3 => 42,
    Instruction::Atom4 => 43,
    Instruction::Atom5 => 44,
    Instruction::Atom6 => 45,
    Instruction::Atom7 => 46,
    Instruction::Atom8 => 47,
    Instruction::Atom9 => 48,
    Instruction::Getfield => 49,
    Instruction::Getfield0 => 50,
    Instruction::Getfield1 => 51,
    Instruction::Getfield2 => 52,
    Instruction::Getfield3 => 53,
    Instruction::Setfield => 54,
    Instruction::Setfield0 => 55,
    Instruction::Setfield1 => 56,
    Instruction::Setfield2 => 57,
    Instruction::Setfield3 => 58,
    Instruction::Stop => 59,
    Instruction::Checksignals => 60,
    Instruction::Apply => 61,
    Instruction::Return => 62,
    Instruction::Appterm => 63,
    Instruction::Grab => 64,
    Instruction::Let => 65,
    Instruction::Letrec1 => 66,
    Instruction::Dummy => 67,
    Instruction::Update => 68,
    Instruction::Endlet => 69,
    Instruction::Endlet1 => 70,
    Instruction::Pushtrap => 71,
    Instruction::Raise => 72,
    Instruction::Poptrap => 73,
    Instruction::Push => 74,
    Instruction::Pop => 75,
    Instruction::Pushmark => 76,
    Instruction::Pushgetglobalapply => 77,
    Instruction::Pushgetglobalappterm => 78,
    Instruction::Boolnot => 79,
    Instruction::Negint => 80,
    Instruction::Succint => 81,
    Instruction::Predint => 82,
    Instruction::Addint => 83,
    Instruction::Subint => 84,
    Instruction::Mulint => 85,
    Instruction::Divint => 86,
    Instruction::Modint => 87,
    Instruction::Andint => 88,
    Instruction::Orint => 89,
    Instruction::Xorint => 90,
    Instruction::Shiftleftint => 91,
    Instruction::Shiftrightintsigned => 92,
    Instruction::Shiftrightintunsigned => 93,
    Instruction::Eq => 94,
    Instruction::Neq => 95,
    Instruction::Ltint => 96,
    Instruction::Gtint => 97,
    Instruction::Leint => 98,
    Instruction::Geint => 99,
    Instruction::Incr => 100,
    Instruction::Decr => 101,
    Instruction::Floatop => 102,
    Instruction::Intoffloat => 103,
    Instruction::Eqfloat => 104,
    Instruction::Neqfloat => 105,
    Instruction::Ltfloat => 106,
    Instruction::Gtfloat => 107,
    Instruction::Lefloat => 108,
    Instruction::Gefloat => 109,
    Instruction::Stringlength => 110,
    Instruction::Getstringchar => 111,
    Instruction::Setstringchar => 112,
    Instruction::Eqstring => 113,
    Instruction::Neqstring => 114,
    Instruction::Ltstring => 115,
    Instruction::Gtstring => 116,
    Instruction::Lestring => 117,
    Instruction::Gestring => 118,
    Instruction::Makevector => 119,
    Instruction::Vectlength => 120,
    Instruction::Getvectitem => 121,
    Instruction::Setvectitem => 122,
    Instruction::Break => 123,
    Instruction::Floatofint => 124,
    Instruction::Negfloat => 125,
    Instruction::Addfloat => 126,
    Instruction::Subfloat => 127,
    Instruction::Mulfloat => 128,
    Instruction::Divfloat => 129,
    Instruction::Makestring => 130,
  }
}

// In Fish:
// cat instruct.h |
//   awk -F',' '/^  [A-Z]/ {
//     ++i;
//     print i " => Instruction::"
//     substr(toupper($1), 3,1) substr(tolower($1), 4) ","
//   }'
pub fn decode(i: i32) -> Instruction {
  match i {
    1 => Instruction::Constbyte,
    2 => Instruction::Constshort,
    3 => Instruction::Getglobal,
    4 => Instruction::Setglobal,
    5 => Instruction::Cur,
    6 => Instruction::Switch,
    7 => Instruction::Branch,
    8 => Instruction::Branchif,
    9 => Instruction::Branchifnot,
    10 => Instruction::Popbranchifnot,
    11 => Instruction::Branchifneqtag,
    12 => Instruction::Branchifeq,
    13 => Instruction::Branchifneq,
    14 => Instruction::Branchiflt,
    15 => Instruction::Branchifgt,
    16 => Instruction::Branchifle,
    17 => Instruction::Branchifge,
    18 => Instruction::Branchinterval,
    19 => Instruction::Ccall1,
    20 => Instruction::Ccall2,
    21 => Instruction::Ccall3,
    22 => Instruction::Ccall4,
    23 => Instruction::Ccall5,
    24 => Instruction::Ccalln,
    25 => Instruction::Makeblock,
    26 => Instruction::Makeblock1,
    27 => Instruction::Makeblock2,
    28 => Instruction::Makeblock3,
    29 => Instruction::Makeblock4,
    30 => Instruction::Tagof,
    31 => Instruction::Access,
    32 => Instruction::Acc0,
    33 => Instruction::Acc1,
    34 => Instruction::Acc2,
    35 => Instruction::Acc3,
    36 => Instruction::Acc4,
    37 => Instruction::Acc5,
    38 => Instruction::Atom,
    39 => Instruction::Atom0,
    40 => Instruction::Atom1,
    41 => Instruction::Atom2,
    42 => Instruction::Atom3,
    43 => Instruction::Atom4,
    44 => Instruction::Atom5,
    45 => Instruction::Atom6,
    46 => Instruction::Atom7,
    47 => Instruction::Atom8,
    48 => Instruction::Atom9,
    49 => Instruction::Getfield,
    50 => Instruction::Getfield0,
    51 => Instruction::Getfield1,
    52 => Instruction::Getfield2,
    53 => Instruction::Getfield3,
    54 => Instruction::Setfield,
    55 => Instruction::Setfield0,
    56 => Instruction::Setfield1,
    57 => Instruction::Setfield2,
    58 => Instruction::Setfield3,
    59 => Instruction::Stop,
    60 => Instruction::Checksignals,
    61 => Instruction::Apply,
    62 => Instruction::Return,
    63 => Instruction::Appterm,
    64 => Instruction::Grab,
    65 => Instruction::Let,
    66 => Instruction::Letrec1,
    67 => Instruction::Dummy,
    68 => Instruction::Update,
    69 => Instruction::Endlet,
    70 => Instruction::Endlet1,
    71 => Instruction::Pushtrap,
    72 => Instruction::Raise,
    73 => Instruction::Poptrap,
    74 => Instruction::Push,
    75 => Instruction::Pop,
    76 => Instruction::Pushmark,
    77 => Instruction::Pushgetglobalapply,
    78 => Instruction::Pushgetglobalappterm,
    79 => Instruction::Boolnot,
    80 => Instruction::Negint,
    81 => Instruction::Succint,
    82 => Instruction::Predint,
    83 => Instruction::Addint,
    84 => Instruction::Subint,
    85 => Instruction::Mulint,
    86 => Instruction::Divint,
    87 => Instruction::Modint,
    88 => Instruction::Andint,
    89 => Instruction::Orint,
    90 => Instruction::Xorint,
    91 => Instruction::Shiftleftint,
    92 => Instruction::Shiftrightintsigned,
    93 => Instruction::Shiftrightintunsigned,
    94 => Instruction::Eq,
    95 => Instruction::Neq,
    96 => Instruction::Ltint,
    97 => Instruction::Gtint,
    98 => Instruction::Leint,
    99 => Instruction::Geint,
    100 => Instruction::Incr,
    101 => Instruction::Decr,
    102 => Instruction::Floatop,
    103 => Instruction::Intoffloat,
    104 => Instruction::Eqfloat,
    105 => Instruction::Neqfloat,
    106 => Instruction::Ltfloat,
    107 => Instruction::Gtfloat,
    108 => Instruction::Lefloat,
    109 => Instruction::Gefloat,
    110 => Instruction::Stringlength,
    111 => Instruction::Getstringchar,
    112 => Instruction::Setstringchar,
    113 => Instruction::Eqstring,
    114 => Instruction::Neqstring,
    115 => Instruction::Ltstring,
    116 => Instruction::Gtstring,
    117 => Instruction::Lestring,
    118 => Instruction::Gestring,
    119 => Instruction::Makevector,
    120 => Instruction::Vectlength,
    121 => Instruction::Getvectitem,
    122 => Instruction::Setvectitem,
    123 => Instruction::Break,
    124 => Instruction::Floatofint,
    125 => Instruction::Negfloat,
    126 => Instruction::Addfloat,
    127 => Instruction::Subfloat,
    128 => Instruction::Mulfloat,
    129 => Instruction::Divfloat,
    130 => Instruction::Makestring,
    _ => panic!("not an instruction: {}", i),
  }
}
