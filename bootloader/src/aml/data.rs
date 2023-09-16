use super::term::opcodes::expr::{DefBuffer, DefPkg, DefVarPkg};
use alloc::vec::Vec;
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    multi::many::many,
    parser::Parser,
    primitive::item::{item, satisfy, take_one},
    sequence::preceded,
};

pub enum ComputationalData<A: Allocator> {
    ByteConst(ByteConst),
    WordConst(WordConst),
    DWordConst(DWordConst),
    QWordConst(QWordConst),
    String(String<A>),
    ConstObj(ConstObj),
    RevisionOp(RevisionOp),
    DefBuffer(DefBuffer<A>),
}

impl<A: Allocator + Clone> ComputationalData<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            ByteConst::p.map(Self::ByteConst),
            WordConst::p.map(Self::WordConst),
            DWordConst::p.map(Self::DWordConst),
            QWordConst::p.map(Self::QWordConst),
            String::p.map(Self::String),
            ConstObj::p.map(Self::ConstObj),
            RevisionOp::p.map(Self::RevisionOp),
            DefBuffer::p.map(Self::DefBuffer),
        )
            .alt()
            .add_context("ComputationalData")
            .parse(input, alloc)
    }
}

pub enum DataObj<A: Allocator> {
    ComputationalData(ComputationalData<A>),
    DefPkg(DefPkg<A>),
    DefVarPkg(DefVarPkg<A>),
}

impl<A: Allocator + Clone> DataObj<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            ComputationalData::p.map(Self::ComputationalData),
            DefPkg::p.map(Self::DefPkg),
            DefVarPkg::p.map(Self::DefVarPkg),
        )
            .alt()
            .add_context("DataObj")
            .parse(input, alloc)
    }
}

pub enum DataRefObj<A: Allocator> {
    DataObj(DataObj<A>),
    // ObjRef(ObjRef),
}

impl<A: Allocator + Clone> DataRefObj<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        // (
        //     DataObj::p.map(Self::DataObj),
        //     ObjRef::p.map(Self::ObjRef)
        // )
        //     .alt()
        DataObj::p
            .map(Self::DataObj)
            .add_context("DataRefObj")
            .parse(input, alloc)
    }
}

pub struct ByteConst(u8);

impl ByteConst {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let byte_prefix = item(0x0a);
        preceded(byte_prefix, byte_data)
            .map(Self)
            .add_context("ByteConst")
            .parse(input, alloc)
    }
}

pub struct WordConst(u16);

impl WordConst {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let word_prefix = item(0x0b);
        preceded(word_prefix, word_data)
            .map(Self)
            .add_context("WordConst")
            .parse(input, alloc)
    }
}

pub struct DWordConst(u32);

impl DWordConst {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let dword_prefix = item(0x0c);
        preceded(dword_prefix, dword_data)
            .map(Self)
            .add_context("DWordConst")
            .parse(input, alloc)
    }
}

pub struct QWordConst(u64);

impl QWordConst {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let qword_prefix = item(0x0e);
        preceded(qword_prefix, qword_data)
            .map(Self)
            .add_context("QWordConst")
            .parse(input, alloc)
    }
}

pub struct String<A: Allocator>(AsciiCharList<A>);

impl<A: Allocator + Clone> String<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let string_prefix = item(0x0d);
        preceded(string_prefix, (AsciiCharList::p, null_char))
            .map(|(char_list, _)| Self(char_list))
            .add_context("String")
            .parse(input, alloc)
    }
}

pub enum ConstObj {
    ZeroOp(ZeroOp),
    OneOp(OneOp),
    OnesOp(OnesOp),
}

impl ConstObj {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            ZeroOp::p.map(Self::ZeroOp),
            OneOp::p.map(Self::OneOp),
            OnesOp::p.map(Self::OnesOp),
        )
            .alt()
            .add_context("ConstObj")
            .parse(input, alloc)
    }
}

pub struct ByteList<A: Allocator>(Vec<u8, A>);

impl<A: Allocator + Clone> ByteList<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        many(take_one())
            .map(Self)
            .add_context("ByteList")
            .parse(input, alloc)
    }
}

pub fn byte_data<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    alloc: A,
) -> ParseResult<I, u8, E> {
    take_one().add_context("byte_data").parse(input, alloc)
}

pub fn word_data<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    alloc: A,
) -> ParseResult<I, u16, E> {
    (byte_data, byte_data)
        .map(|(lower, higher)| ((higher as u16) << 8) | lower as u16)
        .add_context("word_data")
        .parse(input, alloc)
}

pub fn dword_data<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    alloc: A,
) -> ParseResult<I, u32, E> {
    (word_data, word_data)
        .map(|(lower, higher)| ((higher as u32) << 16) | lower as u32)
        .add_context("dword_data")
        .parse(input, alloc)
}

pub fn qword_data<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    alloc: A,
) -> ParseResult<I, u64, E> {
    (dword_data, dword_data)
        .map(|(lower, higher)| ((higher as u64) << 32) | lower as u64)
        .add_context("qword_data")
        .parse(input, alloc)
}

pub struct AsciiCharList<A: Allocator>(Vec<u8, A>);

impl<A: Allocator + Clone> AsciiCharList<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        many(satisfy(|b: &u8| (0x01..=0x7f).contains(b)))
            .map(Self)
            .add_context("AsciiCharList")
            .parse(input, alloc)
    }
}

fn null_char<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    alloc: A,
) -> ParseResult<I, (), E>
where
    E: ParseError<I, A>,
    A: Allocator + Clone,
{
    item(0x00)
        .map(|_| ())
        .add_context("null_char")
        .parse(input, alloc)
}

pub struct ZeroOp;

impl ZeroOp {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        item(0x00)
            .map(|_| Self)
            .add_context("ZeroOp")
            .parse(input, alloc)
    }
}

pub struct OneOp;

impl OneOp {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        item(0x01)
            .map(|_| Self)
            .add_context("OneOp")
            .parse(input, alloc)
    }
}

pub struct OnesOp;

impl OnesOp {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        item(0xff)
            .map(|_| Self)
            .add_context("OnesOp")
            .parse(input, alloc)
    }
}

pub struct RevisionOp;

impl RevisionOp {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (ExtOpPrefix::p, item(0x30))
            .map(|_| Self)
            .add_context("RevisionOp")
            .parse(input, alloc)
    }
}

pub struct ExtOpPrefix;

impl ExtOpPrefix {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        item(0x5b)
            .map(|_| ExtOpPrefix)
            .add_context("ExtOpPrefix")
            .parse(input, alloc)
    }
}
