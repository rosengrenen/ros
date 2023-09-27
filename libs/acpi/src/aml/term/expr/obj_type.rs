use super::{deref_of::DerefOf, index::Index, ref_of::RefOf};
use crate::aml::{misc::DebugObj, name::SimpleName, ops::ObjTypeOp, prefixed::prefixed, Context};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

pub enum ObjType<A: Allocator> {
    SimpleName(SimpleName<A>),
    DebugObj(DebugObj),
    RefOf(RefOf<A>),
    DerefOf(DerefOf<A>),
    Index(Index<A>),
}

impl<A: Allocator> core::fmt::Debug for ObjType<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SimpleName(inner) => f.debug_tuple("SimpleName").field(inner).finish(),
            Self::DebugObj(inner) => f.debug_tuple("DebugObj").field(inner).finish(),
            Self::RefOf(inner) => f.debug_tuple("RefOf").field(inner).finish(),
            Self::DerefOf(inner) => f.debug_tuple("DerefOf").field(inner).finish(),
            Self::Index(inner) => f.debug_tuple("Index").field(inner).finish(),
        }
    }
}

impl<A: Allocator + Clone> ObjType<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context<A>,
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
