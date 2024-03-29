#![cfg_attr(not(test), no_std)]
#![feature(allocator_api)]

pub mod aml;

#[cfg(test)]
mod tests {
    use crate::aml::{context::Context, parser::Input, term::TermObj};
    use std::alloc::Global;

    #[test]
    fn test_name() {
        let aml = include_bytes!("../dsdt.aml");
        const HEADER_LEN: usize = 36;
        let mut input = Input::new(&aml.as_slice()[..]);
        let mut context = Context::new(Global);
        loop {
            match TermObj::parse(input, &mut context, Global) {
                Ok((obj, i)) => {
                    println!("ok {:?}", i);
                    input = i
                }
                Err(_) => {
                    println!("err");
                    break;
                }
            }
        }
        assert!(false);
        // println!("{:?}", res);
        // assert!(res.is_ok());
    }
}
