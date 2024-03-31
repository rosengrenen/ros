use super::{deref_of::DerefOf, index::Index, ref_of::RefOf};
use crate::aml::{
    context::Context,
    misc::DebugObj,
    name::SimpleName,
    ops::ObjTypeOp,
    parser::{fail, Input, ParseResult, ParserError},
};
use core::alloc::Allocator;

pub enum ObjType<A: Allocator> {
    SimpleName(SimpleName<A>),
    DebugObj(DebugObj),
    RefOf(RefOf<A>),
    DerefOf(DerefOf<A>),
    Index(Index<A>),
}

impl<A: Allocator + Clone> ObjType<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = ObjTypeOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        match SimpleName::parse(input, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::SimpleName(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match DebugObj::parse(input) {
            Ok((value, input)) => return Ok((Self::DebugObj(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match RefOf::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::RefOf(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match DerefOf::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::DerefOf(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        let (value, input) = Index::parse(input, context, alloc)?;
        Ok((Self::Index(value), input))
    }
}

impl<A: Allocator> core::fmt::Debug for ObjType<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SimpleName(arg0) => f.debug_tuple("SimpleName").field(arg0).finish(),
            Self::DebugObj(arg0) => f.debug_tuple("DebugObj").field(arg0).finish(),
            Self::RefOf(arg0) => f.debug_tuple("RefOf").field(arg0).finish(),
            Self::DerefOf(arg0) => f.debug_tuple("DerefOf").field(arg0).finish(),
            Self::Index(arg0) => f.debug_tuple("Index").field(arg0).finish(),
        }
    }
}
