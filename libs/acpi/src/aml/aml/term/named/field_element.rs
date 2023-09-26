use crate::aml::{
    aml::{
        data::integer::byte_data,
        name::{NameSeg, NameString},
    },
    pkg_length, prefixed, Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
    primitive::item::item,
};

#[derive(Debug)]
pub struct NamedField(NameSeg, usize);

impl NamedField {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (NameSeg::p, pkg_length)
            .map(|(seg, len)| Self(seg, len))
            .add_context("NamedField")
            .parse(input, context, alloc)
    }
}

#[derive(Debug)]
pub struct ReservedField;

impl ReservedField {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (item(0x00), pkg_length)
            .map(|_| Self)
            .add_context("ReservedField")
            .parse(input, context, alloc)
    }
}

#[derive(Debug)]
pub struct AccessField {
    pub ty: u8,
    pub attrib: u8,
}

impl AccessField {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(item(0x01), (byte_data, byte_data))
            .map(|(ty, attrib)| Self { ty, attrib })
            .add_context("AccessField")
            .parse(input, context, alloc)
    }
}

#[derive(Debug)]
pub enum ConnectField<A: Allocator> {
    NameString(NameString<A>),
    // BufferData(BufferData),
}

impl<A: Allocator + Clone> ConnectField<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
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
#[derive(Debug)]
pub struct ExtendedAccessField {
    pub ty: u8,
    pub attrib: u8,
    pub len: u8,
}

impl ExtendedAccessField {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(item(0x13), (byte_data, byte_data, byte_data))
            .map(|(ty, attrib, len)| Self { ty, attrib, len })
            .add_context("ExtendedAccessField")
            .parse(input, context, alloc)
    }
}

#[derive(Debug)]
pub enum FieldElement<A: Allocator> {
    NamedField(NamedField),
    ReservedField(ReservedField),
    AccessField(AccessField),
    ExtendedAccessField(ExtendedAccessField),
    ConnectField(ConnectField<A>),
}

impl<A: Allocator + Clone> FieldElement<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            NamedField::p.map(Self::NamedField),
            ReservedField::p.map(Self::ReservedField),
            AccessField::p.map(Self::AccessField),
            ExtendedAccessField::p.map(Self::ExtendedAccessField),
            ConnectField::p.map(Self::ConnectField),
        )
            .alt()
            .add_context("FieldElement")
            .parse(input, context, alloc)
    }
}
