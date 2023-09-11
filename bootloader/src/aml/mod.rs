use core::alloc::Allocator;

use crate::sprintln;
use alloc::vec::Vec;
use parser::{
    branch::alt,
    error::{FromExternalError, ParseError, ParseResult},
    input::Input,
    multi::fold,
    parser::Parser,
    primitive::{item, satisfy, take, take_one, take_while1},
    sequence::preceded,
};

pub enum AmlError {
    None,
}

pub fn definition_blocks<'i, 'alloc, E, A>(
    input: &'i [u8],
    alloc: &'alloc A,
) -> ParseResult<&'i [u8], (), E>
where
    E: ParseError<&'i [u8]> + FromExternalError<&'i [u8], AmlError>,
    A: Allocator,
{
    fold(
        def_scope.cut(),
        || Vec::new(alloc),
        |mut acc, name_string| {
            sprintln!("{:?}", name_string);
            acc.push(name_string).unwrap();
            acc
        },
    )
    .map(|output| {
        sprintln!("{:?}", output);
    })
    .parse(input)
}

pub fn def_scope<'i, E>(input: &'i [u8]) -> ParseResult<&'i [u8], NameString, E>
where
    E: ParseError<&'i [u8]> + FromExternalError<&'i [u8], AmlError>,
{
    // TODO: write prefix helper
    let (input, pkg_len) = preceded(item(0x10), pkg_length.cut()).parse(input)?;
    let (input, rest) = input.split_at_index(pkg_len);
    let (_, output) = name_string.cut().parse(input)?;
    Ok((rest, output))
}

pub fn pkg_length<'i, E>(input: &'i [u8]) -> ParseResult<&'i [u8], usize, E>
where
    E: ParseError<&'i [u8]> + FromExternalError<&'i [u8], AmlError>,
{
    let (input, lead_byte) = take_one().parse(input)?;
    let extra_bytes = (lead_byte >> 6) as usize;
    if extra_bytes == 0 {
        return Ok((input, lead_byte as usize - 1));
    }

    take::<&'i [u8], E>(extra_bytes)
        .map_res(move |extra_bytes| {
            if lead_byte & 0b0011_0000 != 0 {
                return Err(AmlError::None);
            }

            let mut pkg_length = (lead_byte & 0xf) as usize;
            for (i, &b) in extra_bytes.iter().enumerate() {
                pkg_length |= (b as usize) << (i * 8 + 4);
            }

            Ok(pkg_length - 1 - extra_bytes.len())
        })
        .parse(input)
}

#[derive(Debug)]
pub struct NameString {
    prefix: NameStringPrefix,
    name_path: NamePath,
}

#[derive(Debug)]
pub enum NameStringPrefix {
    RootChar(RootChar),
    PrefixPath(PrefixPath),
    None,
}

pub fn name_string<'i, E: ParseError<&'i [u8]>>(
    input: &'i [u8],
) -> ParseResult<&'i [u8], NameString, E> {
    alt((
        (root_char, name_path.cut()).map(|(root_char, name_path)| NameString {
            prefix: NameStringPrefix::RootChar(root_char),
            name_path,
        }),
        (prefix_path, name_path).map(|(prefix_path, name_path)| NameString {
            prefix: match prefix_path {
                Some(prefix_path) => NameStringPrefix::PrefixPath(prefix_path),
                None => NameStringPrefix::None,
            },
            name_path,
        }),
    ))
    .parse(input)
}

#[derive(Debug)]
pub struct RootChar;

pub fn root_char<'i, E: ParseError<&'i [u8]>>(
    input: &'i [u8],
) -> ParseResult<&'i [u8], RootChar, E> {
    item(0x5c).map(|_| RootChar).parse(input)
}

#[derive(Debug)]
pub struct PrefixPath(usize);

pub fn prefix_path<'i, E: ParseError<&'i [u8]>>(
    input: &'i [u8],
) -> ParseResult<&'i [u8], Option<PrefixPath>, E> {
    take_while1::<&'i [u8], E, _>(|b| b == 0x5e)
        .map(|value| PrefixPath(value.len()))
        .opt()
        .parse(input)
}

#[derive(Debug)]
pub enum NamePath {
    NameSeg(NameSeg),
    DualName(DualNamePath),
    NullName(NullName),
}

fn name_path<'i, E: ParseError<&'i [u8]>>(input: &'i [u8]) -> ParseResult<&'i [u8], NamePath, E> {
    alt((
        name_seg.map(NamePath::NameSeg),
        dual_name_path.map(NamePath::DualName),
        null_name.map(NamePath::NullName),
    ))
    .parse(input)
}

pub struct DualNamePath([u8; 8]);

impl core::fmt::Debug for DualNamePath {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DualNamePath")
            .field(unsafe { &core::str::from_utf8_unchecked(&self.0) })
            .finish()
    }
}

fn dual_name_path<'i, E: ParseError<&'i [u8]>>(
    input: &'i [u8],
) -> ParseResult<&'i [u8], DualNamePath, E> {
    preceded(
        item(0x2e),
        (name_seg, name_seg)
            .cut()
            .map(|(seg0, seg1)| DualNamePath(concat_arrays(seg0.0, seg1.0))),
    )
    .parse(input)
}

fn concat_arrays<T, const A: usize, const B: usize, const C: usize>(
    a: [T; A],
    b: [T; B],
) -> [T; C] {
    assert_eq!(A + B, C);
    let mut iter = a.into_iter().chain(b);
    core::array::from_fn(|_| iter.next().unwrap())
}

#[derive(Debug)]
pub struct NullName;

fn null_name<'i, E: ParseError<&'i [u8]>>(input: &'i [u8]) -> ParseResult<&'i [u8], NullName, E> {
    item(0).map(|_| NullName).parse(input)
}

pub struct NameSeg([u8; 4]);

impl core::fmt::Debug for NameSeg {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NameSeg")
            .field(unsafe { &core::str::from_utf8_unchecked(&self.0) })
            .finish()
    }
}

fn name_seg<'i, E: ParseError<&'i [u8]>>(input: &'i [u8]) -> ParseResult<&'i [u8], NameSeg, E> {
    (lead_name_char, name_char, name_char, name_char)
        .map(|(lead, c1, c2, c3)| NameSeg([lead, c1, c2, c3]))
        .parse(input)
}

fn lead_name_char<'i, E: ParseError<&'i [u8]>>(input: &'i [u8]) -> ParseResult<&'i [u8], u8, E> {
    satisfy(|b: u8| b == b'_' || b.is_ascii_uppercase()).parse(input)
}

fn digit_char<'i, E: ParseError<&'i [u8]>>(input: &'i [u8]) -> ParseResult<&'i [u8], u8, E> {
    satisfy(|b: u8| b.is_ascii_digit()).parse(input)
}

fn name_char<'i, E: ParseError<&'i [u8]>>(input: &'i [u8]) -> ParseResult<&'i [u8], u8, E> {
    alt((digit_char, lead_name_char)).parse(input)
}
