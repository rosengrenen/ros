#![feature(allocator_api)]
#![allow(dead_code)]
#![allow(unused_attributes)]
#![allow(unused_variables)]

use std::alloc::Global;

use acpi::aml::{
    error::{SimpleError, SimpleErrorKind},
    input::LocatedInput,
    term::TermObj,
    Context,
};

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
                        _ => {
                            println!(
                                "{:x?} {:?} \t\t\t{:?}",
                                &e.0.inner[0..16usize.min(e.0.inner.len())],
                                e.0.span,
                                e.1
                            )
                        }
                    }
                }
            }
        },
    }
}
