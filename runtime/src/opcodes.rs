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
}

pub fn decode(i: u8) -> Instruction {
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
        _ => panic!("not an instruction: {}", i),
    }
}
