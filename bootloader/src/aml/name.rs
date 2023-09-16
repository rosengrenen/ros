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
    satisfy(|&b: &u8| b == b'_' || b.is_ascii_uppercase())
        .add_context("lead_name_char")
        .parse(input, alloc)
}

fn digit_char<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    alloc: A,
) -> ParseResult<I, u8, E> {
    satisfy(|b: &u8| b.is_ascii_digit())
        .add_context("digit_char")
        .parse(input, alloc)
}

fn name_char<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    alloc: A,
) -> ParseResult<I, u8, E> {
    (&digit_char, &lead_name_char)
        .alt()
        .add_context("name_char")
        .parse(input, alloc)
}

fn root_char<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    alloc: A,
) -> ParseResult<I, (), E> {
    item(0x5c)
        .map(|_| ())
        .add_context("root_char")
        .parse(input, alloc)
}

#[derive(Debug)]
pub struct NameSeg([u8; 4]);

impl NameSeg {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (&lead_name_char, &name_char, &name_char, &name_char)
            .map(|(lead, c1, c2, c3)| Self([lead, c1, c2, c3]))
            .add_context("NameSeg")
            .parse(input, alloc)
    }
}

pub enum NameString<A: Allocator> {
    Absolute(NamePath<A>),
    PathPrefixed(usize, NamePath<A>),
    Relative(NamePath<A>),
}

impl<A: Allocator> core::fmt::Debug for NameString<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Absolute(arg0) => f.debug_tuple("Absolute").field(arg0).finish(),
            Self::PathPrefixed(arg0, arg1) => f
                .debug_tuple("PathPrefixed")
                .field(arg0)
                .field(arg1)
                .finish(),
            Self::Relative(arg0) => f.debug_tuple("Relative").field(arg0).finish(),
        }
    }
}

impl<A: Allocator + Clone> NameString<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            &preceded(&root_char, &NamePath::p.cut()).map(Self::Absolute),
            &(&PrefixPath::p, &NamePath::p.cut()).map(
                |(prefix_path, name_path)| match prefix_path {
                    Some(prefix) => Self::PathPrefixed(prefix.0, name_path),
                    None => Self::Relative(name_path),
                },
            ),
        )
            .alt()
            .add_context("NameString")
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
            .add_context("PrefixPath")
            .parse(input, alloc)
    }
}

pub enum NamePath<A: Allocator> {
    NameSeg(NameSeg),
    DualName(DualNamePath),
    MultiName(MultiNamePath<A>),
    NullName(NullName),
}

impl<A: Allocator> core::fmt::Debug for NamePath<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NameSeg(arg0) => f.debug_tuple("NameSeg").field(arg0).finish(),
            Self::DualName(arg0) => f.debug_tuple("DualName").field(arg0).finish(),
            Self::MultiName(arg0) => f.debug_tuple("MultiName").field(arg0).finish(),
            Self::NullName(arg0) => f.debug_tuple("NullName").field(arg0).finish(),
        }
    }
}

impl<A: Allocator + Clone> NamePath<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            &NameSeg::p.map(Self::NameSeg),
            &DualNamePath::p.map(Self::DualName),
            &MultiNamePath::p.map(Self::MultiName),
            &NullName::p.map(Self::NullName),
        )
            .alt()
            .add_context("NamePath")
            .parse(input, alloc)
    }
}

#[derive(Debug)]
pub struct DualNamePath(NameSeg, NameSeg);

impl DualNamePath {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        preceded(
            &item(0x2e),
            &(&NameSeg::p, &NameSeg::p)
                .cut()
                .map(|(seg0, seg1)| Self(seg0, seg1)),
        )
        .add_context("DualNamePath")
        .parse(input, alloc)
    }
}

pub struct MultiNamePath<A: Allocator>(Vec<NameSeg, A>);

impl<A: Allocator> core::fmt::Debug for MultiNamePath<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MultiNamePath").field(&self.0).finish()
    }
}

impl<A: Allocator + Clone> MultiNamePath<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let (input, seg_count) = preceded(&item(0x2e), &take_one()).parse(input, alloc.clone())?;
        many_n(seg_count as usize, &NameSeg::p)
            .map(MultiNamePath)
            .add_context("MultiNamePath")
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
            &ArgObj::p.map(Self::ArgObj),
            &LocalObj::p.map(Self::LocalObj),
            &NameString::p.map(Self::NameString),
        )
            .alt()
            .add_context("SimpleName")
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
            &SimpleName::p.map(Self::SimpleName),
            &DebugObj::p.map(Self::DebugObj),
            &RefTypeOpcode::p.map(|r| Self::RefTypeOpcode(Box::new(r, box_alloc.clone()).unwrap())),
        )
            .alt()
            .add_context("SuperName")
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
        item(0)
            .map(|_| Self)
            .add_context("NullName")
            .parse(input, alloc)
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
            &SuperName::p.map(Self::SuperName),
            &NullName::p.map(Self::NullName),
        )
            .alt()
            .add_context("Target")
            .parse(input, alloc)
    }
}
