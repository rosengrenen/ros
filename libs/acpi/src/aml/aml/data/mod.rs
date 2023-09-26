pub mod const_integer;
pub mod integer;

use self::const_integer::ConstInteger;
use super::term::expr::{buffer::Buffer, pkg::Pkg, var_pkg::VarPkg};
use crate::aml::{
    ops::{OneOp, OnesOp, RevisionOp, StringPrefix, ZeroOp},
    prefixed, Context,
};
use alloc::vec::Vec;
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    multi::many::many,
    parser::Parser,
    primitive::item::{item, satisfy},
};

#[derive(Debug)]
pub enum ComputationalData<A: Allocator> {
    ConstInteger(ConstInteger),
    String(String<A>),
    ConstObj(ConstObj),
    RevisionOp(RevisionOp),
    Buffer(Buffer<A>),
}

impl<A: Allocator + Clone> ComputationalData<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            ConstInteger::p.map(Self::ConstInteger),
            String::p.map(Self::String),
            ConstObj::p.map(Self::ConstObj),
            RevisionOp::p.map(Self::RevisionOp),
            Buffer::p.map(Self::Buffer),
        )
            .alt()
            .add_context("ComputationalData")
            .parse(input, context, alloc)
    }
}

#[derive(Debug)]
pub enum DataObj<A: Allocator> {
    ComputationalData(ComputationalData<A>),
    Pkg(Pkg<A>),
    VarPkg(VarPkg<A>),
}

impl<A: Allocator + Clone> DataObj<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            ComputationalData::p.map(Self::ComputationalData),
            Pkg::p.map(Self::Pkg),
            VarPkg::p.map(Self::VarPkg),
        )
            .alt()
            .add_context("DataObj")
            .parse(input, context, alloc)
    }
}

#[derive(Debug)]
pub enum DataRefObj<A: Allocator> {
    DataObj(DataObj<A>),
    // ObjRef(ObjRef),
}

impl<A: Allocator + Clone> DataRefObj<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
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
            .parse(input, context, alloc)
    }
}

#[derive(Debug)]
pub struct String<A: Allocator>(Vec<u8, A>);

impl<A: Allocator + Clone> String<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(StringPrefix::p, (many(ascii_char), null_char))
            .map(|(char_list, _)| Self(char_list))
            .add_context("String")
            .parse(input, context, alloc)
    }
}

#[derive(Debug)]
pub enum ConstObj {
    ZeroOp(ZeroOp),
    OneOp(OneOp),
    OnesOp(OnesOp),
}

impl ConstObj {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            ZeroOp::p.map(Self::ZeroOp),
            OneOp::p.map(Self::OneOp),
            OnesOp::p.map(Self::OnesOp),
        )
            .alt()
            .add_context("ConstObj")
            .parse(input, context, alloc)
    }
}

pub fn ascii_char<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    context: &mut Context,
    alloc: A,
) -> ParseResult<I, u8, E> {
    satisfy(|b: &u8| (0x01..=0x7f).contains(b))
        .add_context("ascii_char")
        .parse(input, context, alloc)
}

// macro_rules! parse_fn {
//     ($name:ident, $ret:ty => $parser:block) => {
//         fn $name<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
//             input: I,
//             context: &mut Context,
//             alloc: A,
//         ) -> ParseResult<I, $ret, E>
//         where
//             E: ParseError<I, A>,
//             A: Allocator + Clone,
//         {
//             $parser
//                 .add_context(stringify!($name))
//                 .parse(input, context, alloc)
//         }
//     };
// }

// parse_fn!(null_char, () => {
//   item(0x00)
//     .map(|_| ())
// });

fn null_char<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    context: &mut Context,
    alloc: A,
) -> ParseResult<I, (), E>
where
    E: ParseError<I, A>,
    A: Allocator + Clone,
{
    item(0x00)
        .map(|_| ())
        .add_context("null_char")
        .parse(input, context, alloc)
}
