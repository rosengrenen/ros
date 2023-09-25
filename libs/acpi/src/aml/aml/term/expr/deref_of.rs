use crate::aml::{aml::term::TermArg, ops::DerefOfOp, prefixed, Context};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

pub struct DerefOf<A: Allocator> {
    pub obj_ref: TermArg<A>,
}

impl<A: Allocator + Clone> DerefOf<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let obj_ref = TermArg::p; // => ObjRef | String
        prefixed(DerefOfOp::p, obj_ref)
            .map(|obj_ref| Self { obj_ref })
            .add_context("DerefOf")
            .parse(input, context, alloc)
    }
}
