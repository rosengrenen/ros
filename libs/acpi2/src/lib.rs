#![cfg_attr(not(test), no_std)]
#![feature(allocator_api)]

use core::alloc::Allocator;

use alloc::vec::Vec;
use aml::{
    context::Context,
    parser::{fail_if_not_empty, Input, ParseResult},
    term::TermObj,
};

pub mod aml;

pub fn parse_term_objs<'a, A: Allocator + Clone>(
    mut input: Input<'a>,
    context: &mut Context<A>,
    alloc: A,
) -> ParseResult<'a, Vec<TermObj<A>, A>> {
    let mut term_objs = Vec::new(alloc.clone());
    while let Ok((term_obj, i)) = TermObj::parse(input, context, alloc.clone()) {
        term_objs.push(term_obj).unwrap();
        input = i;
    }

    fail_if_not_empty(input)?;
    Ok((term_objs, input))
}

#[cfg(test)]
mod tests {
    use crate::{
        aml::{context::Context, parser::Input},
        parse_term_objs,
    };
    use std::alloc::Global;

    #[test]
    fn test_name() {
        let aml = include_bytes!("../dsdt.aml");
        const HEADER_LEN: usize = 36;
        let input = Input::new(&aml.as_slice()[HEADER_LEN..]);
        let mut context = Context::new(Global);
        let res = parse_term_objs(input, &mut context, Global);
        assert!(res.is_ok());
    }
}
