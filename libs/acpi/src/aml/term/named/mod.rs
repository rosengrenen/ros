mod bank_field;
mod create_field;
mod data_region;
mod device;
mod event;
mod external;
mod field;
mod field_element;
mod index_field;
mod method;
mod mutex;
mod op_region;
mod power_res;
mod processor;
mod thermal_zone;

use self::{
    bank_field::BankField,
    create_field::{CreateConstField, CreateField},
    data_region::DataRegion,
    device::Device,
    event::Event,
    external::External,
    field::Field,
    index_field::IndexField,
    method::Method,
    mutex::Mutex,
    op_region::OpRegion,
    power_res::PowerRes,
    thermal_zone::ThermalZone,
};
use crate::aml::term::named::processor::Processor;

parser_enum_alloc!(
    enum NamedObj {
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
);
