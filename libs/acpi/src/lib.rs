#![cfg_attr(not(test), no_std)]
#![feature(allocator_api)]

pub mod aml;
pub mod tables;

#[cfg(test)]
mod tests {
    use crate::aml::{Context, LocatedInput, SimpleError, TermObj};
    use parser::{multi::many::many, parser::Parser, primitive::fail::fail};
    use std::alloc::Global;

    #[test]
    fn test_name() {
        let aml = include_bytes!("../dsdt.aml");
        let aml = LocatedInput::new(&aml.as_slice()[36..]);
        let mut context = Context::new(Global);
        let res = (
            many(TermObj::p::<LocatedInput<&[u8]>, SimpleError<LocatedInput<&[u8]>, Global>>),
            fail(),
        )
            .parse(aml, &mut context, Global);
        println!("{:?}", res);
        assert!(res.is_ok());
    }
}
