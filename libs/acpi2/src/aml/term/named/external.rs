use core::alloc::Allocator;

use crate::aml::data::byte_data;
use crate::aml::name::NameString;
use crate::aml::ops::ExternalOp;
use crate::aml::parser::fail;
use crate::aml::parser::Input;
use crate::aml::parser::ParseResult;

pub struct External<A: Allocator> {
    pub name: NameString<A>,
    pub obj_type: u8,
    pub argument_count: u8,
}

impl<A: Allocator + Clone> External<A> {
    pub fn parse<'a>(input: Input<'a>, alloc: A) -> ParseResult<'a, Self> {
        let (_, input) = ExternalOp::parse(input)?;
        fail(Self::parse_inner(input, alloc))
    }

    fn parse_inner<'a>(input: Input<'a>, alloc: A) -> ParseResult<'a, Self> {
        let (name, input) = NameString::parse(input, alloc)?;
        let (obj_type, input) = byte_data(input)?;
        let (argument_count, input) = byte_data(input)?;
        let this = Self {
            name,
            obj_type,
            argument_count,
        };
        Ok((this, input))
    }
}

impl<A: Allocator> core::fmt::Debug for External<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("External")
            .field("name", &self.name)
            .field("obj_type", &self.obj_type)
            .field("argument_count", &self.argument_count)
            .finish()
    }
}
