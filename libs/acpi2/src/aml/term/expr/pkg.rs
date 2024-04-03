use alloc::vec::Vec;
use core::alloc::Allocator;

use super::PkgElement;
use crate::aml::context::Context;
use crate::aml::data::byte_data;
use crate::aml::ops::PkgOp;
use crate::aml::parser::fail;
use crate::aml::parser::fail_if_not_empty;
use crate::aml::parser::Input;
use crate::aml::parser::ParseResult;
use crate::aml::pkg_len::pkg;

pub struct Pkg<A: Allocator> {
    pub len: usize,
    pub elements: Vec<PkgElement<A>, A>,
}

impl<A: Allocator + Clone> Pkg<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = PkgOp::parse(input)?;
        let (pkg_input, input) = pkg(input)?;
        let (pkg, pkg_input) = fail(Self::parse_inner(pkg_input, context, alloc))?;
        fail_if_not_empty(pkg_input)?;
        Ok((pkg, input))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (len, mut input) = byte_data(input)?;
        let mut elements = Vec::new(alloc.clone());
        while let Ok((element, i)) = PkgElement::parse(input, context, alloc.clone()) {
            elements.push(element).unwrap();
            input = i;
        }

        let this = Self {
            len: len as usize,
            elements,
        };
        Ok((this, input))
    }
}

impl<A: Allocator> core::fmt::Debug for Pkg<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Pkg")
            .field("len", &self.len)
            .field("elements", &self.elements)
            .finish()
    }
}
