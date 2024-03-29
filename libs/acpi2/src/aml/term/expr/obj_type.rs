use super::{deref_of::DerefOf, index::Index, ref_of::RefOf};
use crate::aml::{
    context::Context,
    misc::DebugObj,
    name::SimpleName,
    ops::ObjTypeOp,
    parser::{fail, Input, ParseResult},
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
        if let Ok((value, input)) = SimpleName::parse(input, alloc.clone()) {
            return Ok((Self::SimpleName(value), input));
        }

        if let Ok((value, input)) = DebugObj::parse(input) {
            return Ok((Self::DebugObj(value), input));
        }

        if let Ok((value, input)) = RefOf::parse(input, context, alloc.clone()) {
            return Ok((Self::RefOf(value), input));
        }

        if let Ok((value, input)) = DerefOf::parse(input, context, alloc.clone()) {
            return Ok((Self::DerefOf(value), input));
        }

        let (value, input) = Index::parse(input, context, alloc)?;
        Ok((Self::Index(value), input))
    }
}

// impl<A: Allocator> core::fmt::Debug for ObjType<A> {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         match self {
//             Self::SimpleName(inner) => f.debug_tuple("SimpleName").field(inner).finish(),
//             Self::DebugObj(inner) => f.debug_tuple("DebugObj").field(inner).finish(),
//             Self::RefOf(inner) => f.debug_tuple("RefOf").field(inner).finish(),
//             Self::DerefOf(inner) => f.debug_tuple("DerefOf").field(inner).finish(),
//             Self::Index(inner) => f.debug_tuple("Index").field(inner).finish(),
//         }
//     }
// }
