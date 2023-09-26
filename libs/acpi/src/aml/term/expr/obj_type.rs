use super::{deref_of::DerefOf, index::Index, ref_of::RefOf};
use crate::aml::{misc::DebugObj, name::SimpleName, ops::ObjTypeOp, prefixed::prefixed, Context};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub enum ObjType<A: Allocator> {
    SimpleName(SimpleName<A>),
    DebugObj(DebugObj),
    RefOf(RefOf<A>),
    DerefOf(DerefOf<A>),
    Index(Index<A>),
}

impl<A: Allocator + Clone> ObjType<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(
            ObjTypeOp::p,
            (
                SimpleName::p.map(Self::SimpleName),
                DebugObj::p.map(Self::DebugObj),
                RefOf::p.map(Self::RefOf),
                DerefOf::p.map(Self::DerefOf),
                Index::p.map(Self::Index),
            )
                .alt(),
        )
        .add_context("ObjType")
        .parse(input, context, alloc)
    }
}
