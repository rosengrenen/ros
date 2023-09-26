use super::{
    misc::{ArgObj, DebugObj, LocalObj},
    term::expr::RefTypeOpcode,
};
use crate::aml::{
    ops::{DualNamePrefix, MultiNamePrefix, RootChar},
    prefixed, Context,
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
};
use std::fmt::Debug;

fn lead_name_char<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    context: &mut Context,
    alloc: A,
) -> ParseResult<I, u8, E> {
    satisfy(|&b: &u8| b == b'_' || b.is_ascii_uppercase())
        .add_context("lead_name_char")
        .parse(input, context, alloc)
}

fn digit_char<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    context: &mut Context,
    alloc: A,
) -> ParseResult<I, u8, E> {
    satisfy(|&b: &u8| b.is_ascii_digit())
        .add_context("digit_char")
        .parse(input, context, alloc)
}

fn name_char<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    context: &mut Context,
    alloc: A,
) -> ParseResult<I, u8, E> {
    (digit_char, lead_name_char)
        .alt()
        .add_context("name_char")
        .parse(input, context, alloc)
}

fn root_char<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    context: &mut Context,
    alloc: A,
) -> ParseResult<I, (), E> {
    RootChar::p
        .map(|_| ())
        .add_context("root_char")
        .parse(input, context, alloc)
}

pub struct NameSeg([u8; 4]);

impl NameSeg {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (lead_name_char, name_char, name_char, name_char)
            .map(|(lead, c1, c2, c3)| Self([lead, c1, c2, c3]))
            .add_context("NameSeg")
            .parse(input, context, alloc)
    }
}

impl Debug for NameSeg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("NameSeg")
            .field(unsafe { &core::str::from_utf8_unchecked(&self.0) })
            .finish()
    }
}

#[derive(Debug)]
pub enum NameString<A: Allocator> {
    Absolute(NamePath<A>),
    Relative(usize, NamePath<A>),
}

impl<A: Allocator + Clone> NameString<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            prefixed(root_char, NamePath::p).map(Self::Absolute),
            (PrefixPath::p, NamePath::p.cut()).map(|(prefix_path, name_path)| match prefix_path {
                Some(prefix) => Self::Relative(prefix.0, name_path),
                None => Self::Relative(0, name_path),
            }),
        )
            .alt()
            .add_context("NameString")
            .parse(input, context, alloc)
    }
}

#[derive(Debug)]
pub struct PrefixPath(usize);

impl PrefixPath {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Option<Self>, E> {
        take_while1::<_, E, _, _, _>(|b| b == 0x5e)
            .map(|value| Self(value.input_len()))
            .opt()
            .add_context("PrefixPath")
            .parse(input, context, alloc)
    }
}

#[derive(Debug)]
pub enum NamePath<A: Allocator> {
    NameSeg(NameSeg),
    DualName(DualNamePath),
    MultiName(MultiNamePath<A>),
    NullName(NullName),
}

impl<A: Allocator + Clone> NamePath<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            NameSeg::p.map(Self::NameSeg),
            DualNamePath::p.map(Self::DualName),
            MultiNamePath::p.map(Self::MultiName),
            NullName::p.map(Self::NullName),
        )
            .alt()
            .add_context("NamePath")
            .parse(input, context, alloc)
    }
}

#[derive(Debug)]
pub struct DualNamePath(NameSeg, NameSeg);

impl DualNamePath {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(
            DualNamePrefix::p,
            (NameSeg::p, NameSeg::p).map(|(seg0, seg1)| Self(seg0, seg1)),
        )
        .add_context("DualNamePath")
        .parse(input, context, alloc)
    }
}

#[derive(Debug)]
pub struct MultiNamePath<A: Allocator>(Vec<NameSeg, A>);

impl<A: Allocator + Clone> MultiNamePath<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let (input, seg_count) =
            prefixed(MultiNamePrefix::p, take_one()).parse(input, context, alloc.clone())?;
        many_n(seg_count as usize, NameSeg::p)
            .cut()
            .map(MultiNamePath)
            .add_context("MultiNamePath")
            .parse(input, context, alloc)
    }
}

#[derive(Debug)]
pub enum SimpleName<A: Allocator> {
    NameString(NameString<A>),
    ArgObj(ArgObj),
    LocalObj(LocalObj),
}

impl<A: Allocator + Clone> SimpleName<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            ArgObj::p.map(Self::ArgObj),
            LocalObj::p.map(Self::LocalObj),
            NameString::p.map(Self::NameString),
        )
            .alt()
            .add_context("SimpleName")
            .parse(input, context, alloc)
    }
}

#[derive(Debug)]
pub enum SuperName<A: Allocator> {
    SimpleName(SimpleName<A>),
    DebugObj(DebugObj),
    RefTypeOpcode(Box<RefTypeOpcode<A>, A>),
}

impl<A: Allocator + Clone> SuperName<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let box_alloc = alloc.clone();
        (
            SimpleName::p.map(Self::SimpleName),
            DebugObj::p.map(Self::DebugObj),
            RefTypeOpcode::p.map(|r| Self::RefTypeOpcode(Box::new(r, box_alloc.clone()).unwrap())),
        )
            .alt()
            .add_context("SuperName")
            .parse(input, context, alloc)
    }
}

#[derive(Debug)]
pub struct NullName;

impl NullName {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        item(0x00)
            .map(|_| Self)
            .add_context("NullName")
            .parse(input, context, alloc)
    }
}

#[derive(Debug)]
pub enum Target<A: Allocator> {
    SuperName(SuperName<A>),
    NullName(NullName),
}

impl<A: Allocator + Clone> Target<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            SuperName::p.map(Self::SuperName),
            NullName::p.map(Self::NullName),
        )
            .alt()
            .add_context("Target")
            .parse(input, context, alloc)
    }
}
