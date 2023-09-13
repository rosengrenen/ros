use alloc::vec::Vec;
use core::alloc::Allocator;
use parser::{
    alloc::parser::ParserAlloc,
    error::{ParseError, ParseResult},
    multi::fold_n,
    parser::Parser,
    primitive::{
        item::{item, satisfy, take_one},
        take::take_while1,
    },
    sequence::preceded,
};

fn lead_name_char<'i, E: ParseError<&'i [u8]>>(input: &'i [u8]) -> ParseResult<&'i [u8], u8, E> {
    satisfy(|&b: &u8| b == b'_' || b.is_ascii_uppercase()).parse(input)
}

fn digit_char<'i, E: ParseError<&'i [u8]>>(input: &'i [u8]) -> ParseResult<&'i [u8], u8, E> {
    satisfy(|b: &u8| b.is_ascii_digit()).parse(input)
}

fn name_char<'i, E: ParseError<&'i [u8]>>(input: &'i [u8]) -> ParseResult<&'i [u8], u8, E> {
    (digit_char, lead_name_char).alt().parse(input)
}

#[derive(Debug)]
pub struct RootChar;

pub fn root_char<'i, E: ParseError<&'i [u8]>>(
    input: &'i [u8],
) -> ParseResult<&'i [u8], RootChar, E> {
    item(0x5c).map(|_| RootChar).parse(input)
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

pub struct NameString<'alloc, A: Allocator> {
    prefix: NameStringPrefix,
    name_path: NamePath<'alloc, A>,
}

impl<'alloc, A: Allocator> core::fmt::Debug for NameString<'alloc, A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("NameString")
            .field("prefix", &self.prefix)
            .field("name_path", &self.name_path)
            .finish()
    }
}

#[derive(Debug)]
pub enum NameStringPrefix {
    RootChar(RootChar),
    PrefixPath(PrefixPath),
    None,
}

pub fn name_string<'i, 'alloc, E: ParseError<&'i [u8]>, A: Allocator>(
    input: &'i [u8],
    alloc: &'alloc A,
) -> ParseResult<&'i [u8], NameString<'alloc, A>, E> {
    (
        (root_char.into_alloc(), name_path.cut()).map(|(root_char, name_path)| NameString {
            prefix: NameStringPrefix::RootChar(root_char),
            name_path,
        }),
        (prefix_path.into_alloc(), name_path).map(|(prefix_path, name_path)| NameString {
            prefix: match prefix_path {
                Some(prefix_path) => NameStringPrefix::PrefixPath(prefix_path),
                None => NameStringPrefix::None,
            },
            name_path,
        }),
    )
        .alt()
        .parse_alloc(input, alloc)
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

pub enum NamePath<'alloc, A: Allocator> {
    NameSeg(NameSeg),
    DualName(DualNamePath),
    MultiName(MultiNamePath<'alloc, A>),
    NullName(NullName),
}

impl<'alloc, A: Allocator> core::fmt::Debug for NamePath<'alloc, A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NameSeg(name_seg) => f.debug_tuple("NameSeg").field(name_seg).finish(),
            Self::DualName(dual) => f.debug_tuple("DualName").field(dual).finish(),
            Self::MultiName(multi) => f.debug_tuple("MultiName").field(multi).finish(),
            Self::NullName(null) => f.debug_tuple("NullName").field(null).finish(),
        }
    }
}

fn name_path<'i, 'alloc, E: ParseError<&'i [u8]>, A: Allocator>(
    input: &'i [u8],
    alloc: &'alloc A,
) -> ParseResult<&'i [u8], NamePath<'alloc, A>, E> {
    (
        name_seg.into_alloc().map(NamePath::NameSeg),
        dual_name_path.into_alloc().map(NamePath::DualName),
        multi_name_path.map(NamePath::MultiName),
        null_name.into_alloc().map(NamePath::NullName),
    )
        .alt()
        .parse_alloc(input, alloc)
}

pub struct DualNamePath(NameSeg, NameSeg);

impl core::fmt::Debug for DualNamePath {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DualNamePath")
            .field(&format_args!("{:?}{:?}", self.0, self.1))
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
            .map(|(seg0, seg1)| DualNamePath(seg0, seg1)),
    )
    .parse(input)
}

pub struct MultiNamePath<'alloc, A: Allocator>(Vec<'alloc, NameSeg, A>);

impl<'alloc, A: Allocator> core::fmt::Debug for MultiNamePath<'alloc, A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MultiNamePath").field(&self.0).finish()
    }
}

fn multi_name_path<'i, 'alloc, E, A>(
    input: &'i [u8],
    alloc: &'alloc A,
) -> ParseResult<&'i [u8], MultiNamePath<'alloc, A>, E>
where
    E: ParseError<&'i [u8]>,
    A: Allocator,
{
    let (input, seg_count) = preceded(item(0x2e), take_one()).parse(input)?;

    fold_n(
        seg_count as usize,
        name_seg,
        || Vec::new(alloc),
        |mut segments, segment| {
            segments.push(segment).unwrap();
            segments
        },
    )
    .map(MultiNamePath)
    .parse(input)
}

// Simple name

// Super name

#[derive(Debug)]
pub struct NullName;

fn null_name<'i, E: ParseError<&'i [u8]>>(input: &'i [u8]) -> ParseResult<&'i [u8], NullName, E> {
    item(0).map(|_| NullName).parse(input)
}

// Target

// Utils
