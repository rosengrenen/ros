use alloc::vec::Vec;
use core::alloc::Allocator;

use crate::aml::context::Context;
use crate::aml::data::byte_data;
use crate::aml::name::NameString;
use crate::aml::ops::MethodOp;
use crate::aml::parser::fail;
use crate::aml::parser::fail_if_not_empty;
use crate::aml::parser::Input;
use crate::aml::parser::ParseResult;
use crate::aml::pkg_len::pkg;
use crate::aml::term::TermObj;

pub struct Method<A: Allocator> {
    pub name: NameString<A>,
    pub flags: u8,
    pub terms: Vec<TermObj<A>, A>,
}

impl<A: Allocator + Clone> Method<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = MethodOp::parse(input)?;
        let (pkg_input, input) = pkg(input)?;
        let (method, pkg_input) = fail(Self::parse_inner(pkg_input, context, alloc))?;
        fail_if_not_empty(pkg_input)?;
        Ok((method, input))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (name, input) = NameString::parse(input, alloc.clone())?;
        let (flags, mut input) = byte_data(input)?;
        context.add_method(&name, flags as usize & 0x7);
        context.push_scope(&name);
        let mut terms = Vec::new(alloc.clone());
        while let Ok((term, i)) = TermObj::parse(input, context, alloc.clone()) {
            terms.push(term).unwrap();
            input = i;
        }

        context.pop_scope();
        Ok((Self { name, flags, terms }, input))
    }
}

impl<A: Allocator> core::fmt::Debug for Method<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Method")
            .field("name", &self.name)
            .field("flags", &self.flags)
            .field("terms", &self.terms)
            .finish()
    }
}
