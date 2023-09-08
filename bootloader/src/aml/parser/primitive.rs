use super::{
    combinator::{map, map_res},
    error::ParserError,
    input::Input,
    ParserResult, ParserState,
};

pub fn take<'input>(count: usize) -> impl Fn(ParserState<'input>) -> ParserResult<Input<'input>> {
    move |state| {
        let (taken, rest) = state.input.take_split(count);
        Ok((
            ParserState {
                input: rest,
                span: state.span.with_len(count),
            },
            taken,
        ))
    }
}

pub fn take_const<'input, const C: usize>() -> impl Fn(ParserState<'input>) -> ParserResult<[u8; C]>
{
    map(take(C), |i| slice_to_array_unchecked::<C>(i.as_slice()))
}

pub fn take_one<'input>() -> impl Fn(ParserState<'input>) -> ParserResult<u8> {
    map(take_const::<1>(), |i| i[0])
}

pub fn byte<'input>(byte: u8) -> impl Fn(ParserState<'input>) -> ParserResult<u8> {
    map_res(take_one(), move |b| match b == byte {
        true => Ok(byte),
        false => Err(ParserError),
    })
}

pub fn satisfy<'input, P>(pred: P) -> impl Fn(ParserState<'input>) -> ParserResult<u8>
where
    P: Fn(u8) -> bool,
{
    map_res(take_one(), move |b| match pred(b) {
        true => Ok(b),
        false => Err(ParserError),
    })
}

fn slice_to_array_unchecked<const C: usize>(slice: &[u8]) -> [u8; C] {
    match slice.try_into() {
        Ok(array) => array,
        Err(_) => unreachable!(),
    }
}
