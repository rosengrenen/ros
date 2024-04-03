use alloc::vec::Vec;
use core::alloc::Allocator;

use super::PkgElement;
use crate::aml::context::Context;
use crate::aml::ops::VarPkgOp;
use crate::aml::parser::fail;
use crate::aml::parser::fail_if_not_empty;
use crate::aml::parser::Input;
use crate::aml::parser::ParseResult;
use crate::aml::pkg_len::pkg;
use crate::aml::term::TermArg;

pub struct VarPkg<A: Allocator> {
    pub len: TermArg<A>,
    pub elements: Vec<PkgElement<A>, A>,
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

impl<A: Allocator> core::fmt::Debug for VarPkg<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("VarPkg")
            .field("len", &self.len)
            .field("elements", &self.elements)
            .finish()
    }
}
