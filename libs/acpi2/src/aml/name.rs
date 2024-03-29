use core::alloc::Allocator;

use alloc::{boxed::Box, vec::Vec};

use super::{
    context::Context,
    misc::{ArgObj, DebugObj, LocalObj},
    ops::{DualNamePrefix, MultiNamePrefix, ParentPrefixChar, RootChar},
    parser::{fail, item, satisfy, take_one, Input, ParseResult, ParserError},
    term::expr::RefTypeOpcode,
};

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
pub struct NameSeg([u8; 4]);

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

// impl<A: Allocator> core::fmt::Debug for NameString<A> {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         match self {
//             Self::Absolute(name) => f.debug_tuple("Absolute").field(&name).finish(),
//             Self::Relative(count, name) => {
//                 f.debug_tuple("Relative").field(count).field(&name).finish()
//             }
//         }
//     }
// }

pub struct PrefixPath {
    length: usize,
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
        if let Ok((value, input)) = NameSeg::parse(input) {
            return Ok((Self::NameSeg(value), input));
        }

        if let Ok((value, input)) = DualNamePath::parse(input) {
            return Ok((Self::DualNamePath(value), input));
        }

        if let Ok((value, input)) = MultiNamePath::parse(input, alloc) {
            return Ok((Self::MultiNamePath(value), input));
        }

        let (value, input) = NullName::parse(input)?;
        Ok((Self::NullName(value), input))
    }
}

pub struct DualNamePath {
    pub first: NameSeg,
    pub second: NameSeg,
}

impl DualNamePath {
    pub fn parse(input: Input) -> ParseResult<Self> {
        let (_, input) = DualNamePrefix::parse(input)?;
        fail(Self::parse_inner(input))
    }

    fn parse_inner(input: Input) -> ParseResult<Self> {
        let (first, input) = NameSeg::parse(input)?;
        let (second, input) = NameSeg::parse(input)?;
        Ok((Self { first, second }, input))
    }
}

pub struct MultiNamePath<A: Allocator>(pub Vec<NameSeg, A>);

impl<A: Allocator> core::fmt::Debug for MultiNamePath<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MultiNamePath").field(&self.0).finish()
    }
}

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

pub enum SimpleName<A: Allocator> {
    ArgObj(ArgObj),
    LocalObj(LocalObj),
    NameString(NameString<A>),
}

impl<A: Allocator + Clone> SimpleName<A> {
    pub fn parse<'a>(input: Input<'a>, alloc: A) -> ParseResult<'a, Self> {
        if let Ok((value, input)) = ArgObj::parse(input) {
            return Ok((Self::ArgObj(value), input));
        }

        if let Ok((value, input)) = LocalObj::parse(input) {
            return Ok((Self::LocalObj(value), input));
        }

        let (value, input) = NameString::parse(input, alloc)?;
        Ok((Self::NameString(value), input))
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
        if let Ok((value, input)) = SimpleName::parse(input, alloc.clone()) {
            return Ok((Self::SimpleName(value), input));
        }

        if let Ok((value, input)) = DebugObj::parse(input) {
            return Ok((Self::DebugObj(value), input));
        }

        let (value, input) = RefTypeOpcode::parse(input, context, alloc.clone())?;
        Ok((Self::RefTypeOpcode(Box::new(value, alloc).unwrap()), input))
    }
}

// impl<A: Allocator> core::fmt::Debug for SuperName<A> {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         match self {
//             Self::SimpleName(inner) => f.debug_tuple("SimpleName").field(&inner).finish(),
//             Self::DebugObj(inner) => f.debug_tuple("DebugObj").field(&inner).finish(),
//             Self::RefTypeOpcode(inner) => f.debug_tuple("RefTypeOpcode").field(&inner).finish(),
//         }
//     }
// }

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
        if let Ok((value, input)) = SuperName::parse(input, context, alloc) {
            return Ok((Self::SuperName(value), input));
        }

        let (value, input) = NullName::parse(input)?;
        Ok((Self::NullName(value), input))
    }
}
