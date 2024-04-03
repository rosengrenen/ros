use alloc::boxed::Box;
use core::alloc::Allocator;

use crate::aml::data::byte_data;
use crate::aml::name::NameSeg;
use crate::aml::name::NameString;
use crate::aml::parser::fail;
use crate::aml::parser::item;
use crate::aml::parser::Input;
use crate::aml::parser::ParseResult;
use crate::aml::parser::ParserError;
use crate::aml::pkg_len::pkg_length;

#[derive(Debug)]
pub struct Named {
    pub name: NameSeg,
    pub len: usize,
}

impl Named {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (name, input) = NameSeg::parse(input)?;
        let (len, input) = pkg_length(input)?;
        Ok((Self { name, len }, input))
    }
}

#[derive(Debug)]
pub struct Reserved;

impl Reserved {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (_, input) = item(input, 0x00)?;
        let (_, input) = pkg_length(input)?;
        Ok((Self, input))
    }
}

#[derive(Debug)]
pub struct Access {
    pub ty: u8,
    pub attrib: u8,
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

impl<A: Allocator> core::fmt::Debug for Connect<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NameString(inner) => f.debug_tuple("NameString").field(inner).finish(),
        }
    }
}

#[derive(Debug)]
pub struct ExtendedAccess {
    pub ty: u8,
    pub attrib: u8,
    pub len: u8,
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
    Connect(Box<Connect<A>, A>),
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

        let (value, input) = Connect::parse(input, alloc.clone())?;
        Ok((Self::Connect(Box::new(value, alloc).unwrap()), input))
    }
}

impl<A: Allocator> core::fmt::Debug for FieldElement<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Named(arg0) => f.debug_tuple("Named").field(arg0).finish(),
            Self::Reserved(arg0) => f.debug_tuple("Reserved").field(arg0).finish(),
            Self::Access(arg0) => f.debug_tuple("Access").field(arg0).finish(),
            Self::ExtendedAccess(arg0) => f.debug_tuple("ExtendedAccess").field(arg0).finish(),
            Self::Connect(arg0) => f.debug_tuple("Connect").field(arg0).finish(),
        }
    }
}
