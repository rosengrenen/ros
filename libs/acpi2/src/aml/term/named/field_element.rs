use crate::aml::{
    data::byte_data,
    name::{NameSeg, NameString},
    parser::{fail, item, Input, ParseResult, ParserError},
    pkg_len::pkg_length,
};
use core::alloc::Allocator;

pub struct Named {
    name: NameSeg,
    len: usize,
}

impl Named {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (name, input) = NameSeg::parse(input)?;
        let (len, input) = pkg_length(input)?;
        Ok((Self { name, len }, input))
    }
}

pub struct Reserved;

impl Reserved {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (_, input) = item(input, 0x00)?;
        Ok((Self, input))
    }
}

pub struct Access {
    ty: u8,
    attrib: u8,
}

impl Access {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (_, input) = item(input, 0x01)?;
        fail(Self::parse_inner(input))
    }

    fn parse_inner<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (ty, input) = byte_data(input)?;
        let (attrib, input) = byte_data(input)?;
        Ok((Self { ty, attrib }, input))
    }
}

pub enum Connect<A: Allocator> {
    NameString(NameString<A>),
    // BufferData(BufferData),
}

impl<A: Allocator + Clone> Connect<A> {
    pub fn parse<'a>(input: Input<'a>, alloc: A) -> ParseResult<'a, Self> {
        let (_, input) = item(input, 0x02)?;
        fail(Self::parse_inner(input, alloc))
    }

    fn parse_inner<'a>(input: Input<'a>, alloc: A) -> ParseResult<'a, Self> {
        let (name, input) = NameString::parse(input, alloc)?;
        Ok((Self::NameString(name), input))
    }
}

// impl<A: Allocator> core::fmt::Debug for Connect<A> {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         match  self {
//             Self::NameString(inner) => f.debug_tuple("NameString").field(inner).finish(),
//         }
//     }
// }

pub struct ExtendedAccess {
    ty: u8,
    attrib: u8,
    len: u8,
}

impl ExtendedAccess {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (_, input) = item(input, 0x13)?;
        fail(Self::parse_inner(input))
    }

    fn parse_inner<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (ty, input) = byte_data(input)?;
        let (attrib, input) = byte_data(input)?;
        let (len, input) = byte_data(input)?;
        Ok((Self { ty, attrib, len }, input))
    }
}

pub enum FieldElement<A: Allocator> {
    Named(Named),
    Reserved(Reserved),
    Access(Access),
    ExtendedAccess(ExtendedAccess),
    Connect(Connect<A>),
}

impl<A: Allocator + Clone> FieldElement<A> {
    pub fn parse<'a>(input: Input<'a>, alloc: A) -> ParseResult<'a, Self> {
        match Named::parse(input) {
            Ok((value, input)) => return Ok((Self::Named(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Reserved::parse(input) {
            Ok((value, input)) => return Ok((Self::Reserved(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Access::parse(input) {
            Ok((value, input)) => return Ok((Self::Access(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match ExtendedAccess::parse(input) {
            Ok((value, input)) => return Ok((Self::ExtendedAccess(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        let (value, input) = Connect::parse(input, alloc)?;
        Ok((Self::Connect(value), input))
    }
}
