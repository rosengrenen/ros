use super::PkgElement;
use crate::aml::{aml::term::TermArg, ops::VarPkgOp, pkg, prefixed, Context};
use alloc::vec::Vec;
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    multi::many::many,
    parser::Parser,
};

pub struct VarPkg<A: Allocator> {
    pub len: TermArg<A>,
    pub elements: Vec<PkgElement<A>, A>,
}

impl<A: Allocator + Clone> VarPkg<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let var_num_elements = TermArg::p; // => Integer
        prefixed(VarPkgOp::p, pkg((var_num_elements, many(PkgElement::p))))
            .map(|(len, elements)| Self { len, elements })
            .add_context("VarPkg")
            .parse(input, context, alloc)
    }
}
