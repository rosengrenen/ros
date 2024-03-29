use core::alloc::Allocator;

use alloc::vec::Vec;

use crate::aml::{
    context::Context,
    data::DataRefObj,
    name::NameString,
    ops::{AliasOp, NameOp, ScopeOp},
    parser::{fail, fail_if_not_empty, Input, ParseResult},
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
        if let Ok((value, input)) = Alias::parse(input, alloc.clone()) {
            return Ok((Self::Alias(value), input));
        }

        if let Ok((value, input)) = Name::parse(input, context, alloc.clone()) {
            return Ok((Self::Name(value), input));
        }

        let (value, input) = Scope::parse(input, context, alloc)?;
        Ok((Self::Scope(value), input))
    }
}

pub struct Alias<A: Allocator> {
    source: NameString<A>,
    alias: NameString<A>,
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

pub struct Name<A: Allocator> {
    name: NameString<A>,
    data: DataRefObj<A>,
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

pub struct Scope<A: Allocator> {
    name: NameString<A>,
    terms: Vec<TermObj<A>, A>,
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
