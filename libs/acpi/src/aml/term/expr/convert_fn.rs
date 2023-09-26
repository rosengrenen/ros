use crate::aml::{
    name::Target,
    ops::{
        FromBcdOp, ToBcdOp, ToBufferOp, ToDecimalStringOp, ToHexStringOp, ToIntegerOp, ToStringOp,
    },
    prefixed::prefixed,
    term::TermArg,
};

parser_enum_alloc!(
    enum ConvertFn {
        FromBcd(FromBcd<A>),
        ToBcd(ToBcd<A>),
        ToBuffer(ToBuffer<A>),
        ToDecimalString(ToDecimalString<A>),
        ToHexString(ToHexString<A>),
        ToInteger(ToInteger<A>),
        ToString(ToString<A>),
    }
);

macro_rules! convert_fn_expr {
    ($name:ident, $op:ident) => {
        parser_struct_alloc!(
            struct $name {
                input: TermArg<A>,
                target: Target<A>,
            },
            prefixed($op::p, (TermArg::p, Target::p))
        );
    };
}

convert_fn_expr!(FromBcd, FromBcdOp);
convert_fn_expr!(ToBcd, ToBcdOp);
convert_fn_expr!(ToBuffer, ToBufferOp);
convert_fn_expr!(ToDecimalString, ToDecimalStringOp);
convert_fn_expr!(ToHexString, ToHexStringOp);
convert_fn_expr!(ToInteger, ToIntegerOp);

parser_struct_alloc!(
    struct ToString {
        arg1: TermArg<A>,
        length_arg: TermArg<A>,
        target: Target<A>,
    },
    prefixed(ToStringOp::p, (TermArg::p, TermArg::p, Target::p))
);
