use super::ops::{
    Arg0Op, Arg1Op, Arg2Op, Arg3Op, Arg4Op, Arg5Op, Arg6Op, DebugOp, Local0Op, Local1Op, Local2Op,
    Local3Op, Local4Op, Local5Op, Local6Op, Local7Op,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
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
            &Arg0Op::p.map(|_| Self::Arg0),
            &Arg1Op::p.map(|_| Self::Arg1),
            &Arg2Op::p.map(|_| Self::Arg2),
            &Arg3Op::p.map(|_| Self::Arg3),
            &Arg4Op::p.map(|_| Self::Arg4),
            &Arg5Op::p.map(|_| Self::Arg5),
            &Arg6Op::p.map(|_| Self::Arg6),
        )
            .alt()
            .add_context("ArgObj")
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
            &Local0Op::p.map(|_| Self::Local0),
            &Local1Op::p.map(|_| Self::Local1),
            &Local2Op::p.map(|_| Self::Local2),
            &Local3Op::p.map(|_| Self::Local3),
            &Local4Op::p.map(|_| Self::Local4),
            &Local5Op::p.map(|_| Self::Local5),
            &Local6Op::p.map(|_| Self::Local6),
            &Local7Op::p.map(|_| Self::Local7),
        )
            .alt()
            .add_context("LocalObj")
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
        DebugOp::p
            .map(|_| Self)
            .add_context("DebugObj")
            .parse(input, alloc)
    }
}
