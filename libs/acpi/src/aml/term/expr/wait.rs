use crate::aml::{
    ops::WaitOp,
    prefixed::prefixed,
    term::{statement::EventObj, TermArg},
};

parser_struct_alloc!(
    struct Wait {
        event: EventObj<A>,
        operand: TermArg<A>,
    },
    prefixed(WaitOp::p, (EventObj::p, TermArg::p))
);
