use super::{
    ops::{
        BytePrefix, DWordPrefix, OneOp, OnesOp, QWordPrefix, RevisionOp, StringPrefix, WordPrefix,
        ZeroOp,
    },
    prefixed::prefixed,
    term::expr::{buffer::Buffer, pkg::Pkg, var_pkg::VarPkg},
};
use alloc::vec::Vec;
use parser::{
    multi::many::many,
    primitive::item::{item, satisfy, take_one},
};

parser_enum_alloc!(
    enum ComputationalData {
        ConstInteger(ConstInteger),
        String(String<A>),
        ConstObj(ConstObj),
        RevisionOp(RevisionOp),
        Buffer(Buffer<A>),
    }
);

parser_enum_alloc!(
    enum DataObj {
        ComputationalData(ComputationalData<A>),
        Pkg(Pkg<A>),
        VarPkg(VarPkg<A>),
    }
);

parser_enum_alloc!(
    enum DataRefObj {
        DataObj(DataObj<A>),
        // ObjRef(ObjRef),
    }
);

parser_enum!(
    enum ConstInteger {
        ByteConst(ByteConst),
        WordConst(WordConst),
        DWordConst(DWordConst),
        QWordConst(QWordConst),
    }
);

parser_struct_wrapper!(struct ByteConst(u8);, prefixed(BytePrefix::p, byte_data));
parser_struct_wrapper!(struct WordConst(u16);, prefixed(WordPrefix::p, word_data));
parser_struct_wrapper!(
    struct DWordConst(u32);,
    prefixed(DWordPrefix::p, dword_data)
);
parser_struct_wrapper!(
    struct QWordConst(u64);,
    prefixed(QWordPrefix::p, qword_data)
);

parser_fn!(
    fn byte_data() -> u8 {
        take_one()
    }
);
parser_fn!(
    fn word_data() -> u16 {
        (byte_data, byte_data).map(|(lower, higher)| ((higher as u16) << 8) | lower as u16)
    }
);
parser_fn!(
    fn dword_data() -> u32 {
        (word_data, word_data).map(|(lower, higher)| ((higher as u32) << 16) | lower as u32)
    }
);
parser_fn!(
    fn qword_data() -> u64 {
        (dword_data, dword_data).map(|(lower, higher)| ((higher as u64) << 32) | lower as u64)
    }
);

parser_struct_wrapper_alloc!(
    struct String(Vec<u8, A>);,
    prefixed(StringPrefix::p, (many(ascii_char), null_char)).map(|(bytes, _)| bytes)
);

parser_enum!(
    enum ConstObj {
        ZeroOp(ZeroOp),
        OneOp(OneOp),
        OnesOp(OnesOp),
    }
);

parser_fn!(
    fn ascii_char() -> u8 {
        satisfy(|b: &u8| (0x01..=0x7f).contains(b))
    }
);

parser_fn!(
    fn null_char() -> () {
        item(0x00).map(|_| ())
    }
);
