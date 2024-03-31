use core::alloc::Allocator;

use super::{
    context::Context,
    ops::{
        BytePrefix, DWordPrefix, OneOp, OnesOp, QWordPrefix, RevisionOp, StringPrefix, WordPrefix,
        ZeroOp,
    },
    parser::{fail, item, satisfy, take_one, Input, ParseResult, ParserError},
    term::expr::{buffer::Buffer, pkg::Pkg, var_pkg::VarPkg},
};
use alloc::vec::Vec;

pub enum ComputationalData<A: Allocator> {
    ConstInteger(ConstInteger),
    String(String<A>),
    ConstObj(ConstObj),
    RevisionOp(RevisionOp),
    Buffer(Buffer<A>),
}

impl<A: Allocator + Clone> ComputationalData<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        match ConstInteger::parse(input) {
            Ok((value, input)) => return Ok((Self::ConstInteger(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match String::parse(input, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::String(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match ConstObj::parse(input) {
            Ok((value, input)) => return Ok((Self::ConstObj(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match RevisionOp::parse(input) {
            Ok((value, input)) => return Ok((Self::RevisionOp(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        let (value, input) = Buffer::parse(input, context, alloc)?;
        Ok((Self::Buffer(value), input))
    }
}

impl<A: Allocator> core::fmt::Debug for ComputationalData<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ConstInteger(arg0) => f.debug_tuple("ConstInteger").field(arg0).finish(),
            Self::String(arg0) => f.debug_tuple("String").field(arg0).finish(),
            Self::ConstObj(arg0) => f.debug_tuple("ConstObj").field(arg0).finish(),
            Self::RevisionOp(arg0) => f.debug_tuple("RevisionOp").field(arg0).finish(),
            Self::Buffer(arg0) => f.debug_tuple("Buffer").field(arg0).finish(),
        }
    }
}

pub enum DataObj<A: Allocator> {
    ComputationalData(ComputationalData<A>),
    Pkg(Pkg<A>),
    VarPkg(VarPkg<A>),
}

impl<A: Allocator + Clone> DataObj<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        match ComputationalData::parse(input, context, alloc.clone()) {
            Ok((computational_data, input)) => {
                return Ok((Self::ComputationalData(computational_data), input))
            }
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Pkg::parse(input, context, alloc.clone()) {
            Ok((pkg, input)) => return Ok((Self::Pkg(pkg), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        let (var_pkg, input) = VarPkg::parse(input, context, alloc)?;
        Ok((Self::VarPkg(var_pkg), input))
    }
}

impl<A: Allocator> core::fmt::Debug for DataObj<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ComputationalData(arg0) => {
                f.debug_tuple("ComputationalData").field(arg0).finish()
            }
            Self::Pkg(arg0) => f.debug_tuple("Pkg").field(arg0).finish(),
            Self::VarPkg(arg0) => f.debug_tuple("VarPkg").field(arg0).finish(),
        }
    }
}

pub enum DataRefObj<A: Allocator> {
    DataObj(DataObj<A>),
    // ObjRef(ObjRef),
}

impl<A: Allocator + Clone> DataRefObj<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (data_obj, input) = DataObj::parse(input, context, alloc)?;
        Ok((Self::DataObj(data_obj), input))
    }
}

impl<A: Allocator> core::fmt::Debug for DataRefObj<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DataObj(arg0) => f.debug_tuple("DataObj").field(arg0).finish(),
        }
    }
}

#[derive(Debug)]
pub enum ConstInteger {
    ByteConst(ByteConst),
    WordConst(WordConst),
    DWordConst(DWordConst),
    QWordConst(QWordConst),
}

impl ConstInteger {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        match ByteConst::parse(input) {
            Ok((byte_const, input)) => return Ok((Self::ByteConst(byte_const), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match WordConst::parse(input) {
            Ok((word_const, input)) => return Ok((Self::WordConst(word_const), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match DWordConst::parse(input) {
            Ok((dword_const, input)) => return Ok((Self::DWordConst(dword_const), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        let (qword_const, input) = QWordConst::parse(input)?;
        Ok((Self::QWordConst(qword_const), input))
    }
}

#[derive(Debug)]
pub struct ByteConst(pub u8);

impl ByteConst {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (_, input) = BytePrefix::parse(input)?;
        let (byte, input) = fail(byte_data(input))?;
        Ok((Self(byte), input))
    }
}

#[derive(Debug)]
pub struct WordConst(pub u16);

impl WordConst {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (_, input) = WordPrefix::parse(input)?;
        let (word, input) = fail(word_data(input))?;
        Ok((Self(word), input))
    }
}

#[derive(Debug)]
pub struct DWordConst(pub u32);

impl DWordConst {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (_, input) = DWordPrefix::parse(input)?;
        let (dword, input) = fail(dword_data(input))?;
        Ok((Self(dword), input))
    }
}

#[derive(Debug)]
pub struct QWordConst(pub u64);

impl QWordConst {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (_, input) = QWordPrefix::parse(input)?;
        let (qword, input) = fail(qword_data(input))?;
        Ok((Self(qword), input))
    }
}

pub fn byte_data<'a>(input: Input<'a>) -> ParseResult<'a, u8> {
    take_one(input)
}

pub fn word_data<'a>(input: Input<'a>) -> ParseResult<'a, u16> {
    let (lower, input) = byte_data(input)?;
    let (higher, input) = byte_data(input)?;
    Ok(((higher as u16) << 8 | lower as u16, input))
}

pub fn dword_data<'a>(input: Input<'a>) -> ParseResult<'a, u32> {
    let (lower, input) = word_data(input)?;
    let (higher, input) = word_data(input)?;
    Ok(((higher as u32) << 16 | lower as u32, input))
}

pub fn qword_data<'a>(input: Input<'a>) -> ParseResult<'a, u64> {
    let (lower, input) = dword_data(input)?;
    let (higher, input) = dword_data(input)?;
    Ok(((higher as u64) << 32 | lower as u64, input))
}

pub struct String<A: Allocator>(pub Vec<u8, A>);

impl<A: Allocator> String<A> {
    pub fn parse<'a>(input: Input<'a>, alloc: A) -> ParseResult<'a, Self> {
        let (_, mut input) = StringPrefix::parse(input)?;
        let mut chars = Vec::new(alloc);
        while let Ok((c, i)) = ascii_char(input) {
            chars.push(c).expect("failed to push char to string array");
            input = i;
        }

        let (_, input) = fail(null_char(input))?;
        Ok((Self(chars), input))
    }
}

impl<A: Allocator> core::fmt::Debug for String<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("String").field(&self.0).finish()
    }
}

#[derive(Debug)]
pub enum ConstObj {
    ZeroOp(ZeroOp),
    OneOp(OneOp),
    OnesOp(OnesOp),
}

impl ConstObj {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        match ZeroOp::parse(input) {
            Ok((op, input)) => return Ok((Self::ZeroOp(op), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match OneOp::parse(input) {
            Ok((op, input)) => return Ok((Self::OneOp(op), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        let (op, input) = OnesOp::parse(input)?;
        Ok((Self::OnesOp(op), input))
    }
}

pub fn ascii_char<'a>(input: Input<'a>) -> ParseResult<'a, u8> {
    satisfy(input, |b| (0x01..=0x7f).contains(&b))
}

pub fn null_char<'a>(input: Input<'a>) -> ParseResult<'a, ()> {
    let (_, input) = item(input, 0x00)?;
    Ok(((), input))
}
