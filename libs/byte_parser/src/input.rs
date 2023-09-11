use crate::error::{ParseError, ParseErrorKind, ParseResult, ParserError};
use core::{iter::Copied, slice::Iter};

pub trait Input: Clone {
    type Item;

    type ItemIter: Iterator<Item = Self::Item>;

    fn input_len(&self) -> usize;

    fn item_iter(&self) -> Self::ItemIter;

    fn split_at_index(&self, index: usize) -> (Self, Self);

    fn position<P>(&self, pred: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool;

    fn split_at_position0<P, E>(&self, pred: P) -> ParseResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
        E: ParseError<Self>,
    {
        match self.position(pred) {
            Some(n) => {
                let (output, input) = self.split_at_index(n);
                Ok((input, output))
            }
            None => {
                let (output, input) = self.split_at_index(self.input_len());
                Ok((input, output))
            }
        }
    }

    fn split_at_position1<P, E>(&self, pred: P, kind: ParseErrorKind) -> ParseResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
        E: ParseError<Self>,
    {
        match self.position(pred) {
            Some(0) => Err(ParserError::Error(E::from_error_kind(self.clone(), kind))),
            Some(n) => {
                let (output, input) = self.split_at_index(n);
                Ok((input, output))
            }
            None => {
                let (output, input) = self.split_at_index(self.input_len());
                Ok((input, output))
            }
        }
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

    fn split_at_index(&self, index: usize) -> (Self, Self) {
        self.split_at(index)
    }

    fn position<P>(&self, pred: P) -> Option<usize>
    where
        P: Fn(u8) -> bool,
    {
        self.iter().position(|&b| pred(b))
    }
}
