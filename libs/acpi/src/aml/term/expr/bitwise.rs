use crate::aml::{
    name::Target,
    ops::{AndOp, NAndOp, NOrOp, NotOp, OrOp, ShiftLeftOp, ShiftRightOp, XOrOp},
    prefixed::prefixed,
    term::TermArg,
};

parser_enum_alloc!(
    enum Bitwise {
        And(And<A>),
        NAnd(NAnd<A>),
        NOr(NOr<A>),
        Or(Or<A>),
        XOr(XOr<A>),
        Not(Not<A>),
        ShiftLeft(ShiftLeft<A>),
        ShiftRight(ShiftRight<A>),
    }
);

macro_rules! bitwise_binary_expr {
    ($name:ident, $op:ident) => {
        parser_struct_alloc!(
            struct $name {
                left: TermArg<A>,
                right: TermArg<A>,
                target: Target<A>,
            },
            prefixed($op::p, (TermArg::p, TermArg::p, Target::p))
        );
    };
}

bitwise_binary_expr!(And, AndOp);
bitwise_binary_expr!(NAnd, NAndOp);
bitwise_binary_expr!(NOr, NOrOp);
bitwise_binary_expr!(Or, OrOp);
bitwise_binary_expr!(XOr, XOrOp);

parser_struct_alloc!(
    struct Not {
        operand: TermArg<A>,
        target: Target<A>,
    },
    prefixed(NotOp::p, (TermArg::p, Target::p))
);

macro_rules! bitwise_shift_expr {
    ($name:ident, $op:ident) => {
        parser_struct_alloc!(
            struct $name {
                operand: TermArg<A>,
                shift_count: TermArg<A>,
                target: Target<A>,
            },
            prefixed($op::p, (TermArg::p, TermArg::p, Target::p))
        );
    };
}

bitwise_shift_expr!(ShiftLeft, ShiftLeftOp);
bitwise_shift_expr!(ShiftRight, ShiftRightOp);
