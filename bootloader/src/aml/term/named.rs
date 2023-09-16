use super::{TermArg, TermList};
use crate::aml::{
    data::{byte_data, dword_data, word_data},
    name::{NameSeg, NameString},
    ops::{
        BankFieldOp, CreateBitFieldOp, CreateByteFieldOp, CreateDWordFieldOp, CreateFieldOp,
        CreateQWordFieldOp, CreateWordFieldOp, DataRegionOp, DeviceOp, EventOp, ExternalOp,
        FieldOp, IndexFieldOp, MethodOp, MutexOp, OpRegionOp, PowerResOp, ThermalZoneOp,
    },
    pkg, pkg_length,
};
use alloc::vec::Vec;
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    multi::many::many,
    parser::Parser,
    primitive::{item::item, rest::rest},
    sequence::preceded,
};

pub enum NamedObj<A: Allocator> {
    DefBankField(DefBankField<A>),
    DefCreateBitField(DefCreateBitField<A>),
    DefCreateByteField(DefCreateByteField<A>),
    DefCreateDWordField(DefCreateDWordField<A>),
    DefCreateField(DefCreateField<A>),
    DefCreateQWordField(DefCreateQWordField<A>),
    DefCreateWordField(DefCreateWordField<A>),
    DefDataRegion(DefDataRegion<A>),
    DefExternal(DefExternal<A>),
    DefOpRegion(DefOpRegion<A>),
    DefPowerRes(DefPowerRes<A>),
    DefThermalZone(DefThermalZone<A>),
    // Not in spec, but should probably be here, see: https://forum.osdev.org/viewtopic.php?f=1&t=29070
    DefField(DefField<A>),
    // Not in spec, but should probably be here, see: https://forum.osdev.org/viewtopic.php?f=1&t=33186
    DefMethod(DefMethod<A>),
}

impl<A: Allocator + Clone> NamedObj<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            &DefBankField::p.map(Self::DefBankField),
            &DefCreateBitField::p.map(Self::DefCreateBitField),
            &DefCreateByteField::p.map(Self::DefCreateByteField),
            &DefCreateDWordField::p.map(Self::DefCreateDWordField),
            &DefCreateField::p.map(Self::DefCreateField),
            &DefCreateQWordField::p.map(Self::DefCreateQWordField),
            &DefCreateWordField::p.map(Self::DefCreateWordField),
            &DefDataRegion::p.map(Self::DefDataRegion),
            &DefExternal::p.map(Self::DefExternal),
            &DefOpRegion::p.map(Self::DefOpRegion),
            &DefPowerRes::p.map(Self::DefPowerRes),
            &DefThermalZone::p.map(Self::DefThermalZone),
            &DefField::p.map(Self::DefField),
            &DefMethod::p.map(Self::DefMethod),
        )
            .alt()
            .add_context("NamedObj")
            .parse(input, alloc)
    }
}

pub struct DefBankField<A: Allocator> {
    name1: NameString<A>,
    name2: NameString<A>,
    bank_value: TermArg<A>,
    field_flags: FieldFlags,
    field_list: FieldList<A>,
}

impl<A: Allocator + Clone> DefBankField<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let bank_value = TermArg::p; // => Integer
        preceded(
            &BankFieldOp::p,
            &pkg(&(
                &NameString::p,
                &NameString::p,
                &bank_value,
                &FieldFlags::p,
                &FieldList::p,
            )),
        )
        .map(|(name1, name2, bank_value, field_flags, field_list)| Self {
            name1,
            name2,
            bank_value,
            field_flags,
            field_list,
        })
        .add_context("DefBankField")
        .parse(input, alloc)
    }
}

pub struct FieldFlags(u8);

impl FieldFlags {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        byte_data
            .map(Self)
            .add_context("FieldFlags")
            .parse(input, alloc)
    }
}

pub struct FieldList<A: Allocator>(Vec<FieldElement<A>, A>);

impl<A: Allocator + Clone> FieldList<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        many(&FieldElement::p)
            .map(Self)
            .add_context("FieldList")
            .parse(input, alloc)
    }
}

pub struct NamedField(NameSeg, usize);

impl NamedField {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (&NameSeg::p, &pkg_length.cut())
            .map(|(seg, len)| Self(seg, len))
            .add_context("NamedField")
            .parse(input, alloc)
    }
}

pub struct ReservedField;

impl ReservedField {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (&item(0x00), &pkg(&rest()))
            .map(|_| Self)
            .add_context("ReservedField")
            .parse(input, alloc)
    }
}

pub struct AccessField {
    ty: AccessType,
    attrib: AccessAttrib,
}

impl AccessField {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        preceded(&item(0x01), &(&AccessType::p, &AccessAttrib::p).cut())
            .map(|(ty, attrib)| Self { ty, attrib })
            .add_context("AccessField")
            .parse(input, alloc)
    }
}

pub struct AccessType(u8);

impl AccessType {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        byte_data
            .map(Self)
            .add_context("AccessType")
            .parse(input, alloc)
    }
}

pub struct AccessAttrib(u8);

impl AccessAttrib {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        byte_data
            .map(Self)
            .add_context("AccessAttrib")
            .parse(input, alloc)
    }
}

pub enum ConnectField<A: Allocator> {
    NameString(NameString<A>),
    // BufferData(BufferData),
}

impl<A: Allocator + Clone> ConnectField<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        preceded(
            &item(0x02),
            // (
            // NameString::p.map(Self::NameString),
            // BufferData::p.map(Self::BufferData),
            // )
            // .alt()
            &NameString::p.map(Self::NameString).cut(),
        )
        .add_context("ConnectField")
        .parse(input, alloc)
    }
}

pub struct DefCreateBitField<A: Allocator> {
    source_buf: TermArg<A>,
    bit_index: TermArg<A>,
    name: NameString<A>,
}

impl<A: Allocator + Clone> DefCreateBitField<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let source_buff = TermArg::p; // => Buffer
        let bit_index = TermArg::p; // => Integer
        preceded(
            &CreateBitFieldOp::p,
            &(&source_buff, &bit_index, &NameString::p).cut(),
        )
        .map(|(source_buf, bit_index, name)| Self {
            source_buf,
            bit_index,
            name,
        })
        .add_context("DefCreateBitField")
        .parse(input, alloc)
    }
}

pub struct DefCreateByteField<A: Allocator> {
    source_buf: TermArg<A>,
    byte_index: TermArg<A>,
    name: NameString<A>,
}

impl<A: Allocator + Clone> DefCreateByteField<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let source_buff = TermArg::p; // => Buffer
        let byte_index = TermArg::p; // => Integer
        preceded(
            &CreateByteFieldOp::p,
            &(&source_buff, &byte_index, &NameString::p).cut(),
        )
        .map(|(source_buf, bit_index, name)| Self {
            source_buf,
            byte_index: bit_index,
            name,
        })
        .add_context("DefCreateByteField")
        .parse(input, alloc)
    }
}

pub struct DefCreateDWordField<A: Allocator> {
    source_buf: TermArg<A>,
    byte_index: TermArg<A>,
    name: NameString<A>,
}

impl<A: Allocator + Clone> DefCreateDWordField<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let source_buff = TermArg::p; // => Buffer
        let byte_index = TermArg::p; // => Integer
        preceded(
            &CreateDWordFieldOp::p,
            &(&source_buff, &byte_index, &NameString::p).cut(),
        )
        .map(|(source_buf, byte_index, name)| Self {
            source_buf,
            byte_index,
            name,
        })
        .add_context("DefCreateDWordField")
        .parse(input, alloc)
    }
}

pub struct DefCreateField<A: Allocator> {
    source_buf: TermArg<A>,
    bit_index: TermArg<A>,
    num_bits: TermArg<A>,
    name: NameString<A>,
}

impl<A: Allocator + Clone> DefCreateField<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let source_buff = TermArg::p; // => Buffer
        let bit_index = TermArg::p; // => Integer
        let num_bits = TermArg::p; // => Integer
        preceded(
            &CreateFieldOp::p,
            &(&source_buff, &bit_index, &num_bits, &NameString::p).cut(),
        )
        .map(|(source_buf, bit_index, num_bits, name)| Self {
            source_buf,
            bit_index,
            num_bits,
            name,
        })
        .add_context("DefCreateField")
        .parse(input, alloc)
    }
}

pub struct DefCreateQWordField<A: Allocator> {
    source_buf: TermArg<A>,
    byte_index: TermArg<A>,
    name: NameString<A>,
}

impl<A: Allocator + Clone> DefCreateQWordField<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let source_buff = TermArg::p; // => Buffer
        let byte_index = TermArg::p; // => Integer
        preceded(
            &CreateQWordFieldOp::p,
            &(&source_buff, &byte_index, &NameString::p).cut(),
        )
        .map(|(source_buf, bit_index, name)| Self {
            source_buf,
            byte_index: bit_index,
            name,
        })
        .add_context("DefCreateQWordField")
        .parse(input, alloc)
    }
}

pub struct DefCreateWordField<A: Allocator> {
    source_buf: TermArg<A>,
    byte_index: TermArg<A>,
    name: NameString<A>,
}

impl<A: Allocator + Clone> DefCreateWordField<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let source_buff = TermArg::p; // => Buffer
        let byte_index = TermArg::p; // => Integer
        preceded(
            &CreateWordFieldOp::p,
            &(&source_buff, &byte_index, &NameString::p).cut(),
        )
        .map(|(source_buf, byte_index, name)| Self {
            source_buf,
            byte_index,
            name,
        })
        .add_context("DefCreateWordField")
        .parse(input, alloc)
    }
}

pub struct DefDataRegion<A: Allocator> {
    name: NameString<A>,
    term1: TermArg<A>,
    term2: TermArg<A>,
    term3: TermArg<A>,
}

impl<A: Allocator + Clone> DefDataRegion<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        preceded(
            &DataRegionOp::p,
            &(&NameString::p, &TermArg::p, &TermArg::p, &TermArg::p).cut(),
        )
        .map(|(name, term1, term2, term3)| Self {
            name,
            term1,
            term2,
            term3,
        })
        .add_context("DefDataRegion")
        .parse(input, alloc)
    }
}

pub struct DefDevice<A: Allocator> {
    name: NameString<A>,
    terms: TermList<A>,
}

impl<A: Allocator + Clone> DefDevice<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        preceded(&DeviceOp::p, &pkg(&(&NameString::p, &TermList::p)))
            .map(|(name, terms)| Self { name, terms })
            .add_context("DefDevice")
            .parse(input, alloc)
    }
}

pub struct DefEvent<A: Allocator> {
    name: NameString<A>,
}

impl<A: Allocator + Clone> DefEvent<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        preceded(&EventOp::p, &NameString::p.cut())
            .map(|name| Self { name })
            .add_context("DefEvent")
            .parse(input, alloc)
    }
}

pub struct DefExternal<A: Allocator> {
    name: NameString<A>,
    obj_type: u8,
    argument_count: u8,
}

impl<A: Allocator + Clone> DefExternal<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        preceded(
            &ExternalOp::p,
            &(&NameString::p, &byte_data, &byte_data).cut(),
        )
        .map(|(name, obj_type, argument_count)| Self {
            name,
            obj_type,
            argument_count,
        })
        .add_context("DefExternal")
        .parse(input, alloc)
    }
}

pub struct DefField<A: Allocator> {
    name: NameString<A>,
    flags: FieldFlags,
    fields: FieldList<A>,
}

impl<A: Allocator + Clone> DefField<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        preceded(
            &FieldOp::p,
            &pkg(&(&NameString::p, &FieldFlags::p, &FieldList::p)),
        )
        .map(|(name, flags, fields)| Self {
            name,
            flags,
            fields,
        })
        .add_context("DefField")
        .parse(input, alloc)
    }
}

pub struct DefIndexField<A: Allocator> {
    name1: NameString<A>,
    name2: NameString<A>,
    flags: FieldFlags,
    fields: FieldList<A>,
}

impl<A: Allocator + Clone> DefIndexField<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        preceded(
            &IndexFieldOp::p,
            &pkg(&(
                &NameString::p,
                &NameString::p,
                &FieldFlags::p,
                &FieldList::p,
            )),
        )
        .map(|(name1, name2, flags, fields)| Self {
            name1,
            name2,
            flags,
            fields,
        })
        .add_context("DefIndexField")
        .parse(input, alloc)
    }
}

pub struct DefMethod<A: Allocator> {
    name: NameString<A>,
    flags: MethodFlags,
    terms: TermList<A>,
}

impl<A: Allocator + Clone> DefMethod<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        preceded(
            &MethodOp::p,
            &pkg(&(&NameString::p, &MethodFlags::p, &TermList::p)),
        )
        .map(|(name, flags, terms)| Self { name, flags, terms })
        .add_context("DefMethod")
        .parse(input, alloc)
    }
}

pub struct MethodFlags(u8);

impl MethodFlags {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        byte_data
            .map(Self)
            .add_context("MethodFlags")
            .parse(input, alloc)
    }
}

pub struct DefMutex<A: Allocator> {
    name: NameString<A>,
    flags: SyncFlags,
}

impl<A: Allocator + Clone> DefMutex<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        preceded(&MutexOp::p, &(&NameString::p, &SyncFlags::p).cut())
            .map(|(name, flags)| Self { name, flags })
            .add_context("DefMutex")
            .parse(input, alloc)
    }
}

pub struct SyncFlags(u8);

impl SyncFlags {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        byte_data
            .map(Self)
            .add_context("SyncFlags")
            .parse(input, alloc)
    }
}

pub struct DefOpRegion<A: Allocator> {
    name: NameString<A>,
    space: RegionSpace,
    offset: TermArg<A>,
    len: TermArg<A>,
}

impl<A: Allocator + Clone> DefOpRegion<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let region_offset = TermArg::p; // => Integer
        let region_len = TermArg::p; // => Integer
        preceded(
            &OpRegionOp::p,
            &(&NameString::p, &RegionSpace::p, &region_offset, &region_len).cut(),
        )
        .map(|(name, space, offset, len)| Self {
            name,
            space,
            offset,
            len,
        })
        .add_context("DefOpRegion")
        .parse(input, alloc)
    }
}

#[derive(Debug)]
pub struct RegionSpace(u8);

impl RegionSpace {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        byte_data
            .map(Self)
            .add_context("RegionSpace")
            .parse(input, alloc)
    }
}

pub struct DefPowerRes<A: Allocator> {
    name: NameString<A>,
    system_level: u8,
    resource_order: u16,
    terms: TermList<A>,
}

impl<A: Allocator + Clone> DefPowerRes<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        preceded(
            &PowerResOp::p,
            &pkg(&(&NameString::p, &system_level, &resource_order, &TermList::p)),
        )
        .map(|(name, system_level, resource_order, terms)| Self {
            name,
            system_level,
            resource_order,
            terms,
        })
        .add_context("DefPowerRes")
        .parse(input, alloc)
    }
}

fn system_level<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    alloc: A,
) -> ParseResult<I, u8, E> {
    byte_data.add_context("system_level").parse(input, alloc)
}

fn resource_order<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    alloc: A,
) -> ParseResult<I, u16, E> {
    word_data.add_context("resource_order").parse(input, alloc)
}

fn proc_id<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    alloc: A,
) -> ParseResult<I, u8, E> {
    byte_data.add_context("proc_id").parse(input, alloc)
}

fn pblk_addr<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    alloc: A,
) -> ParseResult<I, u32, E> {
    dword_data.add_context("pblk_addr").parse(input, alloc)
}

fn pblk_len<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    alloc: A,
) -> ParseResult<I, u8, E> {
    byte_data.add_context("pblk_len").parse(input, alloc)
}

pub struct DefThermalZone<A: Allocator> {
    name: NameString<A>,
    terms: TermList<A>,
}

impl<A: Allocator + Clone> DefThermalZone<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        preceded(&ThermalZoneOp::p, &pkg(&(&NameString::p, &TermList::p)))
            .map(|(name, terms)| Self { name, terms })
            .add_context("DefThermalZone")
            .parse(input, alloc)
    }
}

pub struct ExtendedAccessField {
    ty: AccessType,
    attrib: ExtendedAccessAttrib,
    len: AccessLength,
}

impl ExtendedAccessField {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        preceded(
            &item(0x13),
            &(&AccessType::p, &ExtendedAccessAttrib::p, &AccessLength::p).cut(),
        )
        .map(|(ty, attrib, len)| Self { ty, attrib, len })
        .add_context("ExtendedAccessField")
        .parse(input, alloc)
    }
}

pub struct ExtendedAccessAttrib(u8);

impl ExtendedAccessAttrib {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        byte_data
            .map(Self)
            .add_context("ExtendedAccessAttrib")
            .parse(input, alloc)
    }
}

pub struct AccessLength(u8);

impl AccessLength {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        byte_data
            .map(Self)
            .add_context("AccessLength")
            .parse(input, alloc)
    }
}

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
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            &NamedField::p.map(Self::NamedField),
            &ReservedField::p.map(Self::ReservedField),
            &AccessField::p.map(Self::AccessField),
            &ExtendedAccessField::p.map(Self::ExtendedAccessField),
            &ConnectField::p.map(Self::ConnectField),
        )
            .alt()
            .add_context("FieldElement")
            .parse(input, alloc)
    }
}
