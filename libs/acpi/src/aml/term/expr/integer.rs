use crate::aml::{
    name::{SuperName, Target},
    ops::{AddOp, DecrementOp, DivideOp, IncrementOp, MultiplyOp, SubtractOp},
    prefixed::prefixed,
    term::TermArg,
};

parser_enum_alloc!(
    enum Integer {
        Add(Add<A>),
        Multiply(Multiply<A>),
        Subtract(Subtract<A>),
        Divide(Divide<A>),
        Decrement(Decrement<A>),
        Increment(Increment<A>),
    }
);

macro_rules! integer_binary_expr {
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

integer_binary_expr!(Add, AddOp);
integer_binary_expr!(Multiply, MultiplyOp);
integer_binary_expr!(Subtract, SubtractOp);

parser_struct_alloc!(
    struct Divide {
        dividend: TermArg<A>,
        divisor: TermArg<A>,
        remainder: Target<A>,
        quotient: Target<A>,
    },
    prefixed(DivideOp::p, (TermArg::p, TermArg::p, Target::p, Target::p))
);

macro_rules! integer_unary_expr {
    ($name:ident, $op:ident) => {
        parser_struct_alloc!(
            struct $name {
                name: SuperName<A>,
            },
            prefixed($op::p, SuperName::p)
        );
    };
}

integer_unary_expr!(Decrement, DecrementOp);
integer_unary_expr!(Increment, IncrementOp);
