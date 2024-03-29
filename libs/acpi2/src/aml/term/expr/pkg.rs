use core::alloc::Allocator;

use super::PkgElement;
use crate::aml::{
    context::Context,
    data::byte_data,
    ops::PkgOp,
    parser::{fail, fail_if_not_empty, Input, ParseResult},
    pkg_len::pkg,
};
use alloc::vec::Vec;

pub struct Pkg<A: Allocator> {
    len: usize,
    elements: Vec<PkgElement<A>, A>,
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
