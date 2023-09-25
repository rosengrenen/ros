pub mod aml;
pub mod ops;

use alloc::vec::Vec;
use parser::{
    error::{FromExternalError, ParseError, ParseErrorKind, ParseResult},
    input::Input,
    parser::Parser,
    primitive::{fail::fail, item::take_one, take::take},
    sequence::preceded,
};
use std::{alloc::Allocator, fmt::Debug, marker::PhantomData};

pub struct Context {}

pub fn pkg_length<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    context: &mut Context,
    alloc: A,
) -> ParseResult<I, usize, E> {
    pkg_length_inner
        .map(|(pkg_len, _)| pkg_len)
        .parse(input, context, alloc)
}

fn pkg_length_inner<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    context: &mut Context,
    alloc: A,
) -> ParseResult<I, (usize, usize), E> {
    let (input, lead_byte) = take_one().parse(input, context, alloc.clone())?;
    let extra_bytes = (lead_byte >> 6) as usize;
    if extra_bytes == 0 {
        return Ok((input, (lead_byte as usize, 1)));
    }

    take(extra_bytes)
        .map_res1(move |extra_bytes: I| {
            if lead_byte & 0b0011_0000 != 0 {
                return Err(());
            }

            let mut pkg_length = (lead_byte & 0xf) as usize;
            for (i, b) in extra_bytes.item_iter().enumerate() {
                pkg_length |= (b as usize) << (i * 8 + 4);
            }

            Ok((pkg_length, 1 + extra_bytes.input_len()))
        })
        .add_context("pkg_length")
        .parse(input, context, alloc)
}

pub fn pkg<I, O, E, P, A>(parser: P) -> Pkg<P, E>
where
    I: Input<Item = u8>,
    E: ParseError<I, A>,
    P: Parser<I, Context, A, Output = O, Error = E>,
    A: Allocator + Clone,
{
    Pkg {
        parser,
        error: PhantomData,
    }
}

#[derive(Clone)]
pub struct Pkg<P, E> {
    parser: P,
    error: PhantomData<E>,
}

impl<I, O, E: ParseError<I, A>, P, A: Allocator + Clone> Parser<I, Context, A> for Pkg<P, E>
where
    I: Input<Item = u8>,
    E: ParseError<I, A>,
    P: Parser<I, Context, A, Output = O, Error = E>,
    A: Allocator + Clone,
{
    type Output = O;

    type Error = E;

    fn parse(
        self,
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self::Output, Self::Error> {
        let (input, (pkg_len, pkg_len_bytes_read)) =
            pkg_length_inner
                .add_context("pkg")
                .parse(input, context, alloc.clone())?;
        take(pkg_len - pkg_len_bytes_read)
            .and_then((
                self.parser,
                fail().add_context("whole package was not read"),
            ))
            .map(|(output, _)| output)
            .add_context("pkg")
            .parse(input, context, alloc)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum SimpleErrorKind {
    Context(&'static str),
    Parser(ParseErrorKind),
}

#[derive(Clone)]
pub struct SimpleError<I, A: Allocator> {
    pub errors: Vec<(I, SimpleErrorKind), A>,
}

impl<I, A> Debug for SimpleError<I, A>
where
    I: Clone + Debug,
    A: Allocator + Clone,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SimpleError")
            .field("errors", &self.errors)
            .finish()
    }
}

impl<I, A> ParseError<I, A> for SimpleError<I, A>
where
    I: Input,
    A: Allocator + Clone,
{
    fn from_error_kind(input: I, kind: ParseErrorKind, alloc: A) -> Self {
        let mut errors = Vec::new(alloc);
        errors.push((input, SimpleErrorKind::Parser(kind))).unwrap();
        Self { errors }
    }

    fn append(mut self, input: I, kind: ParseErrorKind) -> Self {
        self.errors
            .push((input, SimpleErrorKind::Parser(kind)))
            .unwrap();
        self
    }

    fn replace(self, input: I, kind: ParseErrorKind) -> Self {
        // TODO: evaluate if replacement is EVER needed
        self.append(input, kind)
        // *self.errors.last_mut().unwrap() = (input, SimpleErrorKind::Parser(kind));
        // self
    }

    fn add_context(mut self, input: I, context: &'static str) -> Self {
        self.errors
            .push((input, SimpleErrorKind::Context(context)))
            .unwrap();
        self
    }
}

impl<I, E, A> FromExternalError<I, E, A> for SimpleError<I, A>
where
    I: Clone,
    A: Allocator + Clone,
{
    fn from_external_error(input: I, kind: ParseErrorKind, _error: E, alloc: A) -> Self {
        let mut errors = Vec::new(alloc);
        errors.push((input, SimpleErrorKind::Parser(kind))).unwrap();
        Self { errors }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    fn split_at_index(&self, index: usize) -> (Self, Self) {
        (
            Self {
                start: self.start,
                end: self.start + index,
            },
            Self {
                start: self.start + index,
                end: self.end,
            },
        )
    }
}

#[derive(Clone)]
pub struct LocatedInput<I> {
    pub inner: I,
    pub span: Span,
}

impl<I> core::fmt::Debug for LocatedInput<I>
where
    I: Input,
    I::Item: core::fmt::Debug,
    I::ItemIter: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("LocatedInput")
            // .field("inner", &self.inner.item_iter().take(32))
            .field("span", &self.span)
            .finish()
    }
}

impl<I> LocatedInput<I>
where
    I: Input,
{
    pub fn new(input: I) -> Self {
        let len = input.input_len();
        Self::with_span(input, Span { start: 0, end: len })
    }

    fn with_span(input: I, span: Span) -> Self {
        Self { inner: input, span }
    }
}

impl<I> Input for LocatedInput<I>
where
    I: Input,
{
    type Item = I::Item;

    type ItemIter = I::ItemIter;

    fn input_len(&self) -> usize {
        self.inner.input_len()
    }

    fn item_iter(&self) -> Self::ItemIter {
        self.inner.item_iter()
    }

    fn split_at_index_unchecked(&self, index: usize) -> (Self, Self) {
        let (left, right) = self.inner.split_at_index_unchecked(index);
        let (left_span, right_span) = self.span.split_at_index(index);
        (
            Self::with_span(left, left_span),
            Self::with_span(right, right_span),
        )
    }
}

pub fn prefixed<I, O1, O2, E, P1, P2, A>(first: P1, second: P2) -> Prefixed<P1, P2, E>
where
    I: Input<Item = u8>,
    E: ParseError<I, A>,
    P1: Parser<I, Context, A, Output = O1, Error = E>,
    P2: Parser<I, Context, A, Output = O2, Error = E>,
    A: Allocator + Clone,
{
    Prefixed {
        first,
        second,
        error: PhantomData,
    }
}

#[derive(Clone)]
pub struct Prefixed<P1, P2, E> {
    first: P1,
    second: P2,
    error: PhantomData<E>,
}

impl<I, O1, O2, E: ParseError<I, A>, P1, P2, A: Allocator + Clone> Parser<I, Context, A>
    for Prefixed<P1, P2, E>
where
    I: Input<Item = u8>,
    E: ParseError<I, A>,
    P1: Parser<I, Context, A, Output = O1, Error = E>,
    P2: Parser<I, Context, A, Output = O2, Error = E>,
    A: Allocator + Clone,
{
    type Output = O2;

    type Error = E;

    fn parse(
        self,
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self::Output, Self::Error> {
        preceded(self.first, self.second.cut())
            .add_context("Prefixed")
            .parse(input, context, alloc)
    }
}
