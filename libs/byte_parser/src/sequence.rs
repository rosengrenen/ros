use super::{input::Input, parser::Parser, ParserResult};

pub fn tuple<'input, O, T: Tuple<'input, O>>(list: T) -> impl Parser<'input, O> {
    move |input| list.parse(input)
}

pub trait Tuple<'input, Out> {
    fn parse(&self, input: Input<'input>) -> ParserResult<'input, Out>;
}

impl<'input, Out1, F1> Tuple<'input, (Out1,)> for (F1,)
where
    F1: Parser<'input, Out1>,
{
    fn parse(&self, input: Input<'input>) -> ParserResult<'input, (Out1,)> {
        let (input, output, span) = self.0.parse(input)?;
        Ok((input, (output,), span))
    }
}

macro_rules! tuple_trait(
    ($out1:ident $f1:ident, $out2: ident $f2:ident, $($out:ident $f:ident),*) => (
        tuple_trait!(__impl $out1 $f1, $out2 $f2; $($out $f),*);
    );
    (__impl $($out:ident $f: ident),+; $out1:ident $f1:ident, $($out2:ident $f2:ident),*) => (
        tuple_trait_impl!($($out $f),+);
        tuple_trait!(__impl $($out $f),+ , $out1 $f1; $($out2 $f2),*);
    );
    (__impl $($out:ident $f: ident),+; $out1:ident $f1:ident) => (
        tuple_trait_impl!($($out $f),+);
        tuple_trait_impl!($($out $f),+, $out1 $f1);
    );
);

macro_rules! tuple_trait_impl {
    ($($out:ident $f:ident),+) => {
        impl<'input, $($out),+, $($f),+> Tuple<'input, ($($out),+,)> for ($($f),+,)
        where
            $($f: Parser<'input, $out>),+
        {
            fn parse(&self, input: Input<'input>) -> ParserResult<'input, ($($out),+,)> {
                tuple_trait_inner!(0, self, input, (), (), $($f)+)
            }
        }
    };
}

macro_rules! tuple_trait_inner(
    ($iter:tt, $self:expr, $input:expr, (), (), $head:ident $($f:ident)+) => ({
        let (input, output, span) = $self.$iter.parse($input)?;
        succ!($iter, tuple_trait_inner!($self, input, (output), (span), $($f)+))
    });
    ($iter:tt, $self:expr, $input:expr, ($($output:tt)*), ($($span:tt)*), $head:ident $($f:ident)+) => ({
        let (input, output, span) = $self.$iter.parse($input.clone())?;
        succ!($iter, tuple_trait_inner!($self, input, ($($output)*, output), ($($span)*, span), $($f)+))
    });
    ($iter:tt, $self:expr, $input:expr, ($($output:tt)*), ($($span:tt)*), $head:ident) => ({
        let (input, output, span) = $self.$iter.parse($input.clone())?;
        Ok((input, ($($output)*, output), crate::Span::combine_many(&[$($span)*, span])))
    });
);

tuple_trait!(Out1 F1, Out2 F2, Out3 F3, Out4 F4, Out5 F5, Out6 F6, Out7 F7, Out8 F8);
