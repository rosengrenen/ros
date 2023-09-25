use super::TermArg;
use crate::aml::{
    ops::{
        LAndOp, LEqualOp, LGreaterEqualOp, LGreaterOp, LLessEqualOp, LLessOp, LNotEqualOp, LNotOp,
        LOrOp,
    },
    prefixed, Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

pub enum Logical<A: Allocator> {
    And(And<A>),
    Equal(Equal<A>),
    GreaterEqual(GreaterEqual<A>),
    Greater(Greater<A>),
    LessEqual(LessEqual<A>),
    Less(Less<A>),
    NotEqual(NotEqual<A>),
    Or(Or<A>),
}

impl<A: Allocator + Clone> Logical<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            And::p.map(Self::And),
            Equal::p.map(Self::Equal),
            GreaterEqual::p.map(Self::GreaterEqual),
            Greater::p.map(Self::Greater),
            LessEqual::p.map(Self::LessEqual),
            Less::p.map(Self::Less),
            NotEqual::p.map(Self::NotEqual),
            Or::p.map(Self::Or),
        )
            .alt()
            .add_context("Logical")
            .parse(input, context, alloc)
    }
}

macro_rules! logical_binary_expr {
    ($name:ident, $op:ident) => {
        pub struct $name<A: Allocator> {
            pub left: TermArg<A>,
            pub right: TermArg<A>,
        }

        impl<A: Allocator + Clone> $name<A> {
            pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
                input: I,
                context: &mut Context,
                alloc: A,
            ) -> ParseResult<I, Self, E> {
                prefixed($op::p, (TermArg::p, TermArg::p))
                    .map(|(left, right)| Self { left, right })
                    .add_context(stringify!($name))
                    .parse(input, context, alloc)
            }
        }
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

pub struct Not<A: Allocator> {
    pub operand: TermArg<A>,
}

impl<A: Allocator + Clone> Not<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(LNotOp::p, TermArg::p)
            .map(|operand| Self { operand })
            .add_context("LNot")
            .parse(input, context, alloc)
    }
}
