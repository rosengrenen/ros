use crate::aml::Context;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
    primitive::item::take_one,
};
use std::alloc::Allocator;

pub fn byte_data<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    context: &mut Context,
    alloc: A,
) -> ParseResult<I, u8, E> {
    take_one()
        .add_context("byte_data")
        .parse(input, context, alloc)
}

pub fn word_data<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    context: &mut Context,
    alloc: A,
) -> ParseResult<I, u16, E> {
    (byte_data, byte_data)
        .map(|(lower, higher)| ((higher as u16) << 8) | lower as u16)
        .add_context("word_data")
        .parse(input, context, alloc)
}

pub fn dword_data<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    context: &mut Context,
    alloc: A,
) -> ParseResult<I, u32, E> {
    (word_data, word_data)
        .map(|(lower, higher)| ((higher as u32) << 16) | lower as u32)
        .add_context("dword_data")
        .parse(input, context, alloc)
}

pub fn qword_data<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    context: &mut Context,
    alloc: A,
) -> ParseResult<I, u64, E> {
    (dword_data, dword_data)
        .map(|(lower, higher)| ((higher as u64) << 32) | lower as u64)
        .add_context("qword_data")
        .parse(input, context, alloc)
}
