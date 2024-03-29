use core::alloc::Allocator;

use super::PkgElement;
use crate::aml::{
    context::Context,
    ops::VarPkgOp,
    parser::{fail, fail_if_not_empty, Input, ParseResult},
    pkg_len::pkg,
    term::TermArg,
};
use alloc::vec::Vec;

pub struct VarPkg<A: Allocator> {
    len: TermArg<A>,
    elements: Vec<PkgElement<A>, A>,
}

impl<A: Allocator + Clone> VarPkg<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = VarPkgOp::parse(input)?;
        let (pkg_input, input) = pkg(input)?;
        let (var_pkg, pkg_input) = fail(Self::parse_inner(pkg_input, context, alloc))?;
        fail_if_not_empty(pkg_input)?;
        Ok((var_pkg, input))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (len, mut input) = TermArg::parse(input, context, alloc.clone())?;
        let mut elements = Vec::new(alloc.clone());
        while let Ok((element, i)) = PkgElement::parse(input, context, alloc.clone()) {
            elements.push(element).unwrap();
            input = i;
        }

        Ok((Self { len, elements }, input))
    }
}
