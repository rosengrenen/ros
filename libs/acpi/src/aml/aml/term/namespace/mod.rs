pub mod alias;
pub mod name;
pub mod scope;

use self::{alias::Alias, name::Name, scope::Scope};
use crate::aml::Context;
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub enum NameSpaceModObj<A: Allocator> {
    Alias(Alias<A>),
    Name(Name<A>),
    Scope(Scope<A>),
}

impl<A: Allocator + Clone> NameSpaceModObj<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            Alias::p.map(Self::Alias),
            Name::p.map(Self::Name),
            Scope::p.map(Self::Scope),
        )
            .alt()
            .add_context("NameSpaceModObj")
            .parse(input, context, alloc)
    }
}
