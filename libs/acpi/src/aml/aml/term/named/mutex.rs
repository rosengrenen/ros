use crate::aml::{
    aml::{data::integer::byte_data, name::NameString},
    ops::MutexOp,
    prefixed, Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub struct Mutex<A: Allocator> {
    pub name: NameString<A>,
    pub flags: SyncFlags,
}

impl<A: Allocator + Clone> Mutex<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(MutexOp::p, (NameString::p, SyncFlags::p))
            .map(|(name, flags)| Self { name, flags })
            .add_context("Mutex")
            .parse(input, context, alloc)
    }
}

#[derive(Debug)]
pub struct SyncFlags(u8);

impl SyncFlags {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        byte_data
            .map(Self)
            .add_context("SyncFlags")
            .parse(input, context, alloc)
    }
}
