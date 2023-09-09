use crate::sprintln;
use byte_parser::{
    branch::alt,
    combinator::{cut, map, map_res, opt},
    error::ParseError,
    input::Input,
    primitive::{byte, satisfy, take, take_one, take_while1},
    sequence::tuple,
    ParserResult,
};

pub fn definition_blocks<'input>(mut input: Input<'input>) -> ParserResult<'input, ()> {
    loop {
        match def_scope(input) {
            Ok((new_input, output, span)) => {
                sprintln!("{:?} {:?}", output, span);
                input = new_input;
            }
            Err(ParseError::Error(error)) => {
                // return accumulated items
                return Err(ParseError::Error(error));
            }
            Err(ParseError::Failure(error)) => return Err(ParseError::Failure(error)),
        }
    }
}

pub fn def_scope<'input>(input: Input<'input>) -> ParserResult<'input, NameString> {
    // TODO: write prefix helper
    let (input, _, _) = byte(0x10)(input)?;
    let (input, len, _) = pkg_length(input)?;
    let (input, rest) = input.split_at_index(len);
    let (_, output, span) = name_string(input)?;
    Ok((rest, output, span))
}

pub fn pkg_length<'input>(input: Input<'input>) -> ParserResult<'input, usize> {
    let (input, lead_byte, span) = take_one()(input)?;
    let extra_bytes = (lead_byte >> 6) as usize;
    if extra_bytes == 0 {
        return Ok((input, lead_byte as usize - 1, span));
    }

    map_res(take(extra_bytes as usize), move |extra_bytes| {
        if lead_byte & 0b0011_0000 != 0 {
            return Err(());
        }

        let mut pkg_length = (lead_byte & 0xf) as usize;
        for (i, &b) in extra_bytes.iter().enumerate() {
            pkg_length |= (b as usize) << (i * 8 + 4);
        }

        Ok(pkg_length - 1 - extra_bytes.len())
    })(input)
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

pub fn name_string<'input>(input: Input<'input>) -> ParserResult<'input, NameString> {
    let root_prefixed = map(
        tuple((root_char, cut(name_path))),
        |(root_char, name_path)| NameString {
            prefix: NameStringPrefix::RootChar(root_char),
            name_path,
        },
    );
    let path_prefixed = map(
        tuple((prefix_path, cut(name_path))),
        |(prefix_path, name_path)| NameString {
            prefix: match prefix_path {
                Some(prefix_path) => NameStringPrefix::PrefixPath(prefix_path),
                None => NameStringPrefix::None,
            },
            name_path,
        },
    );

    alt((root_prefixed, path_prefixed))(input)
}

#[derive(Debug)]
pub struct RootChar;

pub fn root_char<'input>(input: Input<'input>) -> ParserResult<'input, RootChar> {
    map(byte(b'\\'), |_| RootChar)(input)
}

#[derive(Debug)]
pub struct PrefixPath(usize);

pub fn prefix_path<'input>(input: Input<'input>) -> ParserResult<'input, Option<PrefixPath>> {
    opt(map(take_while1(|b| b == b'^'), |value| {
        PrefixPath(value.len())
    }))(input)
}

#[derive(Debug)]
pub enum NamePath {
    NameSeg(NameSeg),
    DualNamePath(DualNamePath),
    NullName(NullName),
}

pub fn name_path<'input>(input: Input<'input>) -> ParserResult<'input, NamePath> {
    alt((
        map(name_seg, |value| NamePath::NameSeg(value)),
        map(dual_name_path, |value| NamePath::DualNamePath(value)),
        map(null_name, |value| NamePath::NullName(value)),
    ))(input)
}

pub struct DualNamePath([u8; 8]);

impl core::fmt::Debug for DualNamePath {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DualNamePath")
            .field(unsafe { &core::str::from_utf8_unchecked(&self.0) })
            .finish()
    }
}

fn dual_name_path<'input>(input: Input<'input>) -> ParserResult<'input, DualNamePath> {
    let (input, _, _) = byte(0x2e)(input)?;
    let (input, (NameSeg(seg0), NameSeg(seg1)), span) = cut(tuple((name_seg, name_seg)))(input)?;
    Ok((input, DualNamePath(concat_arrays(seg0, seg1)), span))
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

fn null_name<'input>(input: Input<'input>) -> ParserResult<'input, NullName> {
    map(byte(0), |_| NullName)(input)
}

pub struct NameSeg([u8; 4]);

impl core::fmt::Debug for NameSeg {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NameSeg")
            .field(unsafe { &core::str::from_utf8_unchecked(&self.0) })
            .finish()
    }
}

fn name_seg<'input>(input: Input<'input>) -> ParserResult<'input, NameSeg> {
    let (input, (lead, c0, c1, c2), span) =
        tuple((lead_name_char, name_char, name_char, name_char))(input)?;
    Ok((input, NameSeg([lead, c0, c1, c2]), span))
}

fn lead_name_char<'input>(input: Input<'input>) -> ParserResult<'input, u8> {
    satisfy(|b| b == b'_' || (b'A'..=b'Z').contains(&b))(input)
}

fn digit_char<'input>(input: Input<'input>) -> ParserResult<'input, u8> {
    satisfy(|b| (b'0'..=b'9').contains(&b))(input)
}

fn name_char<'input>(input: Input<'input>) -> ParserResult<'input, u8> {
    alt((digit_char, lead_name_char))(input)
}
