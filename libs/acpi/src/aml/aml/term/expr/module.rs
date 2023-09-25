use crate::aml::{
    aml::{name::Target, term::TermArg},
    ops::ModOp,
    prefixed, Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

pub struct Mod<A: Allocator> {
    pub dividend: TermArg<A>,
    pub divisor: TermArg<A>,
    pub target: Target<A>,
}

impl<A: Allocator + Clone> Mod<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let dividend = TermArg::p; // => Integer
        let divisor = TermArg::p; // => Integer
        prefixed(ModOp::p, (dividend, divisor, Target::p))
            .map(|(dividend, divisor, target)| Self {
                dividend,
                divisor,
                target,
            })
            .add_context("Mod")
            .parse(input, context, alloc)
    }
}
