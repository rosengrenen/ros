#![no_std]
#![feature(trait_alias)]

#[macro_use]
pub mod macros;

pub mod branch;
pub mod combinator;
pub mod error;
pub mod input;
pub mod primitive;
pub mod sequence;

use error::ParseError;

use self::input::Input;

pub type ParserResult<'input, O> = Result<(Input<'input>, O, Span), ParseError<'input>>;

pub trait ParserFn<'input, O> = Fn(Input<'input>) -> ParserResult<'input, O>;

#[derive(Clone, Copy, Debug, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn split_at(&self, index: usize) -> (Self, Self) {
        (
            Span::new(self.start, self.start + index),
            Span::new(self.start + index, self.end),
        )
    }

    pub fn combine(&self, other: Self) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    pub fn combine_many(spans: &[Self]) -> Self {
        spans
            .iter()
            .fold(spans[0], |combined, &current| combined.combine(current))
    }
}
