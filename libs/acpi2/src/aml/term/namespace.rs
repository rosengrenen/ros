use core::alloc::Allocator;

use alloc::vec::Vec;

use crate::aml::{
    context::Context,
    data::DataRefObj,
    name::NameString,
    ops::{AliasOp, NameOp, ScopeOp},
    parser::{fail, fail_if_not_empty, Input, ParseResult, ParserError},
    pkg_len::pkg,
};

use super::TermObj;

pub enum NameSpaceModObj<A: Allocator> {
    Alias(Alias<A>),
    Name(Name<A>),
    Scope(Scope<A>),
}

impl<A: Allocator + Clone> NameSpaceModObj<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        match Alias::parse(input, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Alias(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Name::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Name(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        let (value, input) = Scope::parse(input, context, alloc)?;
        Ok((Self::Scope(value), input))
    }
}

impl<A: Allocator> core::fmt::Debug for NameSpaceModObj<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Alias(arg0) => f.debug_tuple("Alias").field(arg0).finish(),
            Self::Name(arg0) => f.debug_tuple("Name").field(arg0).finish(),
            Self::Scope(arg0) => f.debug_tuple("Scope").field(arg0).finish(),
        }
    }
}

pub struct Alias<A: Allocator> {
    pub source: NameString<A>,
    pub alias: NameString<A>,
}

impl<A: Allocator + Clone> Alias<A> {
    pub fn parse<'a>(input: Input<'a>, alloc: A) -> ParseResult<'a, Self> {
        let (_, input) = AliasOp::parse(input)?;
        fail(Self::parse_inner(input, alloc))
    }

    fn parse_inner<'a>(input: Input<'a>, alloc: A) -> ParseResult<'a, Self> {
        let (source, input) = NameString::parse(input, alloc.clone())?;
        let (alias, input) = NameString::parse(input, alloc)?;
        Ok((Self { source, alias }, input))
    }
}

impl<A: Allocator> core::fmt::Debug for Alias<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Alias")
            .field("source", &self.source)
            .field("alias", &self.alias)
            .finish()
    }
}

pub struct Name<A: Allocator> {
    pub name: NameString<A>,
    pub data: DataRefObj<A>,
}

impl<A: Allocator + Clone> Name<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = NameOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (name, input) = NameString::parse(input, alloc.clone())?;
        let (data, input) = DataRefObj::parse(input, context, alloc)?;
        Ok((Self { name, data }, input))
    }
}

impl<A: Allocator> core::fmt::Debug for Name<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Name")
            .field("name", &self.name)
            .field("data", &self.data)
            .finish()
    }
}

pub struct Scope<A: Allocator> {
    pub name: NameString<A>,
    pub terms: Vec<TermObj<A>, A>,
}

impl<A: Allocator + Clone> Scope<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = ScopeOp::parse(input)?;
        let (pkg_input, input) = pkg(input)?;
        let (value, pkg_input) = fail(Self::parse_inner(pkg_input, context, alloc))?;
        fail_if_not_empty(pkg_input)?;
        Ok((value, input))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (name, mut input) = NameString::parse(input, alloc.clone())?;
        context.push_scope(&name);
        let mut terms = Vec::new(alloc.clone());
        while let Ok((term, i)) = TermObj::parse(input, context, alloc.clone()) {
            terms.push(term).unwrap();
            input = i;
        }

        context.pop_scope();
        Ok((Self { name, terms }, input))
    }
}

impl<A: Allocator> core::fmt::Debug for Scope<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Scope")
            .field("name", &self.name)
            .field("terms", &self.terms)
            .finish()
    }
}
