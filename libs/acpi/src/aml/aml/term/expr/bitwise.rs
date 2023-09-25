use crate::aml::{
    aml::{name::Target, term::TermArg},
    ops::{AndOp, NAndOp, NOrOp, NotOp, OrOp, ShiftLeftOp, ShiftRightOp, XOrOp},
    prefixed, Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

pub enum Bitwise<A: Allocator> {
    And(And<A>),
    NAnd(NAnd<A>),
    NOr(NOr<A>),
    Or(Or<A>),
    XOr(XOr<A>),
    Not(Not<A>),
    ShiftLeft(ShiftLeft<A>),
    ShiftRight(ShiftRight<A>),
}

impl<A: Allocator + Clone> Bitwise<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            And::p.map(Self::And),
            NAnd::p.map(Self::NAnd),
            NOr::p.map(Self::NOr),
            Or::p.map(Self::Or),
            XOr::p.map(Self::XOr),
            Not::p.map(Self::Not),
            ShiftLeft::p.map(Self::ShiftLeft),
            ShiftRight::p.map(Self::ShiftRight),
        )
            .alt()
            .add_context("Bitwise")
            .parse(input, context, alloc)
    }
}

macro_rules! bitwise_binary_expr {
    ($name:ident, $op:ident) => {
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

bitwise_binary_expr!(And, AndOp);
bitwise_binary_expr!(NAnd, NAndOp);
bitwise_binary_expr!(NOr, NOrOp);
bitwise_binary_expr!(Or, OrOp);
bitwise_binary_expr!(XOr, XOrOp);

pub struct Not<A: Allocator> {
    pub operand: TermArg<A>,
    pub target: Target<A>,
}

impl<A: Allocator + Clone> Not<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(NotOp::p, (TermArg::p, Target::p))
            .map(|(operand, target)| Self { operand, target })
            .add_context("Not")
            .parse(input, context, alloc)
    }
}

macro_rules! bitwise_shift_expr {
    ($name:ident, $op:ident) => {
        pub struct $name<A: Allocator> {
            pub operand: TermArg<A>,
            pub shift_count: TermArg<A>,
            pub target: Target<A>,
        }

        impl<A: Allocator + Clone> $name<A> {
            pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
                input: I,
                context: &mut Context,
                alloc: A,
            ) -> ParseResult<I, Self, E> {
                let shift_count = TermArg::p; // => Integer
                prefixed($op::p, (TermArg::p, shift_count, Target::p))
                    .map(|(operand, shift_count, target)| Self {
                        operand,
                        shift_count,
                        target,
                    })
                    .add_context(stringify!($name))
                    .parse(input, context, alloc)
            }
        }
    };
}

bitwise_shift_expr!(ShiftLeft, ShiftLeftOp);
bitwise_shift_expr!(ShiftRight, ShiftRightOp);
