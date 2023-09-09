use super::{
    combinator::{map, map_res},
    input::Input,
    ParserFn,
};

pub fn take<'input>(count: usize) -> impl ParserFn<'input, &'input [u8]> {
    move |input| {
        let (taken, rest) = input.split_at_index(count);
        Ok((rest, taken.as_slice(), taken.span))
    }
}

pub fn take_const<'input, const C: usize>() -> impl ParserFn<'input, [u8; C]> {
    map(take(C), |slice| slice_to_array_unchecked::<C>(slice))
}

pub fn take_one<'input>() -> impl ParserFn<'input, u8> {
    map(take_const::<1>(), |i| i[0])
}

pub fn take_while0<'input, P>(pred: P) -> impl ParserFn<'input, &'input [u8]>
where
    P: Fn(u8) -> bool,
{
    move |input: Input<'input>| input.split_at_position0(|b| !pred(b))
}

pub fn take_while1<'input, P>(pred: P) -> impl ParserFn<'input, &'input [u8]>
where
    P: Fn(u8) -> bool,
{
    move |input: Input<'input>| input.split_at_position1(|b| !pred(b))
}

pub fn byte<'input>(byte: u8) -> impl ParserFn<'input, u8> {
    map_res(take_one(), move |b| match b == byte {
        true => Ok(byte),
        false => Err(()),
    })
}

pub fn satisfy<'input, P>(pred: P) -> impl ParserFn<'input, u8>
where
    P: Fn(u8) -> bool,
{
    map_res(take_one(), move |b| match pred(b) {
        true => Ok(b),
        false => Err(()),
    })
}

fn slice_to_array_unchecked<const C: usize>(slice: &[u8]) -> [u8; C] {
    match slice.try_into() {
        Ok(array) => array,
        Err(_) => unreachable!(),
    }
}
