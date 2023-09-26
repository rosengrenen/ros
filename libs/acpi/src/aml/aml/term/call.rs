use super::term_arg::TermArg;
use crate::aml::{aml::name::NameString, Context};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
    primitive::fail::fail,
};

#[derive(Debug)]
pub struct MethodInvocation<A: Allocator> {
    pub name: NameString<A>,
    pub args: Vec<TermArg<A>, A>,
}

impl<A: Allocator + Clone> MethodInvocation<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        fail()
            .add_context("MethodInvocation")
            .parse(input, context, alloc)?;
        panic!();
        // (NameString::p, many(TermArg::p))
        //     .map(|(name, args)| Self { name, args })
        //     .add_context("MethodInvocation")
        //     .parse(input, context, alloc)
    }
}
