use super::parser::{take, take_one, Input, ParseResult, ParserError};

pub fn pkg_length<'a>(input: Input<'a>) -> ParseResult<'a, usize> {
    let ((pkg_len, _), input) = pkg_length_inner(input)?;
    Ok((pkg_len, input))
}

pub fn pkg_length_left<'a>(input: Input<'a>) -> ParseResult<'a, usize> {
    let ((pkg_len, pkg_len_bytes_read), input) = pkg_length_inner(input)?;
    Ok((pkg_len - pkg_len_bytes_read, input))
}

fn pkg_length_inner<'a>(input: Input<'a>) -> ParseResult<'a, (usize, usize)> {
    let (lead_byte, input) = take_one(input)?;
    let extra_bytes = (lead_byte >> 6) as usize;
    if extra_bytes == 0 {
        return Ok(((lead_byte as usize, 1), input));
    }

    let (extra_bytes, input) = take(input, extra_bytes)?;
    if lead_byte & 0b0011_0000 != 0 {
        return Err(ParserError::Error);
    }

    let mut pkg_length = (lead_byte & 0xf) as usize;
    for (i, &b) in extra_bytes.iter().enumerate() {
        pkg_length |= (b as usize) << (i * 8 + 4);
    }

    Ok(((pkg_length, 1 + extra_bytes.len()), input))
}

pub fn pkg<'a>(input: Input<'a>) -> ParseResult<'a, Input<'a>> {
    let (pkg_len_left, input) = pkg_length_left(input)?;
    input.split_at_index(pkg_len_left).ok_or(ParserError::Error)
}
