use core::alloc::Allocator;

use crate::aml::name::NameString;
use crate::aml::ops::EventOp;
use crate::aml::parser::fail;
use crate::aml::parser::Input;
use crate::aml::parser::ParseResult;

pub struct Event<A: Allocator> {
    pub name: NameString<A>,
}

impl<A: Allocator + Clone> Event<A> {
    pub fn parse<'a>(input: Input<'a>, alloc: A) -> ParseResult<'a, Self> {
        let (_, input) = EventOp::parse(input)?;
        fail(Self::parse_inner(input, alloc))
    }

    fn parse_inner<'a>(input: Input<'a>, alloc: A) -> ParseResult<'a, Self> {
        let (name, input) = NameString::parse(input, alloc)?;
        Ok((Self { name }, input))
    }
}

impl<A: Allocator> core::fmt::Debug for Event<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Event").field("name", &self.name).finish()
    }
}
