use crate::aml::{
    name::NameString,
    ops::{
        CreateBitFieldOp, CreateByteFieldOp, CreateDWordFieldOp, CreateFieldOp, CreateQWordFieldOp,
        CreateWordFieldOp,
    },
    prefixed::prefixed,
    term::TermArg,
};

parser_enum_alloc!(
    enum CreateConstField {
        Bit(Bit<A>),
        Byte(Byte<A>),
        Word(Word<A>),
        DWord(DWord<A>),
        QWord(QWord<A>),
    }
);

macro_rules! create_const_field {
    ($name:ident, $op:ident) => {
        parser_struct_alloc!(
            struct $name {
                source: TermArg<A>,
                index: TermArg<A>,
                name: NameString<A>,
            },
            prefixed($op::p, (TermArg::p, TermArg::p, NameString::p))
        );
    };
}

create_const_field!(Bit, CreateBitFieldOp);
create_const_field!(Byte, CreateByteFieldOp);
create_const_field!(Word, CreateWordFieldOp);
create_const_field!(DWord, CreateDWordFieldOp);
create_const_field!(QWord, CreateQWordFieldOp);

parser_struct_alloc!(
    struct CreateField {
        source_buf: TermArg<A>,
        bit_index: TermArg<A>,
        num_bits: TermArg<A>,
        name: NameString<A>,
    },
    prefixed(
        CreateFieldOp::p,
        (TermArg::p, TermArg::p, TermArg::p, NameString::p),
    )
);
