use core::alloc::Allocator;

use crate::aml::context::Context;
use crate::aml::name::SuperName;
use crate::aml::ops::StoreOp;
use crate::aml::parser::fail;
use crate::aml::parser::Input;
use crate::aml::parser::ParseResult;
use crate::aml::term::TermArg;

pub struct Store<A: Allocator> {
    pub term: TermArg<A>,
    pub name: SuperName<A>,
}

impl<A: Allocator + Clone> Store<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = StoreOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (term, input) = TermArg::parse(input, context, alloc.clone())?;
        let (name, input) = SuperName::parse(input, context, alloc)?;
        Ok((Self { term, name }, input))
    }
}

impl<A: Allocator> core::fmt::Debug for Store<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Store")
            .field("term", &self.term)
            .field("name", &self.name)
            .finish()
    }
}
