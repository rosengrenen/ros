use core::alloc::Allocator;

use crate::aml::context::Context;
use crate::aml::name::NameString;
use crate::aml::ops::DataRegionOp;
use crate::aml::parser::fail;
use crate::aml::parser::Input;
use crate::aml::parser::ParseResult;
use crate::aml::term::TermArg;

pub struct DataRegion<A: Allocator> {
    pub name: NameString<A>,
    pub term1: TermArg<A>,
    pub term2: TermArg<A>,
    pub term3: TermArg<A>,
}

impl<A: Allocator + Clone> DataRegion<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = DataRegionOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (name, input) = NameString::parse(input, alloc.clone())?;
        let (term1, input) = TermArg::parse(input, context, alloc.clone())?;
        let (term2, input) = TermArg::parse(input, context, alloc.clone())?;
        let (term3, input) = TermArg::parse(input, context, alloc)?;
        let this = Self {
            name,
            term1,
            term2,
            term3,
        };
        Ok((this, input))
    }
}

impl<A: Allocator> core::fmt::Debug for DataRegion<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DataRegion")
            .field("name", &self.name)
            .field("term1", &self.term1)
            .field("term2", &self.term2)
            .field("term3", &self.term3)
            .finish()
    }
}
