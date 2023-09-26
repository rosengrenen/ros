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
        CreateBitField(CreateBitField<A>),
        CreateByteField(CreateByteField<A>),
        CreateWordField(CreateWordField<A>),
        CreateDWordField(CreateDWordField<A>),
        CreateQWordField(CreateQWordField<A>),
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

create_const_field!(CreateBitField, CreateBitFieldOp);
create_const_field!(CreateByteField, CreateByteFieldOp);
create_const_field!(CreateWordField, CreateWordFieldOp);
create_const_field!(CreateDWordField, CreateDWordFieldOp);
create_const_field!(CreateQWordField, CreateQWordFieldOp);

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
