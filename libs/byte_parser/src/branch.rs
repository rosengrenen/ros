use crate::{input::Input, ParserResult};

use super::parser::Parser;

pub fn alt<'input, O, A: Alt<'input, O>>(list: A) -> impl Parser<'input, O> {
    move |input| list.parse(input)
}

pub trait Alt<'input, Out> {
    fn parse(&self, input: Input<'input>) -> ParserResult<'input, Out>;
}

impl<'input, Out, F1> Alt<'input, Out> for (F1,)
where
    F1: Parser<'input, Out>,
{
    fn parse(&self, input: Input<'input>) -> ParserResult<'input, Out> {
        self.0.parse(input)
    }
}

macro_rules! alt_trait(
  ($f1:ident, $f2:ident, $($f:ident),*) => (
    alt_trait!(__impl $f1, $f2; $($f),*);
  );
  (__impl $($f: ident),+; $f1:ident, $($f2:ident),*) => (
    alt_trait_impl!($($f),+);
    alt_trait!(__impl $($f),+ , $f1; $($f2),*);
  );
  (__impl $($f: ident),+; $f1:ident) => (
    alt_trait_impl!($($f),+);
    alt_trait_impl!($($f),+, $f1);
  );
);

macro_rules! alt_trait_impl {
    ($($f:ident),+) => {
        impl<'input, Out, $($f),+> Alt<'input, Out> for ($($f),+,)
        where
            $($f: Parser<'input, Out>),+
        {
            fn parse(&self, input: Input<'input>) -> ParserResult<'input, Out> {
                alt_trait_inner!(0, self, input, $($f)+)
            }
        }
    };
}

macro_rules! alt_trait_inner(
  ($iter:tt, $self:expr, $input:expr, $head:ident $($f:ident)+) => ({
    match $self.$iter.parse($input.clone()) {
      Ok(inner) => return Ok(inner),
      Err(_) => succ!($iter, alt_trait_inner!($self, $input, $($f)+))
    }
  });
  ($iter:tt, $self:expr, $input:expr, $head:ident) => ({
    $self.$iter.parse($input.clone())
  });
);

alt_trait!(F1, F2, F3, F4, F5, F6, F7, F8);
