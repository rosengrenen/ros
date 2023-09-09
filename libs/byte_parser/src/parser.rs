use super::{input::Input, ParserResult};

pub trait Parser<'input, Out> {
    fn parse(&self, input: Input<'input>) -> ParserResult<'input, Out>;

    // fn map<F, O1, O2>(self, f: F) -> Map<Self, F>
    // where
    //     F: Fn(O1) -> O2,
    //     Self: Sized,
    // {
    //     Map { parser: self, f }
    // }

    // fn flat_map<G, H, O2>(self, g: G) -> FlatMap<Self, G, O>
    // where
    //     G: FnMut(O) -> H,
    //     H: Parser<I, O2, E>,
    //     Self: Sized,
    // { ... }
    // fn and_then<G, O2>(self, g: G) -> AndThen<Self, G, O>
    // where
    //     G: Parser<O, O2, E>,
    //     Self: Sized,
    // { ... }
    // fn and<G, O2>(self, g: G) -> And<Self, G>
    // where
    //     G: Parser<I, O2, E>,
    //     Self: Sized,
    // { ... }
    // fn or<G>(self, g: G) -> Or<Self, G>
    // where
    //     G: Parser<I, O, E>,
    //     Self: Sized,
    // { ... }
    // fn into<O2: From<O>, E2: From<E>>(self) -> Into<Self, O, O2, E, E2>
    // where
    //     Self: Sized,
    // { ... }
}

impl<'input, Out, F> Parser<'input, Out> for F
where
    F: Fn(Input<'input>) -> ParserResult<'input, Out>,
{
    fn parse(&self, input: Input<'input>) -> ParserResult<'input, Out> {
        self(input)
    }
}
