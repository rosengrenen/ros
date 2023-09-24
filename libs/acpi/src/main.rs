#![feature(allocator_api)]

use acpi::aml::{Context, TermObj};
use std::alloc::Global;

fn main() {
    let aml = include_bytes!("../dsdt.aml");
    let aml = &aml[36..];
    let mut context = Context {};
    let _ = TermObj::p(aml, &mut context, Global);
    // println!("{:x?}", aml);
    println!("hello");
}
