use crate::aml::{
    aml::{
        name::{SuperName, Target},
        term::term_arg::TermArg,
    },
    ops::{AddOp, DecrementOp, DivideOp, IncrementOp, MultiplyOp, SubtractOp},
    prefixed, Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub enum Integer<A: Allocator> {
    Add(Add<A>),
    Multiply(Multiply<A>),
    Subtract(Subtract<A>),
    Divide(Divide<A>),
    Decrement(Decrement<A>),
    Increment(Increment<A>),
}

impl<A: Allocator + Clone> Integer<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            Add::p.map(Self::Add),
            Multiply::p.map(Self::Multiply),
            Subtract::p.map(Self::Subtract),
            Divide::p.map(Self::Divide),
            Decrement::p.map(Self::Decrement),
            Increment::p.map(Self::Increment),
        )
            .alt()
            .add_context("Integer")
            .parse(input, context, alloc)
    }
}

macro_rules! integer_binary_expr {
    ($name:ident, $op:ident) => {
        #[derive(Debug)]
        pub struct $name<A: Allocator> {
            pub left: TermArg<A>,
            pub right: TermArg<A>,
            pub target: Target<A>,
        }

        impl<A: Allocator + Clone> $name<A> {
            pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
                input: I,
                context: &mut Context,
                alloc: A,
            ) -> ParseResult<I, Self, E> {
                prefixed($op::p, (TermArg::p, TermArg::p, Target::p))
                    .map(|(left, right, target)| Self {
                        left,
                        right,
                        target,
                    })
                    .add_context(stringify!($name))
                    .parse(input, context, alloc)
            }
        }
    };
}

integer_binary_expr!(Add, AddOp);
integer_binary_expr!(Multiply, MultiplyOp);
integer_binary_expr!(Subtract, SubtractOp);

#[derive(Debug)]
pub struct Divide<A: Allocator> {
    pub dividend: TermArg<A>,
    pub divisor: TermArg<A>,
    pub remainder: Target<A>,
    pub quotient: Target<A>,
}

impl<A: Allocator + Clone> Divide<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let dividend = TermArg::p; // => Integer
        let divisor = TermArg::p; // => Integer
        prefixed(DivideOp::p, (dividend, divisor, Target::p, Target::p))
            .map(|(dividend, divisor, remainder, quotient)| Self {
                dividend,
                divisor,
                remainder,
                quotient,
            })
            .add_context("Divide")
            .parse(input, context, alloc)
    }
}

macro_rules! integer_unary_expr {
    ($name:ident, $op:ident) => {
        #[derive(Debug)]
        pub struct $name<A: Allocator> {
            pub name: SuperName<A>,
        }

        impl<A: Allocator + Clone> $name<A> {
            pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
                input: I,
                context: &mut Context,
                alloc: A,
            ) -> ParseResult<I, Self, E> {
                prefixed($op::p, SuperName::p)
                    .map(|name| Self { name })
                    .add_context(stringify!($name))
                    .parse(input, context, alloc)
            }
        }
    };
}

integer_unary_expr!(Decrement, DecrementOp);
integer_unary_expr!(Increment, IncrementOp);
