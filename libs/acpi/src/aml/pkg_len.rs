use super::Context;
use core::{alloc::Allocator, marker::PhantomData};
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
    primitive::{fail::fail, item::take_one, take::take},
};

parser_fn!(
    fn pkg_length() -> usize {
        pkg_length_inner.map(|(pkg_len, _)| pkg_len)
    }
);

parser_fn!(
    fn pkg_length_left() -> usize {
        pkg_length_inner.map(|(pkg_len, pkg_len_bytes_read)| pkg_len - pkg_len_bytes_read)
    }
);

fn pkg_length_inner<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    context: &mut Context<A>,
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
    P: Parser<I, Context<A>, A, Output = O, Error = E>,
    A: Allocator + Clone,
{
    Pkg {
        parser,
        error: PhantomData,
    }
}

#[derive(Clone)]
pub struct Pkg<P, E> {
    pub parser: P,
    pub error: PhantomData<E>,
}

impl<I, O, E: ParseError<I, A>, P, A: Allocator + Clone> Parser<I, Context<A>, A> for Pkg<P, E>
where
    I: Input<Item = u8>,
    E: ParseError<I, A>,
    P: Parser<I, Context<A>, A, Output = O, Error = E>,
    A: Allocator + Clone,
{
    type Output = O;

    type Error = E;

    fn parse(
        self,
        input: I,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<I, Self::Output, Self::Error> {
        let (input, (pkg_len, pkg_len_bytes_read)) = pkg_length_inner
            .add_context("pkg pkg_length_inner")
            .parse(input, context, alloc.clone())?;
        let (rest, input) = take(pkg_len - pkg_len_bytes_read)
            .add_context("pkg take")
            .parse(input, context, alloc.clone())?;
        let (_, output) = (
            self.parser,
            fail().add_context("whole package was not read"),
        )
            .map(|(output, _)| output)
            .add_context("pkg parser")
            .parse(input, context, alloc.clone())?;
        Ok((rest, output))
    }
}
