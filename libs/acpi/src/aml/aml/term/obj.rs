use super::{named::NamedObj, namespace::NameSpaceModObj};
use crate::aml::Context;
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub enum Obj<A: Allocator> {
    NameSpaceModObj(NameSpaceModObj<A>),
    NamedObj(NamedObj<A>),
}

impl<A: Allocator + Clone> Obj<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            NameSpaceModObj::p.map(Self::NameSpaceModObj),
            NamedObj::p.map(Self::NamedObj),
        )
            .alt()
            .add_context("Obj")
            .parse(input, context, alloc)
    }
}
