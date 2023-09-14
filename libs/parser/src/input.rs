use crate::error::{ParseError, ParseErrorKind, ParseResult, ParserError};
use core::{alloc::Allocator, iter::Copied, slice::Iter};

pub trait Input: Clone {
    type Item;

    type ItemIter: Iterator<Item = Self::Item>;

    fn input_len(&self) -> usize;

    fn item_iter(&self) -> Self::ItemIter;

    fn split_at_index_unchecked(&self, index: usize) -> (Self, Self);

    fn split_at_index<'alloc, E, A>(
        &self,
        index: usize,
        kind: ParseErrorKind,
        alloc: &'alloc A,
    ) -> ParseResult<Self, Self, E>
    where
        E: ParseError<'alloc, Self, A>,
        A: Allocator,
    {
        if self.input_len() < index {
            return Err(ParserError::Error(E::from_error_kind(
                self.clone(),
                kind,
                alloc,
            )));
        }

        let (output, input) = self.split_at_index_unchecked(index);
        Ok((input, output))
    }

    fn split_at_position_m_n<'alloc, E, P, A>(
        &self,
        min: usize,
        max: usize,
        pred: P,
        kind: ParseErrorKind,
        alloc: &'alloc A,
    ) -> ParseResult<Self, Self, E>
    where
        E: ParseError<'alloc, Self, A>,
        P: Fn(Self::Item) -> bool,
        A: Allocator,
    {
        let mut n = 0;
        let mut iter = self.item_iter();
        while n <= max {
            match iter.next() {
                Some(item) => match pred(item) {
                    true => break,
                    false => {
                        n += 1;
                    }
                },
                None => break,
            }
        }

        if n < min {
            return Err(ParserError::Error(E::from_error_kind(
                self.clone(),
                kind,
                alloc,
            )));
        }

        self.split_at_index(n, kind, alloc)
    }
}

impl<'input> Input for &'input [u8] {
    type Item = u8;

    type ItemIter = Copied<Iter<'input, Self::Item>>;

    fn input_len(&self) -> usize {
        self.len()
    }

    fn item_iter(&self) -> Self::ItemIter {
        self.iter().copied()
    }

    fn split_at_index_unchecked(&self, index: usize) -> (Self, Self) {
        self.split_at(index)
    }
}
