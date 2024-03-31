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

impl<A: Allocator> core::fmt::Debug for CreateConstField<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Bit(arg0) => f.debug_tuple("Bit").field(arg0).finish(),
            Self::Byte(arg0) => f.debug_tuple("Byte").field(arg0).finish(),
            Self::Word(arg0) => f.debug_tuple("Word").field(arg0).finish(),
            Self::DWord(arg0) => f.debug_tuple("DWord").field(arg0).finish(),
            Self::QWord(arg0) => f.debug_tuple("QWord").field(arg0).finish(),
        }
    }
}

pub struct Bit<A: Allocator> {
    pub source: TermArg<A>,
    pub index: TermArg<A>,
    pub name: NameString<A>,
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

impl<A: Allocator> core::fmt::Debug for Bit<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Bit")
            .field("source", &self.source)
            .field("index", &self.index)
            .field("name", &self.name)
            .finish()
    }
}

pub struct Byte<A: Allocator> {
    pub source: TermArg<A>,
    pub index: TermArg<A>,
    pub name: NameString<A>,
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

impl<A: Allocator> core::fmt::Debug for Byte<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Byte")
            .field("source", &self.source)
            .field("index", &self.index)
            .field("name", &self.name)
            .finish()
    }
}

pub struct Word<A: Allocator> {
    pub source: TermArg<A>,
    pub index: TermArg<A>,
    pub name: NameString<A>,
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

impl<A: Allocator> core::fmt::Debug for Word<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Word")
            .field("source", &self.source)
            .field("index", &self.index)
            .field("name", &self.name)
            .finish()
    }
}

pub struct DWord<A: Allocator> {
    pub source: TermArg<A>,
    pub index: TermArg<A>,
    pub name: NameString<A>,
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

impl<A: Allocator> core::fmt::Debug for DWord<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DWord")
            .field("source", &self.source)
            .field("index", &self.index)
            .field("name", &self.name)
            .finish()
    }
}

pub struct QWord<A: Allocator> {
    pub source: TermArg<A>,
    pub index: TermArg<A>,
    pub name: NameString<A>,
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

impl<A: Allocator> core::fmt::Debug for QWord<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("QWord")
            .field("source", &self.source)
            .field("index", &self.index)
            .field("name", &self.name)
            .finish()
    }
}

pub struct CreateField<A: Allocator> {
    pub source_buf: TermArg<A>,
    pub bit_index: TermArg<A>,
    pub num_bits: TermArg<A>,
    pub name: NameString<A>,
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

impl<A: Allocator> core::fmt::Debug for CreateField<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CreateField")
            .field("source_buf", &self.source_buf)
            .field("bit_index", &self.bit_index)
            .field("num_bits", &self.num_bits)
            .field("name", &self.name)
            .finish()
    }
}
