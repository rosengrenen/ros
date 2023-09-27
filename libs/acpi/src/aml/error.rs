use alloc::vec::Vec;
use parser::{
    error::{FromExternalError, ParseError, ParseErrorKind},
    input::Input,
};
use std::{alloc::Allocator, fmt::Debug};

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
