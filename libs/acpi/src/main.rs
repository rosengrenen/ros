#![feature(allocator_api)]
#![allow(dead_code)]
#![allow(unused_attributes)]
#![allow(unused_variables)]

use acpi::aml::{
    aml::term::term_obj::TermObj, Context, LocatedInput, SimpleError, SimpleErrorKind,
};
use std::alloc::Global;

fn main() {
    let aml = include_bytes!("../dsdt.aml");
    let aml = LocatedInput::new(&aml[36..]);
    let mut context = Context {};
    let res = TermObj::p::<_, SimpleError<LocatedInput<&[u8]>, Global>>(aml, &mut context, Global);
    match res {
        Ok(ast) => println!("{:#?}", ast),
        Err(e) => match e {
            parser::error::ParserError::Error(e) => todo!(),
            parser::error::ParserError::Failure(e) => {
                for e in &e.errors {
                    match e.1 {
                        SimpleErrorKind::Context(_) => {
                            println!(
                                "{:x?} {:?} {:?}",
                                &e.0.inner[0..16usize.min(e.0.inner.len())],
                                e.0.span,
                                e.1
                            )
                        }
                        _ => (),
                    }
                }
            }
        },
    }
}
