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
        take::take_while1,
    },
};
use std::fmt::Formatter;
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

pub struct NameSeg([u8; 4]);

impl core::fmt::Debug for NameSeg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("NameSeg")
            .field(unsafe { &core::str::from_utf8_unchecked(&self.0) })
            .finish()
    }
}

impl NameSeg {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (lead_name_char, name_char, name_char, name_char)
            .map(|(lead, c1, c2, c3)| Self([lead, c1, c2, c3]))
            .add_context("NameSeg")
            .parse(input, context, alloc)
    }
}

// parser_struct_wrapper!(
//     struct NameSeg([u8; 4]);,
//     (lead_name_char, name_char, name_char, name_char).map(|(lead, c1, c2, c3)| [lead, c1, c2, c3])
// );

impl Display for NameSeg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", unsafe { &core::str::from_utf8_unchecked(&self.0) })
    }
}

pub enum NameString<A: Allocator> {
    Absolute(NamePath<A>),
    Relative(usize, NamePath<A>),
}

impl<A: Allocator> core::fmt::Debug for NameString<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Absolute(name) => f.debug_tuple("Absolute").field(&name).finish(),
            Self::Relative(count, name) => {
                f.debug_tuple("Relative").field(count).field(&name).finish()
            }
        }
    }
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
            (NamePath::p).map(|name_path| Self::Relative(0, name_path)),
        )
            .alt()
            .add_context("NameString")
            .parse(input, context, alloc)
    }
}

parser_struct_wrapper!(
    struct PrefixPath(usize);,
    take_while1::<_, E, _, _, _>(|b| b == 0x5e).map(|value| value.input_len())
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

pub struct MultiNamePath<A: Allocator>(Vec<NameSeg, A>);

impl<A: Allocator> core::fmt::Debug for MultiNamePath<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("MultiNamePath").field(&self.0).finish()
    }
}

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
        ArgObj(ArgObj),
        LocalObj(LocalObj),
        NameString(NameString<A>),
    }
);

pub enum SuperName<A: Allocator> {
    SimpleName(SimpleName<A>),
    DebugObj(DebugObj),
    RefTypeOpcode(Box<RefTypeOpcode<A>, A>),
}

impl<A: Allocator> core::fmt::Debug for SuperName<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SimpleName(inner) => f.debug_tuple("SimpleName").field(&inner).finish(),
            Self::DebugObj(inner) => f.debug_tuple("DebugObj").field(&inner).finish(),
            Self::RefTypeOpcode(inner) => f.debug_tuple("RefTypeOpcode").field(&inner).finish(),
        }
    }
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
