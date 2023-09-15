use super::{
    misc::{ArgObj, DebugObj, LocalObj},
    term::opcodes::expr::RefTypeOpcode,
};
use alloc::{boxed::Box, vec::Vec};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    multi::many::many_n,
    parser::Parser,
    primitive::{
        item::{item, satisfy, take_one},
        take::take_while1,
    },
    sequence::preceded,
};

fn lead_name_char<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    alloc: A,
) -> ParseResult<I, u8, E> {
    satisfy(|&b: &u8| b == b'_' || b.is_ascii_uppercase()).parse(input, alloc)
}

fn digit_char<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    alloc: A,
) -> ParseResult<I, u8, E> {
    satisfy(|b: &u8| b.is_ascii_digit()).parse(input, alloc)
}

fn name_char<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    alloc: A,
) -> ParseResult<I, u8, E> {
    (digit_char, lead_name_char).alt().parse(input, alloc)
}

fn root_char<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    alloc: A,
) -> ParseResult<I, (), E> {
    item(0x5c).map(|_| ()).parse(input, alloc)
}

pub struct NameSeg([u8; 4]);

impl NameSeg {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (lead_name_char, name_char, name_char, name_char)
            .map(|(lead, c1, c2, c3)| Self([lead, c1, c2, c3]))
            .parse(input, alloc)
    }
}

pub enum NameString<A: Allocator> {
    Absolute(NamePath<A>),
    PathPrefixed(usize, NamePath<A>),
    Relative(NamePath<A>),
}

impl<A: Allocator + Clone> NameString<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let root_char = item(0x5c);
        (
            preceded(root_char, NamePath::p.cut()).map(Self::Absolute),
            (PrefixPath::p, NamePath::p.cut()).map(|(prefix_path, name_path)| match prefix_path {
                Some(prefix) => Self::PathPrefixed(prefix.0, name_path),
                None => Self::Relative(name_path),
            }),
        )
            .alt()
            .parse(input, alloc)
    }
}

#[derive(Debug)]
pub struct PrefixPath(usize);

impl PrefixPath {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Option<Self>, E> {
        take_while1::<I, E, _, A>(|b| b == 0x5e)
            .map(|value| Self(value.input_len()))
            .opt()
            .parse(input, alloc)
    }
}

pub enum NamePath<A: Allocator> {
    NameSeg(NameSeg),
    DualName(DualNamePath),
    MultiName(MultiNamePath<A>),
    NullName(NullName),
}

impl<A: Allocator + Clone> NamePath<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            NameSeg::p.map(Self::NameSeg),
            DualNamePath::p.map(Self::DualName),
            MultiNamePath::p.map(Self::MultiName),
            NullName::p.map(Self::NullName),
        )
            .alt()
            .parse(input, alloc)
    }
}

pub struct DualNamePath(NameSeg, NameSeg);

impl DualNamePath {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        preceded(
            item(0x2e),
            (NameSeg::p, NameSeg::p)
                .cut()
                .map(|(seg0, seg1)| Self(seg0, seg1)),
        )
        .parse(input, alloc)
    }
}

pub struct MultiNamePath<A: Allocator>(Vec<NameSeg, A>);

impl<A: Allocator + Clone> MultiNamePath<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let (input, seg_count) = preceded(item(0x2e), take_one()).parse(input, alloc.clone())?;
        many_n(seg_count as usize, NameSeg::p)
            .map(MultiNamePath)
            .parse(input, alloc)
    }
}

pub enum SimpleName<A: Allocator> {
    NameString(NameString<A>),
    ArgObj(ArgObj),
    LocalObj(LocalObj),
}

impl<A: Allocator + Clone> SimpleName<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            NameString::p.map(Self::NameString),
            ArgObj::p.map(Self::ArgObj),
            LocalObj::p.map(Self::LocalObj),
        )
            .alt()
            .parse(input, alloc)
    }
}

pub enum SuperName<A: Allocator> {
    SimpleName(SimpleName<A>),
    DebugObj(DebugObj),
    RefTypeOpcode(Box<RefTypeOpcode<A>, A>),
}

impl<A: Allocator + Clone> SuperName<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let box_alloc = alloc.clone();
        (
            SimpleName::p.map(Self::SimpleName),
            DebugObj::p.map(Self::DebugObj),
            RefTypeOpcode::p.map(|r| Self::RefTypeOpcode(Box::new(r, box_alloc.clone()).unwrap())),
        )
            .alt()
            .parse(input, alloc)
    }
}

#[derive(Debug)]
pub struct NullName;

impl NullName {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        item(0).map(|_| Self).parse(input, alloc)
    }
}

pub enum Target<A: Allocator> {
    SuperName(SuperName<A>),
    NullName(NullName),
}

impl<A: Allocator + Clone> Target<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            SuperName::p.map(Self::SuperName),
            NullName::p.map(Self::NullName),
        )
            .alt()
            .parse(input, alloc)
    }
}
