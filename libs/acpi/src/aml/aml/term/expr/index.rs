use crate::aml::{
    aml::{name::Target, term::TermArg},
    ops::IndexOp,
    prefixed, Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub struct Index<A: Allocator> {
    pub buf_pkg_str_obj: TermArg<A>,
    pub index_value: TermArg<A>,
    pub target: Target<A>,
}

impl<A: Allocator + Clone> Index<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let buf_pkg_str_obj = TermArg::p; // => Buffer | Pkg | String
        let index_value = TermArg::p; // => Integer
        prefixed(IndexOp::p, (buf_pkg_str_obj, index_value, Target::p))
            .map(|(buf_pkg_str_obj, index_value, target)| Self {
                buf_pkg_str_obj,
                index_value,
                target,
            })
            .add_context("Index")
            .parse(input, context, alloc)
    }
}
