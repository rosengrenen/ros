use alloc::vec::Vec;
use core::alloc::Allocator;

use crate::aml::context::Context;
use crate::aml::name::NameString;
use crate::aml::ops::DeviceOp;
use crate::aml::parser::fail;
use crate::aml::parser::fail_if_not_empty;
use crate::aml::parser::Input;
use crate::aml::parser::ParseResult;
use crate::aml::pkg_len::pkg;
use crate::aml::term::TermObj;

pub struct Device<A: Allocator> {
    pub name: NameString<A>,
    pub terms: Vec<TermObj<A>, A>,
}

impl<A: Allocator + Clone> Device<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = DeviceOp::parse(input)?;
        let (pkg_input, input) = pkg(input)?;
        let (device, pkg_input) = fail(Self::parse_inner(pkg_input, context, alloc))?;
        fail_if_not_empty(pkg_input)?;
        Ok((device, input))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (name, mut input) = NameString::parse(input, alloc.clone())?;
        context.push_scope(&name);
        let mut terms = Vec::new(alloc.clone());
        while let Ok((term, i)) = TermObj::parse(input, context, alloc.clone()) {
            terms.push(term).unwrap();
            input = i;
        }

        context.pop_scope();
        Ok((Self { name, terms }, input))
    }
}

impl<A: Allocator> core::fmt::Debug for Device<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Device")
            .field("name", &self.name)
            .field("terms", &self.terms)
            .finish()
    }
}
