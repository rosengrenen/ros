use crate::aml::{
    ops::{
        LAndOp, LEqualOp, LGreaterEqualOp, LGreaterOp, LLessEqualOp, LLessOp, LNotEqualOp, LNotOp,
        LOrOp,
    },
    prefixed::prefixed,
    term::TermArg,
};

parser_enum_alloc!(
    enum Logical {
        And(And<A>),
        Equal(Equal<A>),
        GreaterEqual(GreaterEqual<A>),
        Greater(Greater<A>),
        LessEqual(LessEqual<A>),
        Less(Less<A>),
        NotEqual(NotEqual<A>),
        Or(Or<A>),
    }
);

macro_rules! logical_binary_expr {
    ($name:ident, $op:ident) => {
        parser_struct_alloc!(
            struct $name {
                left: TermArg<A>,
                right: TermArg<A>,
            },
            prefixed($op::p, (TermArg::p, TermArg::p))
        );
    };
}

logical_binary_expr!(And, LAndOp);
logical_binary_expr!(Equal, LEqualOp);
logical_binary_expr!(GreaterEqual, LGreaterEqualOp);
logical_binary_expr!(Greater, LGreaterOp);
logical_binary_expr!(LessEqual, LLessEqualOp);
logical_binary_expr!(Less, LLessOp);
logical_binary_expr!(NotEqual, LNotEqualOp);
logical_binary_expr!(Or, LOrOp);

parser_struct_alloc!(
    struct Not {
        operand: TermArg<A>,
    },
    prefixed(LNotOp::p, TermArg::p)
);
