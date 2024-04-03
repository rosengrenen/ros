#![cfg_attr(not(test), no_std)]
#![feature(allocator_api)]

pub mod aml;

use alloc::vec::Vec;
use core::alloc::Allocator;

use aml::context::Context;
use aml::parser::fail_if_not_empty;
use aml::parser::Input;
use aml::parser::ParseResult;
use aml::term::TermObj;

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
    use std::alloc::Global;

    use crate::aml::context::Context;
    use crate::aml::parser::Input;
    use crate::parse_term_objs;

    #[test]
    fn parse_example_aml() {
        let aml = include_bytes!("../dsdt.aml");
        const HEADER_LEN: usize = 36;
        let input = Input::new(&aml.as_slice()[HEADER_LEN..]);
        let mut context = Context::new(Global);
        let res = parse_term_objs(input, &mut context, Global);
        assert!(res.is_ok());
    }
}
