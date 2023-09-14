mod name;

use crate::sprintln;
use core::alloc::Allocator;
use parser::{
    error::{FromExternalError, ParseError, ParseResult},
    multi::many::many,
    parser::Parser,
    primitive::{
        item::{item, take_one},
        take::take,
    },
    sequence::preceded,
};

use self::name::{name_string, NameString};

pub enum AmlError {
    None,
}

pub fn definition_blocks<'i, 'alloc, E, A>(
    input: &'i [u8],
    alloc: &'alloc A,
) -> ParseResult<'alloc, &'i [u8], (), E>
where
    E: ParseError<'alloc, &'i [u8], A> + FromExternalError<'alloc, &'i [u8], AmlError, A>,
    A: Allocator,
{
    many(def_scope)
        .map(|output| {
            sprintln!("{:#?}", output);
        })
        .parse(input, alloc)
}

pub fn def_scope<'i, 'alloc, E, A>(
    input: &'i [u8],
    alloc: &'alloc A,
) -> ParseResult<'alloc, &'i [u8], NameString<'alloc, A>, E>
where
    E: ParseError<'alloc, &'i [u8], A> + FromExternalError<'alloc, &'i [u8], AmlError, A>,
    A: Allocator,
{
    let (input, pkg_len) = preceded(item(0x10), pkg_length.cut()).parse(input, alloc)?;
    let (rest, input) = take(pkg_len).parse(input, alloc)?;
    let (_, output) = name_string.cut().parse(input, alloc)?;
    Ok((rest, output))
}

pub fn pkg_length<'i, 'alloc, E, A>(
    input: &'i [u8],
    alloc: &'alloc A,
) -> ParseResult<'alloc, &'i [u8], usize, E>
where
    E: ParseError<'alloc, &'i [u8], A> + FromExternalError<'alloc, &'i [u8], AmlError, A>,
    A: Allocator,
{
    let (input, lead_byte) = take_one().parse(input, alloc)?;
    let extra_bytes = (lead_byte >> 6) as usize;
    if extra_bytes == 0 {
        return Ok((input, lead_byte as usize - 1));
    }

    take(extra_bytes)
        .map_res(move |extra_bytes: &'i [u8]| {
            if lead_byte & 0b0011_0000 != 0 {
                return Err(AmlError::None);
            }

            let mut pkg_length = (lead_byte & 0xf) as usize;
            for (i, &b) in extra_bytes.iter().enumerate() {
                pkg_length |= (b as usize) << (i * 8 + 4);
            }

            Ok(pkg_length - 1 - extra_bytes.len())
        })
        .parse(input, alloc)
}
