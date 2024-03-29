use crate::aml::{
    data::byte_data,
    name::{NameSeg, NameString},
    pkg_len::pkg_length,
    prefixed::prefixed,
    Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
    primitive::item::item,
};

parser_struct!(
    struct Named {
        name: NameSeg,
        len: usize,
    },
    (NameSeg::p, pkg_length)
);

parser_struct_empty!(struct Reserved;, (item(0x00), pkg_length));

parser_struct!(
    struct Access {
        ty: u8,
        attrib: u8,
    },
    prefixed(item(0x01), (byte_data, byte_data))
);

pub enum Connect<A: Allocator> {
    NameString(NameString<A>),
    // BufferData(BufferData),
}

impl<A: Allocator> core::fmt::Debug for Connect<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NameString(inner) => f.debug_tuple("NameString").field(inner).finish(),
        }
    }
}

impl<A: Allocator + Clone> Connect<A> {
    fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(
            item(0x02),
            // (
            // NameString::p.map(Self::NameString),
            // BufferData::p.map(Self::BufferData),
            // )
            // .alt()
            NameString::p.map(Self::NameString),
        )
        .add_context("ConnectField")
        .parse(input, context, alloc)
    }
}

parser_struct!(
    struct ExtendedAccess {
        ty: u8,
        attrib: u8,
        len: u8,
    },
    prefixed(item(0x13), (byte_data, byte_data, byte_data))
);

parser_enum_alloc!(
    enum FieldElement {
        Named(Named),
        Reserved(Reserved),
        Access(Access),
        ExtendedAccess(ExtendedAccess),
        Connect(Connect<A>),
    }
);
