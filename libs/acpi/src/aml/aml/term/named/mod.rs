pub mod bank_field;
pub mod create_field;
pub mod data_region;
pub mod device;
pub mod event;
pub mod external;
pub mod field;
pub mod field_element;
pub mod index_field;
pub mod method;
pub mod mutex;
pub mod op_region;
pub mod power_res;
pub mod thermal_zone;

use self::{
    bank_field::BankField,
    create_field::{CreateConstField, CreateField},
    data_region::DataRegion,
    external::External,
    field::Field,
    method::Method,
    op_region::OpRegion,
    power_res::PowerRes,
    thermal_zone::ThermalZone,
};
use crate::aml::Context;
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub enum NamedObj<A: Allocator> {
    BankField(BankField<A>),
    CreateConstField(CreateConstField<A>),
    CreateField(CreateField<A>),
    DataRegion(DataRegion<A>),
    External(External<A>),
    OpRegion(OpRegion<A>),
    PowerRes(PowerRes<A>),
    ThermalZone(ThermalZone<A>),
    // Not in spec, but should probably be here, see: https://forum.osdev.org/viewtopic.php?f=1t=29070
    Field(Field<A>),
    // Not in spec, but should probably be here, see: https://forum.osdev.org/viewtopic.php?f=1t=33186
    Method(Method<A>),
}

impl<A: Allocator + Clone> NamedObj<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            BankField::p.map(Self::BankField),
            CreateConstField::p.map(Self::CreateConstField),
            CreateField::p.map(Self::CreateField),
            DataRegion::p.map(Self::DataRegion),
            External::p.map(Self::External),
            OpRegion::p.map(Self::OpRegion),
            PowerRes::p.map(Self::PowerRes),
            ThermalZone::p.map(Self::ThermalZone),
            Field::p.map(Self::Field),
            Method::p.map(Self::Method),
        )
            .alt()
            .add_context("NamedObj")
            .parse(input, context, alloc)
    }
}
