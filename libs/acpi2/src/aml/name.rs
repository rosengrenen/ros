use alloc::boxed::Box;
use alloc::vec::Vec;
use core::alloc::Allocator;

use super::context::Context;
use super::misc::ArgObj;
use super::misc::DebugObj;
use super::misc::LocalObj;
use super::ops::DualNamePrefix;
use super::ops::MultiNamePrefix;
use super::ops::ParentPrefixChar;
use super::ops::RootChar;
use super::parser::fail;
use super::parser::item;
use super::parser::satisfy;
use super::parser::take_one;
use super::parser::Input;
use super::parser::ParseResult;
use super::parser::ParserError;
use super::term::expr::RefTypeOpcode;

fn lead_name_char<'a>(input: Input<'a>) -> ParseResult<'a, u8> {
    satisfy(input, |b| b == b'_' || b.is_ascii_uppercase())
}

fn digit_char<'a>(input: Input<'a>) -> ParseResult<'a, u8> {
    satisfy(input, |b| b.is_ascii_digit())
}

fn name_char<'a>(input: Input<'a>) -> ParseResult<'a, u8> {
    if let Ok(result) = digit_char(input) {
        return Ok(result);
    }

    lead_name_char(input)
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct NameSeg(pub [u8; 4]);

impl core::fmt::Debug for NameSeg {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NameSeg")
            .field(unsafe { &core::str::from_utf8_unchecked(&self.0) })
            .finish()
    }
}

impl NameSeg {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (c0, input) = lead_name_char(input)?;
        let (c1, input) = name_char(input)?;
        let (c2, input) = name_char(input)?;
        let (c3, input) = name_char(input)?;
        Ok((Self([c0, c1, c2, c3]), input))
    }
}

impl core::fmt::Display for NameSeg {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", unsafe { &core::str::from_utf8_unchecked(&self.0) })
    }
}

pub enum NameString<A: Allocator> {
    Absolute(NamePath<A>),
    Relative(usize, NamePath<A>),
}

impl<A: Allocator + Clone> NameString<A> {
    pub fn parse<'a>(input: Input<'a>, alloc: A) -> ParseResult<'a, Self> {
        if let Ok((_, input)) = RootChar::parse(input) {
            let (path, input) = fail(NamePath::parse(input, alloc))?;
            return Ok((Self::Absolute(path), input));
        }

        if let Ok((prefix_path, input)) = PrefixPath::parse(input) {
            let (path, input) = fail(NamePath::parse(input, alloc))?;
            return Ok((Self::Relative(prefix_path.length, path), input));
        }

        let (path, input) = NamePath::parse(input, alloc)?;
        Ok((Self::Relative(0, path), input))
    }
}

impl<A: Allocator> core::fmt::Debug for NameString<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Absolute(name) => f.debug_tuple("Absolute").field(&name).finish(),
            Self::Relative(count, name) => {
                f.debug_tuple("Relative").field(count).field(&name).finish()
            }
        }
    }
}

pub struct PrefixPath {
    pub length: usize,
}

impl PrefixPath {
    pub fn parse<'a>(mut input: Input<'a>) -> ParseResult<'a, Self> {
        let mut length = 0;
        while let Ok((_, i)) = ParentPrefixChar::parse(input) {
            length += 1;
            input = i;
        }

        if length == 0 {
            return Err(ParserError::Error);
        }

        Ok((Self { length }, input))
    }
}

pub enum NamePath<A: Allocator> {
    NameSeg(NameSeg),
    DualNamePath(DualNamePath),
    MultiNamePath(MultiNamePath<A>),
    NullName(NullName),
}

impl<A: Allocator> NamePath<A> {
    pub fn parse<'a>(input: Input<'a>, alloc: A) -> ParseResult<'a, Self> {
        match NameSeg::parse(input) {
            Ok((value, input)) => return Ok((Self::NameSeg(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match DualNamePath::parse(input) {
            Ok((value, input)) => return Ok((Self::DualNamePath(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match MultiNamePath::parse(input, alloc) {
            Ok((value, input)) => return Ok((Self::MultiNamePath(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        let (value, input) = NullName::parse(input)?;
        Ok((Self::NullName(value), input))
    }
}

impl<A: Allocator> core::fmt::Debug for NamePath<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NameSeg(arg0) => f.debug_tuple("NameSeg").field(arg0).finish(),
            Self::DualNamePath(arg0) => f.debug_tuple("DualNamePath").field(arg0).finish(),
            Self::MultiNamePath(arg0) => f.debug_tuple("MultiNamePath").field(arg0).finish(),
            Self::NullName(arg0) => f.debug_tuple("NullName").field(arg0).finish(),
        }
    }
}

#[derive(Debug)]
pub struct DualNamePath {
    pub first: NameSeg,
    pub second: NameSeg,
}

impl DualNamePath {
    pub fn parse<'a>(input: Input) -> ParseResult<Self> {
        let (_, input) = DualNamePrefix::parse(input)?;
        fail(Self::parse_inner(input))
    }

    fn parse_inner<'a>(input: Input) -> ParseResult<Self> {
        let (first, input) = NameSeg::parse(input)?;
        let (second, input) = NameSeg::parse(input)?;
        Ok((Self { first, second }, input))
    }
}

pub struct MultiNamePath<A: Allocator>(pub Vec<NameSeg, A>);

impl<A: Allocator> MultiNamePath<A> {
    pub fn parse<'a>(input: Input<'a>, alloc: A) -> ParseResult<'a, Self> {
        let (_, input) = MultiNamePrefix::parse(input)?;
        fail(Self::parse_inner(input, alloc))
    }

    fn parse_inner<'a>(input: Input<'a>, alloc: A) -> ParseResult<'a, Self> {
        let (seg_count, mut input) = take_one(input)?;
        let mut segments = Vec::new(alloc);
        for _ in 0..seg_count {
            let (segment, i) = NameSeg::parse(input)?;
            segments.push(segment).unwrap();
            input = i;
        }

        Ok((Self(segments), input))
    }
}

impl<A: Allocator> core::fmt::Debug for MultiNamePath<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MultiNamePath").field(&self.0).finish()
    }
}

pub enum SimpleName<A: Allocator> {
    ArgObj(ArgObj),
    LocalObj(LocalObj),
    NameString(NameString<A>),
}

impl<A: Allocator + Clone> SimpleName<A> {
    pub fn parse<'a>(input: Input<'a>, alloc: A) -> ParseResult<'a, Self> {
        match ArgObj::parse(input) {
            Ok((value, input)) => return Ok((Self::ArgObj(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match LocalObj::parse(input) {
            Ok((value, input)) => return Ok((Self::LocalObj(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        let (value, input) = NameString::parse(input, alloc)?;
        Ok((Self::NameString(value), input))
    }
}

impl<A: Allocator> core::fmt::Debug for SimpleName<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ArgObj(arg0) => f.debug_tuple("ArgObj").field(arg0).finish(),
            Self::LocalObj(arg0) => f.debug_tuple("LocalObj").field(arg0).finish(),
            Self::NameString(arg0) => f.debug_tuple("NameString").field(arg0).finish(),
        }
    }
}

pub enum SuperName<A: Allocator> {
    SimpleName(SimpleName<A>),
    DebugObj(DebugObj),
    RefTypeOpcode(Box<RefTypeOpcode<A>, A>),
}

impl<A: Allocator + Clone> SuperName<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        match SimpleName::parse(input, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::SimpleName(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match DebugObj::parse(input) {
            Ok((value, input)) => return Ok((Self::DebugObj(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        let (value, input) = RefTypeOpcode::parse(input, context, alloc.clone())?;
        Ok((Self::RefTypeOpcode(Box::new(value, alloc).unwrap()), input))
    }
}

impl<A: Allocator> core::fmt::Debug for SuperName<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SimpleName(arg0) => f.debug_tuple("SimpleName").field(arg0).finish(),
            Self::DebugObj(arg0) => f.debug_tuple("DebugObj").field(arg0).finish(),
            Self::RefTypeOpcode(arg0) => f.debug_tuple("RefTypeOpcode").field(arg0).finish(),
        }
    }
}

#[derive(Debug)]
pub struct NullName;

impl NullName {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (_, input) = item(input, 0x00)?;
        Ok((Self, input))
    }
}

pub enum Target<A: Allocator> {
    SuperName(SuperName<A>),
    NullName(NullName),
}

impl<A: Allocator + Clone> Target<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        match SuperName::parse(input, context, alloc) {
            Ok((value, input)) => return Ok((Self::SuperName(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        let (value, input) = NullName::parse(input)?;
        Ok((Self::NullName(value), input))
    }
}

impl<A: Allocator> core::fmt::Debug for Target<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SuperName(arg0) => f.debug_tuple("SuperName").field(arg0).finish(),
            Self::NullName(arg0) => f.debug_tuple("NullName").field(arg0).finish(),
        }
    }
}
