use super::data::ExtOpPrefix;
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
    primitive::item::item,
};

#[derive(Debug)]
pub enum ArgObj {
    Arg0,
    Arg1,
    Arg2,
    Arg3,
    Arg4,
    Arg5,
    Arg6,
}

impl ArgObj {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            item(0x68).map(|_| Self::Arg0),
            item(0x69).map(|_| Self::Arg1),
            item(0x6a).map(|_| Self::Arg2),
            item(0x6b).map(|_| Self::Arg3),
            item(0x6c).map(|_| Self::Arg4),
            item(0x6d).map(|_| Self::Arg5),
            item(0x6e).map(|_| Self::Arg6),
        )
            .alt()
            .parse(input, alloc)
    }
}

#[derive(Debug)]
pub enum LocalObj {
    Local0,
    Local1,
    Local2,
    Local3,
    Local4,
    Local5,
    Local6,
    Local7,
}

impl LocalObj {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            item(0x60).map(|_| Self::Local0),
            item(0x61).map(|_| Self::Local1),
            item(0x62).map(|_| Self::Local2),
            item(0x63).map(|_| Self::Local3),
            item(0x64).map(|_| Self::Local4),
            item(0x65).map(|_| Self::Local5),
            item(0x66).map(|_| Self::Local6),
            item(0x67).map(|_| Self::Local7),
        )
            .alt()
            .parse(input, alloc)
    }
}

#[derive(Debug)]
pub struct DebugObj;

impl DebugObj {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (ExtOpPrefix::p, item(0x32).map(|_| LocalObj::Local0))
            .map(|_| DebugObj)
            .parse(input, alloc)
    }
}
