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
pub mod processor;
pub mod thermal_zone;

use core::alloc::Allocator;

use self::bank_field::BankField;
use self::create_field::CreateConstField;
use self::create_field::CreateField;
use self::data_region::DataRegion;
use self::device::Device;
use self::event::Event;
use self::external::External;
use self::field::Field;
use self::index_field::IndexField;
use self::method::Method;
use self::mutex::Mutex;
use self::op_region::OpRegion;
use self::power_res::PowerRes;
use self::thermal_zone::ThermalZone;
use crate::aml::context::Context;
use crate::aml::parser::Input;
use crate::aml::parser::ParseResult;
use crate::aml::parser::ParserError;
use crate::aml::term::named::processor::Processor;

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
    // More that aren't included
    Device(Device<A>),
    Event(Event<A>),
    IndexField(IndexField<A>),
    Mutex(Mutex<A>),
    // Deprecated in 6.4
    Processor(Processor<A>),
}

impl<A: Allocator + Clone> NamedObj<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        match BankField::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::BankField(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match CreateConstField::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::CreateConstField(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match CreateField::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::CreateField(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match DataRegion::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::DataRegion(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match External::parse(input, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::External(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match OpRegion::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::OpRegion(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match PowerRes::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::PowerRes(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match ThermalZone::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::ThermalZone(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Field::parse(input, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Field(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Method::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Method(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Device::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Device(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Event::parse(input, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Event(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match IndexField::parse(input, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::IndexField(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Mutex::parse(input, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Mutex(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        let (value, input) = Processor::parse(input, context, alloc)?;
        Ok((Self::Processor(value), input))
    }
}

impl<A: Allocator> core::fmt::Debug for NamedObj<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::BankField(arg0) => f.debug_tuple("BankField").field(arg0).finish(),
            Self::CreateConstField(arg0) => f.debug_tuple("CreateConstField").field(arg0).finish(),
            Self::CreateField(arg0) => f.debug_tuple("CreateField").field(arg0).finish(),
            Self::DataRegion(arg0) => f.debug_tuple("DataRegion").field(arg0).finish(),
            Self::External(arg0) => f.debug_tuple("External").field(arg0).finish(),
            Self::OpRegion(arg0) => f.debug_tuple("OpRegion").field(arg0).finish(),
            Self::PowerRes(arg0) => f.debug_tuple("PowerRes").field(arg0).finish(),
            Self::ThermalZone(arg0) => f.debug_tuple("ThermalZone").field(arg0).finish(),
            Self::Field(arg0) => f.debug_tuple("Field").field(arg0).finish(),
            Self::Method(arg0) => f.debug_tuple("Method").field(arg0).finish(),
            Self::Device(arg0) => f.debug_tuple("Device").field(arg0).finish(),
            Self::Event(arg0) => f.debug_tuple("Event").field(arg0).finish(),
            Self::IndexField(arg0) => f.debug_tuple("IndexField").field(arg0).finish(),
            Self::Mutex(arg0) => f.debug_tuple("Mutex").field(arg0).finish(),
            Self::Processor(arg0) => f.debug_tuple("Processor").field(arg0).finish(),
        }
    }
}
