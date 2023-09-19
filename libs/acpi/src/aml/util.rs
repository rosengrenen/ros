use super::ParseResult;

pub fn take(input: &[u8], count: usize) -> ParseResult<&[u8], &[u8]> {
    if input.len() < count {
        return Err(());
    }

    let (output, input) = input.split_at(count);
    Ok((input, output))
}

pub fn take_one(input: &[u8]) -> ParseResult<&[u8], u8> {
    let (input, output) = take(input, 1)?;
    Ok((input, output[0]))
}

pub fn item(input: &[u8], item: u8) -> ParseResult<&[u8], u8> {
    let (input, output) = take_one(input)?;
    if output != item {
        return Err(());
    }

    Ok((input, output))
}
