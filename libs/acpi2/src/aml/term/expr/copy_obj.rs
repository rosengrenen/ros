use core::alloc::Allocator;

use crate::aml::context::Context;
use crate::aml::name::SimpleName;
use crate::aml::ops::CopyObjOp;
use crate::aml::parser::fail;
use crate::aml::parser::Input;
use crate::aml::parser::ParseResult;
use crate::aml::term::TermArg;

pub struct CopyObj<A: Allocator> {
    pub arg: TermArg<A>,
    pub name: SimpleName<A>,
}

impl<A: Allocator + Clone> CopyObj<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = CopyObjOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (arg, input) = TermArg::parse(input, context, alloc.clone())?;
        let (name, input) = SimpleName::parse(input, alloc)?;
        Ok((Self { arg, name }, input))
    }
}

impl<A: Allocator> core::fmt::Debug for CopyObj<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CopyObj")
            .field("arg", &self.arg)
            .field("name", &self.name)
            .finish()
    }
}
