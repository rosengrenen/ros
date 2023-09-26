use super::integer::{byte_data, dword_data, qword_data, word_data};
use crate::aml::{
    ops::{BytePrefix, DWordPrefix, QWordPrefix, WordPrefix},
    prefixed, Context,
};
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};
use std::alloc::Allocator;

#[derive(Debug)]
pub enum ConstInteger {
    ByteConst(ByteConst),
    WordConst(WordConst),
    DWordConst(DWordConst),
    QWordConst(QWordConst),
}

impl ConstInteger {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            ByteConst::p.map(Self::ByteConst),
            WordConst::p.map(Self::WordConst),
            DWordConst::p.map(Self::DWordConst),
            QWordConst::p.map(Self::QWordConst),
        )
            .alt()
            .add_context("ConstInteger")
            .parse(input, context, alloc)
    }
}

macro_rules! const_integer {
    ($name:ident, $ty:ty, $op:ident, $parser:ident) => {
        #[derive(Debug)]
        pub struct $name($ty);

        impl $name {
            pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
                input: I,
                context: &mut Context,
                alloc: A,
            ) -> ParseResult<I, Self, E> {
                prefixed($op::p, $parser)
                    .map(Self)
                    .add_context(stringify!($name))
                    .parse(input, context, alloc)
            }
        }
    };
}

const_integer!(ByteConst, u8, BytePrefix, byte_data);
const_integer!(WordConst, u16, WordPrefix, word_data);
const_integer!(DWordConst, u32, DWordPrefix, dword_data);
const_integer!(QWordConst, u64, QWordPrefix, qword_data);
