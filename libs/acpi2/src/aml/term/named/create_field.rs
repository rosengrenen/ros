use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    name::NameString,
    ops::{
        CreateBitFieldOp, CreateByteFieldOp, CreateDWordFieldOp, CreateFieldOp, CreateQWordFieldOp,
        CreateWordFieldOp,
    },
    parser::{fail, Input, ParseResult, ParserError},
    term::TermArg,
};

pub enum CreateConstField<A: Allocator> {
    Bit(Bit<A>),
    Byte(Byte<A>),
    Word(Word<A>),
    DWord(DWord<A>),
    QWord(QWord<A>),
}

impl<A: Allocator + Clone> CreateConstField<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        match Bit::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Bit(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Byte::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Byte(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Word::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Word(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match DWord::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::DWord(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        let (value, input) = QWord::parse(input, context, alloc)?;
        Ok((Self::QWord(value), input))
    }
}

struct Bit<A: Allocator> {
    source: TermArg<A>,
    index: TermArg<A>,
    name: NameString<A>,
}

impl<A: Allocator + Clone> Bit<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = CreateBitFieldOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (source, input) = TermArg::parse(input, context, alloc.clone())?;
        let (index, input) = TermArg::parse(input, context, alloc.clone())?;
        let (name, input) = NameString::parse(input, alloc)?;
        let this = Self {
            source,
            index,
            name,
        };
        Ok((this, input))
    }
}

struct Byte<A: Allocator> {
    source: TermArg<A>,
    index: TermArg<A>,
    name: NameString<A>,
}

impl<A: Allocator + Clone> Byte<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = CreateByteFieldOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (source, input) = TermArg::parse(input, context, alloc.clone())?;
        let (index, input) = TermArg::parse(input, context, alloc.clone())?;
        let (name, input) = NameString::parse(input, alloc)?;
        let this = Self {
            source,
            index,
            name,
        };
        Ok((this, input))
    }
}

struct Word<A: Allocator> {
    source: TermArg<A>,
    index: TermArg<A>,
    name: NameString<A>,
}

impl<A: Allocator + Clone> Word<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = CreateWordFieldOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (source, input) = TermArg::parse(input, context, alloc.clone())?;
        let (index, input) = TermArg::parse(input, context, alloc.clone())?;
        let (name, input) = NameString::parse(input, alloc)?;
        let this = Self {
            source,
            index,
            name,
        };
        Ok((this, input))
    }
}

struct DWord<A: Allocator> {
    source: TermArg<A>,
    index: TermArg<A>,
    name: NameString<A>,
}

impl<A: Allocator + Clone> DWord<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = CreateDWordFieldOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (source, input) = TermArg::parse(input, context, alloc.clone())?;
        let (index, input) = TermArg::parse(input, context, alloc.clone())?;
        let (name, input) = NameString::parse(input, alloc)?;
        let this = Self {
            source,
            index,
            name,
        };
        Ok((this, input))
    }
}

struct QWord<A: Allocator> {
    source: TermArg<A>,
    index: TermArg<A>,
    name: NameString<A>,
}

impl<A: Allocator + Clone> QWord<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = CreateQWordFieldOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (source, input) = TermArg::parse(input, context, alloc.clone())?;
        let (index, input) = TermArg::parse(input, context, alloc.clone())?;
        let (name, input) = NameString::parse(input, alloc)?;
        let this = Self {
            source,
            index,
            name,
        };
        Ok((this, input))
    }
}

pub struct CreateField<A: Allocator> {
    source_buf: TermArg<A>,
    bit_index: TermArg<A>,
    num_bits: TermArg<A>,
    name: NameString<A>,
}

impl<A: Allocator + Clone> CreateField<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = CreateFieldOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (source_buf, input) = TermArg::parse(input, context, alloc.clone())?;
        let (bit_index, input) = TermArg::parse(input, context, alloc.clone())?;
        let (num_bits, input) = TermArg::parse(input, context, alloc.clone())?;
        let (name, input) = NameString::parse(input, alloc)?;
        let this = Self {
            source_buf,
            bit_index,
            num_bits,
            name,
        };
        Ok((this, input))
    }
}
