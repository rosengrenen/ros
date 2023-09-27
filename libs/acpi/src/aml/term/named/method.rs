use crate::aml::{
    data::byte_data,
    name::NameString,
    ops::MethodOp,
    pkg_len::{pkg, pkg_length_left},
    prefixed::prefixed,
    term::TermObj,
    Context,
};
use alloc::vec::Vec;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    multi::many::many,
    parser::Parser,
    primitive::{fail::fail, take::take},
};
use std::{alloc::Allocator, fmt::Formatter};

pub struct Method<A: Allocator> {
    pub name: NameString<A>,
    pub flags: u8,
    pub terms: Vec<TermObj<A>, A>,
}

impl<A: Allocator> core::fmt::Debug for Method<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Method")
            .field("name", &self.name)
            .field("flags", &self.flags)
            .field("terms", &self.terms)
            .finish()
    }
}

impl<A: Allocator + Clone> Method<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let (input, _) = MethodOp::p.parse(input, context, alloc.clone())?;
        let (input, pkg_len) =
            pkg_length_left
                .cut()
                .add_context("Method")
                .parse(input, context, alloc.clone())?;
        let (rest, input) =
            take(pkg_len)
                .cut()
                .add_context("Method")
                .parse(input, context, alloc.clone())?;
        let (input, (name, flags)) = (NameString::p, byte_data)
            .cut()
            .add_context("Method")
            .parse(input, context, alloc.clone())?;
        context.add_method(&name, flags as usize & 0x7);
        let (_, (terms, _)) = (many(TermObj::p), fail())
            .add_context("Method")
            .parse(input, context, alloc)?;
        Ok((rest, Self { name, flags, terms }))
    }
}
