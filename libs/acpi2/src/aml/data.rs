use core::alloc::Allocator;

use super::{
    context::Context,
    ops::{
        BytePrefix, DWordPrefix, OneOp, OnesOp, QWordPrefix, RevisionOp, StringPrefix, WordPrefix,
        ZeroOp,
    },
    parser::{fail, item, satisfy, take_one, Input, ParseResult},
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
        if let Ok((value, input)) = ConstInteger::parse(input) {
            return Ok((Self::ConstInteger(value), input));
        }

        if let Ok((value, input)) = String::parse(input, alloc.clone()) {
            return Ok((Self::String(value), input));
        }

        if let Ok((value, input)) = ConstObj::parse(input) {
            return Ok((Self::ConstObj(value), input));
        }

        if let Ok((value, input)) = RevisionOp::parse(input) {
            return Ok((Self::RevisionOp(value), input));
        }

        let (value, input) = Buffer::parse(input, context, alloc)?;
        Ok((Self::Buffer(value), input))
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
        if let Ok((computational_data, input)) =
            ComputationalData::parse(input, context, alloc.clone())
        {
            return Ok((Self::ComputationalData(computational_data), input));
        }

        if let Ok((pkg, input)) = Pkg::parse(input, context, alloc.clone()) {
            return Ok((Self::Pkg(pkg), input));
        }

        let (var_pkg, input) = VarPkg::parse(input, context, alloc)?;
        Ok((Self::VarPkg(var_pkg), input))
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

pub enum ConstInteger {
    ByteConst(ByteConst),
    WordConst(WordConst),
    DWordConst(DWordConst),
    QWordConst(QWordConst),
}

impl ConstInteger {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        if let Ok((byte_const, input)) = ByteConst::parse(input) {
            return Ok((Self::ByteConst(byte_const), input));
        }

        if let Ok((word_const, input)) = WordConst::parse(input) {
            return Ok((Self::WordConst(word_const), input));
        }

        if let Ok((dword_const, input)) = DWordConst::parse(input) {
            return Ok((Self::DWordConst(dword_const), input));
        }

        let (qword_const, input) = QWordConst::parse(input)?;
        Ok((Self::QWordConst(qword_const), input))
    }
}

pub struct ByteConst(u8);

impl ByteConst {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (_, input) = BytePrefix::parse(input)?;
        let (byte, input) = fail(byte_data(input))?;
        Ok((Self(byte), input))
    }
}

pub struct WordConst(u16);
impl WordConst {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (_, input) = WordPrefix::parse(input)?;
        let (word, input) = fail(word_data(input))?;
        Ok((Self(word), input))
    }
}

pub struct DWordConst(u32);

impl DWordConst {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (_, input) = DWordPrefix::parse(input)?;
        let (dword, input) = fail(dword_data(input))?;
        Ok((Self(dword), input))
    }
}

pub struct QWordConst(u64);

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

pub struct String<A: Allocator>(Vec<u8, A>);

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

pub enum ConstObj {
    ZeroOp(ZeroOp),
    OneOp(OneOp),
    OnesOp(OnesOp),
}

impl ConstObj {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        if let Ok((op, input)) = ZeroOp::parse(input) {
            return Ok((Self::ZeroOp(op), input));
        }

        if let Ok((op, input)) = OneOp::parse(input) {
            return Ok((Self::OneOp(op), input));
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
