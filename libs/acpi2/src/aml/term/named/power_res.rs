use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    data::{byte_data, word_data},
    name::NameString,
    ops::PowerResOp,
    parser::{fail, fail_if_not_empty, Input, ParseResult},
    pkg_len::pkg,
    term::TermObj,
};
use alloc::vec::Vec;

pub struct PowerRes<A: Allocator> {
    pub name: NameString<A>,
    pub system_level: u8,
    pub resource_order: u16,
    pub terms: Vec<TermObj<A>, A>,
}

impl<A: Allocator + Clone> PowerRes<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = PowerResOp::parse(input)?;
        let (pkg_input, input) = pkg(input)?;
        let (power_res, pkg_input) = fail(Self::parse_inner(pkg_input, context, alloc))?;
        fail_if_not_empty(pkg_input)?;
        Ok((power_res, input))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (name, input) = NameString::parse(input, alloc.clone())?;
        let (system_level, input) = byte_data(input)?;
        let (resource_order, mut input) = word_data(input)?;
        context.push_scope(&name);
        let mut terms = Vec::new(alloc.clone());
        while let Ok((term, i)) = TermObj::parse(input, context, alloc.clone()) {
            terms.push(term).unwrap();
            input = i;
        }

        context.pop_scope();
        let this = Self {
            name,
            system_level,
            resource_order,
            terms,
        };
        Ok((this, input))
    }
}

impl<A: Allocator> core::fmt::Debug for PowerRes<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PowerRes")
            .field("name", &self.name)
            .field("system_level", &self.system_level)
            .field("resource_order", &self.resource_order)
            .field("terms", &self.terms)
            .finish()
    }
}
