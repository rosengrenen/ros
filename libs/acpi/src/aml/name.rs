use super::{
    misc::{ArgObj, DebugObj, LocalObj},
    ops::{DualNamePrefix, MultiNamePrefix, RootChar},
    prefixed::prefixed,
    term::expr::RefTypeOpcode,
    Context,
};
use alloc::{boxed::Box, vec::Vec};
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    multi::many::many_n,
    parser::Parser,
    primitive::{
        item::{item, satisfy, take_one},
        take::take_while,
    },
};
use std::{
    alloc::Allocator,
    fmt::{Debug, Display},
};

parser_fn!(
    fn lead_name_char() -> u8 {
        satisfy(|&b: &u8| b == b'_' || b.is_ascii_uppercase())
    }
);

parser_fn!(
    fn digit_char() -> u8 {
        satisfy(|&b: &u8| b.is_ascii_digit())
    }
);

parser_fn!(
    fn name_char() -> u8 {
        (digit_char, lead_name_char).alt()
    }
);

parser_struct_wrapper!(
    struct NameSeg([u8; 4]);,
    (lead_name_char, name_char, name_char, name_char).map(|(lead, c1, c2, c3)| [lead, c1, c2, c3])
);

impl Display for NameSeg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", unsafe { &core::str::from_utf8_unchecked(&self.0) })
    }
}

#[derive(Debug)]
pub enum NameString<A: Allocator> {
    Absolute(NamePath<A>),
    Relative(usize, NamePath<A>),
}

impl<A: Allocator + Clone> NameString<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            prefixed(RootChar::p, NamePath::p).map(Self::Absolute),
            (PrefixPath::p, NamePath::p.cut())
                .map(|(prefix_path, name_path)| Self::Relative(prefix_path.0, name_path)),
        )
            .alt()
            .add_context("NameString")
            .parse(input, context, alloc)
    }
}

parser_struct_wrapper!(
    struct PrefixPath(usize);,
    take_while::<_, E, _, _, _>(|b| b == 0x5e).map(|value| value.input_len())
);

parser_enum_alloc!(
    enum NamePath {
        NameSeg(NameSeg),
        DualNamePath(DualNamePath),
        MultiNamePath(MultiNamePath<A>),
        NullName(NullName),
    }
);

parser_struct!(
    struct DualNamePath {
        first: NameSeg,
        second: NameSeg,
    },
    prefixed(DualNamePrefix::p, (NameSeg::p, NameSeg::p),)
);

#[derive(Debug)]
pub struct MultiNamePath<A: Allocator>(Vec<NameSeg, A>);

impl<A: Allocator + Clone> MultiNamePath<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let (input, seg_count) =
            prefixed(MultiNamePrefix::p, take_one()).parse(input, context, alloc.clone())?;
        many_n(seg_count as usize, NameSeg::p)
            .cut()
            .map(MultiNamePath)
            .add_context("MultiNamePath")
            .parse(input, context, alloc)
    }
}

parser_enum_alloc!(
    enum SimpleName {
        NameString(NameString<A>),
        ArgObj(ArgObj),
        LocalObj(LocalObj),
    }
);

#[derive(Debug)]
pub enum SuperName<A: Allocator> {
    SimpleName(SimpleName<A>),
    DebugObj(DebugObj),
    RefTypeOpcode(Box<RefTypeOpcode<A>, A>),
}

impl<A: Allocator + Clone> SuperName<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            SimpleName::p.map(Self::SimpleName),
            DebugObj::p.map(Self::DebugObj),
            RefTypeOpcode::p.boxed().map(Self::RefTypeOpcode),
        )
            .alt()
            .add_context("SuperName")
            .parse(input, context, alloc)
    }
}

parser_struct_empty!(struct NullName;, item(0x00));

parser_enum_alloc!(
    enum Target {
        SuperName(SuperName<A>),
        NullName(NullName),
    }
);
