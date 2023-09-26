#![feature(allocator_api)]
#![allow(dead_code)]
#![allow(unused_attributes)]
#![allow(unused_variables)]

use acpi::aml::{
    error::{SimpleError, SimpleErrorKind},
    input::LocatedInput,
    term::TermObj,
    Context,
};
use parser::multi::many::{many, many_n};
use parser::parser::Parser;
use std::alloc::Global;

fn main() {
    let aml = include_bytes!("../dsdt.aml");
    let aml = LocatedInput::new(aml.as_slice());
    let mut context = Context {};
    let res = many_n(
        3,
        TermObj::p::<LocatedInput<&[u8]>, SimpleError<LocatedInput<&[u8]>, Global>>,
    )
    .parse(aml, &mut context, Global);
    match res {
        Ok(ast) => println!("{:#?}", ast),
        Err(e) => match e {
            parser::error::ParserError::Error(e) => todo!(),
            parser::error::ParserError::Failure(e) => {
                for e in &e.errors {
                    match e.1 {
                        SimpleErrorKind::Context(_) => {
                            println!(
                                "{:x?} {:x?} {:?}",
                                &e.0.inner[0..16usize.min(e.0.inner.len())],
                                e.0.span,
                                e.1
                            )
                        }
                        _ => {
                            println!(
                                "{:x?} {:x?} \t\t\t{:?}",
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
