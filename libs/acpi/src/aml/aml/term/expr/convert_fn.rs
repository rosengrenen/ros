use crate::aml::{
    aml::{name::Target, term::TermArg},
    ops::{
        FromBcdOp, ToBcdOp, ToBufferOp, ToDecimalStringOp, ToHexStringOp, ToIntegerOp, ToStringOp,
    },
    prefixed, Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

pub enum ConvertFn<A: Allocator> {
    FromBcd(FromBcd<A>),
    ToBcd(ToBcd<A>),
    ToBuffer(ToBuffer<A>),
    ToDecimalString(ToDecimalString<A>),
    ToHexString(ToHexString<A>),
    ToInteger(ToInteger<A>),
    ToString(ToString<A>),
}

impl<A: Allocator + Clone> ConvertFn<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            FromBcd::p.map(Self::FromBcd),
            ToBcd::p.map(Self::ToBcd),
            ToBuffer::p.map(Self::ToBuffer),
            ToDecimalString::p.map(Self::ToDecimalString),
            ToHexString::p.map(Self::ToHexString),
            ToInteger::p.map(Self::ToInteger),
            ToString::p.map(Self::ToString),
        )
            .alt()
            .add_context("ConvertFn")
            .parse(input, context, alloc)
    }
}

macro_rules! convert_fn_expr {
    ($name:ident, $op:ident) => {
        pub struct $name<A: Allocator> {
            pub input: TermArg<A>,
            pub target: Target<A>,
        }

        impl<A: Allocator + Clone> $name<A> {
            pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
                input: I,
                context: &mut Context,
                alloc: A,
            ) -> ParseResult<I, Self, E> {
                prefixed($op::p, (TermArg::p, Target::p))
                    .map(|(input, target)| Self { input, target })
                    .add_context(stringify!($name))
                    .parse(input, context, alloc)
            }
        }
    };
}

convert_fn_expr!(FromBcd, FromBcdOp);
convert_fn_expr!(ToBcd, ToBcdOp);
convert_fn_expr!(ToBuffer, ToBufferOp);
convert_fn_expr!(ToDecimalString, ToDecimalStringOp);
convert_fn_expr!(ToHexString, ToHexStringOp);
convert_fn_expr!(ToInteger, ToIntegerOp);

pub struct ToString<A: Allocator> {
    pub arg1: TermArg<A>,
    pub length_arg: TermArg<A>,
    pub target: Target<A>,
}

impl<A: Allocator + Clone> ToString<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let length_arg = TermArg::p; // => Integer
        prefixed(ToStringOp::p, (TermArg::p, length_arg, Target::p))
            .map(|(arg1, length_arg, target)| Self {
                arg1,
                length_arg,
                target,
            })
            .add_context("ToString")
            .parse(input, context, alloc)
    }
}
