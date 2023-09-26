use parser::input::Input;
use std::fmt::Debug;

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

impl<I> Debug for LocatedInput<I>
where
    I: Input,
    I::Item: core::fmt::Debug,
    I::ItemIter: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("LocatedInput")
            .field("span", &self.span)
            .field("inner", &self.inner)
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
    I::Item: Debug,
    I::ItemIter: Debug,
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
