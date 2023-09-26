use crate::aml::{
    aml::{name::NameString, term::term_arg::TermArg},
    ops::{
        CreateBitFieldOp, CreateByteFieldOp, CreateDWordFieldOp, CreateFieldOp, CreateQWordFieldOp,
        CreateWordFieldOp,
    },
    prefixed, Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub enum CreateConstField<A: Allocator> {
    CreateBitField(CreateBitField<A>),
    CreateByteField(CreateByteField<A>),
    CreateWordField(CreateWordField<A>),
    CreateDWordField(CreateDWordField<A>),
    CreateQWordField(CreateQWordField<A>),
}

impl<A: Allocator + Clone> CreateConstField<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            CreateBitField::p.map(Self::CreateBitField),
            CreateByteField::p.map(Self::CreateByteField),
            CreateWordField::p.map(Self::CreateWordField),
            CreateDWordField::p.map(Self::CreateDWordField),
            CreateQWordField::p.map(Self::CreateQWordField),
        )
            .alt()
            .add_context("CreateConstField")
            .parse(input, context, alloc)
    }
}

macro_rules! create_const_field {
    ($name:ident, $op:ident) => {
        #[derive(Debug)]
        pub struct $name<A: Allocator> {
            pub source: TermArg<A>,
            pub index: TermArg<A>,
            pub name: NameString<A>,
        }

        impl<A: Allocator + Clone> $name<A> {
            pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
                input: I,
                context: &mut Context,
                alloc: A,
            ) -> ParseResult<I, Self, E> {
                prefixed($op::p, (TermArg::p, TermArg::p, NameString::p))
                    .map(|(source, index, name)| Self {
                        source,
                        index,
                        name,
                    })
                    .add_context(stringify!($name))
                    .parse(input, context, alloc)
            }
        }
    };
}

create_const_field!(CreateBitField, CreateBitFieldOp);
create_const_field!(CreateByteField, CreateByteFieldOp);
create_const_field!(CreateWordField, CreateWordFieldOp);
create_const_field!(CreateDWordField, CreateDWordFieldOp);
create_const_field!(CreateQWordField, CreateQWordFieldOp);

#[derive(Debug)]
pub struct CreateField<A: Allocator> {
    pub source_buf: TermArg<A>,
    pub bit_index: TermArg<A>,
    pub num_bits: TermArg<A>,
    pub name: NameString<A>,
}

impl<A: Allocator + Clone> CreateField<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let source_buff = TermArg::p; // => Buffer
        let bit_index = TermArg::p; // => Integer
        let num_bits = TermArg::p; // => Integer
        prefixed(
            CreateFieldOp::p,
            (source_buff, bit_index, num_bits, NameString::p),
        )
        .map(|(source_buf, bit_index, num_bits, name)| Self {
            source_buf,
            bit_index,
            num_bits,
            name,
        })
        .add_context("CreateField")
        .parse(input, context, alloc)
    }
}
