use super::{TermArg, TermObj};
use crate::aml::{
    aml::{
        data::{byte_data, dword_data, word_data},
        name::{NameSeg, NameString},
    },
    ops::{
        BankFieldOp, CreateBitFieldOp, CreateByteFieldOp, CreateDWordFieldOp, CreateFieldOp,
        CreateQWordFieldOp, CreateWordFieldOp, DataRegionOp, DeviceOp, EventOp, ExternalOp,
        FieldOp, IndexFieldOp, MethodOp, MutexOp, OpRegionOp, PowerResOp, ThermalZoneOp,
    },
    pkg, pkg_length, prefixed, Context,
};
use alloc::vec::Vec;
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    multi::many::many,
    parser::Parser,
    primitive::{item::item, rest::rest},
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
    // Not in spec, but should probably be here, see: https://forum.osdev.org/viewtopic.php?f=1t=29070
    DefField(DefField<A>),
    // Not in spec, but should probably be here, see: https://forum.osdev.org/viewtopic.php?f=1t=33186
    DefMethod(DefMethod<A>),
}

impl<A: Allocator + Clone> NamedObj<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            DefBankField::p.map(Self::DefBankField),
            DefCreateBitField::p.map(Self::DefCreateBitField),
            DefCreateByteField::p.map(Self::DefCreateByteField),
            DefCreateDWordField::p.map(Self::DefCreateDWordField),
            DefCreateField::p.map(Self::DefCreateField),
            DefCreateQWordField::p.map(Self::DefCreateQWordField),
            DefCreateWordField::p.map(Self::DefCreateWordField),
            DefDataRegion::p.map(Self::DefDataRegion),
            DefExternal::p.map(Self::DefExternal),
            DefOpRegion::p.map(Self::DefOpRegion),
            DefPowerRes::p.map(Self::DefPowerRes),
            DefThermalZone::p.map(Self::DefThermalZone),
            DefField::p.map(Self::DefField),
            DefMethod::p.map(Self::DefMethod),
        )
            .alt()
            .add_context("NamedObj")
            .parse(input, context, alloc)
    }
}

pub struct DefBankField<A: Allocator> {
    pub name1: NameString<A>,
    pub name2: NameString<A>,
    pub bank_value: TermArg<A>,
    pub field_flags: FieldFlags,
    pub field_list: Vec<FieldElement<A>, A>,
}

impl<A: Allocator + Clone> DefBankField<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let bank_value = TermArg::p; // => Integer
        prefixed(
            BankFieldOp::p,
            pkg((
                NameString::p,
                NameString::p,
                bank_value,
                FieldFlags::p,
                many(FieldElement::p),
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
        .parse(input, context, alloc)
    }
}

pub struct FieldFlags(u8);

impl FieldFlags {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        byte_data
            .map(Self)
            .add_context("FieldFlags")
            .parse(input, context, alloc)
    }
}

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

pub struct ReservedField;

impl ReservedField {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (item(0x00), pkg(rest()))
            .map(|_| Self)
            .add_context("ReservedField")
            .parse(input, context, alloc)
    }
}

pub struct AccessField {
    pub ty: AccessType,
    pub attrib: AccessAttrib,
}

impl AccessField {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(item(0x01), (AccessType::p, AccessAttrib::p))
            .map(|(ty, attrib)| Self { ty, attrib })
            .add_context("AccessField")
            .parse(input, context, alloc)
    }
}

pub struct AccessType(u8);

impl AccessType {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        byte_data
            .map(Self)
            .add_context("AccessType")
            .parse(input, context, alloc)
    }
}

pub struct AccessAttrib(u8);

impl AccessAttrib {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        byte_data
            .map(Self)
            .add_context("AccessAttrib")
            .parse(input, context, alloc)
    }
}

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

pub struct DefCreateBitField<A: Allocator> {
    pub source_buf: TermArg<A>,
    pub bit_index: TermArg<A>,
    pub name: NameString<A>,
}

impl<A: Allocator + Clone> DefCreateBitField<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let source_buff = TermArg::p; // => Buffer
        let bit_index = TermArg::p; // => Integer
        prefixed(CreateBitFieldOp::p, (source_buff, bit_index, NameString::p))
            .map(|(source_buf, bit_index, name)| Self {
                source_buf,
                bit_index,
                name,
            })
            .add_context("DefCreateBitField")
            .parse(input, context, alloc)
    }
}

pub struct DefCreateByteField<A: Allocator> {
    pub source_buf: TermArg<A>,
    pub byte_index: TermArg<A>,
    pub name: NameString<A>,
}

impl<A: Allocator + Clone> DefCreateByteField<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let source_buff = TermArg::p; // => Buffer
        let byte_index = TermArg::p; // => Integer
        prefixed(
            CreateByteFieldOp::p,
            (source_buff, byte_index, NameString::p),
        )
        .map(|(source_buf, bit_index, name)| Self {
            source_buf,
            byte_index: bit_index,
            name,
        })
        .add_context("DefCreateByteField")
        .parse(input, context, alloc)
    }
}

pub struct DefCreateDWordField<A: Allocator> {
    pub source_buf: TermArg<A>,
    pub byte_index: TermArg<A>,
    pub name: NameString<A>,
}

impl<A: Allocator + Clone> DefCreateDWordField<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let source_buff = TermArg::p; // => Buffer
        let byte_index = TermArg::p; // => Integer
        prefixed(
            CreateDWordFieldOp::p,
            (source_buff, byte_index, NameString::p),
        )
        .map(|(source_buf, byte_index, name)| Self {
            source_buf,
            byte_index,
            name,
        })
        .add_context("DefCreateDWordField")
        .parse(input, context, alloc)
    }
}

pub struct DefCreateField<A: Allocator> {
    pub source_buf: TermArg<A>,
    pub bit_index: TermArg<A>,
    pub num_bits: TermArg<A>,
    pub name: NameString<A>,
}

impl<A: Allocator + Clone> DefCreateField<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let source_buff = TermArg::p; // => Buffer
        let bit_index = TermArg::p; // => Integer
        let num_bits = TermArg::p; // => Integer
        prefixed(
            CreateFieldOp::p,
            (source_buff, bit_index, num_bits, NameString::p),
        )
        .map(|(source_buf, bit_index, num_bits, name)| Self {
            source_buf,
            bit_index,
            num_bits,
            name,
        })
        .add_context("DefCreateField")
        .parse(input, context, alloc)
    }
}

pub struct DefCreateQWordField<A: Allocator> {
    pub source_buf: TermArg<A>,
    pub byte_index: TermArg<A>,
    pub name: NameString<A>,
}

impl<A: Allocator + Clone> DefCreateQWordField<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let source_buff = TermArg::p; // => Buffer
        let byte_index = TermArg::p; // => Integer
        prefixed(
            CreateQWordFieldOp::p,
            (source_buff, byte_index, NameString::p),
        )
        .map(|(source_buf, bit_index, name)| Self {
            source_buf,
            byte_index: bit_index,
            name,
        })
        .add_context("DefCreateQWordField")
        .parse(input, context, alloc)
    }
}

pub struct DefCreateWordField<A: Allocator> {
    pub source_buf: TermArg<A>,
    pub byte_index: TermArg<A>,
    pub name: NameString<A>,
}

impl<A: Allocator + Clone> DefCreateWordField<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let source_buff = TermArg::p; // => Buffer
        let byte_index = TermArg::p; // => Integer
        prefixed(
            CreateWordFieldOp::p,
            (source_buff, byte_index, NameString::p),
        )
        .map(|(source_buf, byte_index, name)| Self {
            source_buf,
            byte_index,
            name,
        })
        .add_context("DefCreateWordField")
        .parse(input, context, alloc)
    }
}

pub struct DefDataRegion<A: Allocator> {
    pub name: NameString<A>,
    pub term1: TermArg<A>,
    pub term2: TermArg<A>,
    pub term3: TermArg<A>,
}

impl<A: Allocator + Clone> DefDataRegion<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(
            DataRegionOp::p,
            (NameString::p, TermArg::p, TermArg::p, TermArg::p),
        )
        .map(|(name, term1, term2, term3)| Self {
            name,
            term1,
            term2,
            term3,
        })
        .add_context("DefDataRegion")
        .parse(input, context, alloc)
    }
}

pub struct DefDevice<A: Allocator> {
    pub name: NameString<A>,
    pub terms: Vec<TermObj<A>, A>,
}

impl<A: Allocator + Clone> DefDevice<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(DeviceOp::p, pkg((NameString::p, many(TermObj::p))))
            .map(|(name, terms)| Self { name, terms })
            .add_context("DefDevice")
            .parse(input, context, alloc)
    }
}

pub struct DefEvent<A: Allocator> {
    pub name: NameString<A>,
}

impl<A: Allocator + Clone> DefEvent<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(EventOp::p, NameString::p)
            .map(|name| Self { name })
            .add_context("DefEvent")
            .parse(input, context, alloc)
    }
}

pub struct DefExternal<A: Allocator> {
    pub name: NameString<A>,
    pub obj_type: u8,
    pub argument_count: u8,
}

impl<A: Allocator + Clone> DefExternal<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(ExternalOp::p, (NameString::p, byte_data, byte_data))
            .map(|(name, obj_type, argument_count)| Self {
                name,
                obj_type,
                argument_count,
            })
            .add_context("DefExternal")
            .parse(input, context, alloc)
    }
}

pub struct DefField<A: Allocator> {
    pub name: NameString<A>,
    pub flags: FieldFlags,
    pub fields: Vec<FieldElement<A>, A>,
}

impl<A: Allocator + Clone> DefField<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        // fail().add_context("heh").parse(input, context, alloc)?;
        // panic!("heh");
        prefixed(
            FieldOp::p,
            pkg((NameString::p, FieldFlags::p, many(FieldElement::p))),
        )
        .map(|(name, flags, fields)| Self {
            name,
            flags,
            fields,
        })
        .add_context("DefField")
        .parse(input, context, alloc)
    }
}

pub struct DefIndexField<A: Allocator> {
    pub name1: NameString<A>,
    pub name2: NameString<A>,
    pub flags: FieldFlags,
    pub fields: Vec<FieldElement<A>, A>,
}

impl<A: Allocator + Clone> DefIndexField<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(
            IndexFieldOp::p,
            pkg((
                NameString::p,
                NameString::p,
                FieldFlags::p,
                many(FieldElement::p),
            )),
        )
        .map(|(name1, name2, flags, fields)| Self {
            name1,
            name2,
            flags,
            fields,
        })
        .add_context("DefIndexField")
        .parse(input, context, alloc)
    }
}

pub struct DefMethod<A: Allocator> {
    pub name: NameString<A>,
    pub flags: MethodFlags,
    pub terms: Vec<TermObj<A>, A>,
}

impl<A: Allocator + Clone> DefMethod<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(
            MethodOp::p,
            pkg((NameString::p, MethodFlags::p, many(TermObj::p))),
        )
        .map(|(name, flags, terms)| Self { name, flags, terms })
        .add_context("DefMethod")
        .parse(input, context, alloc)
    }
}

pub struct MethodFlags(u8);

impl MethodFlags {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        byte_data
            .map(Self)
            .add_context("MethodFlags")
            .parse(input, context, alloc)
    }
}

pub struct DefMutex<A: Allocator> {
    pub name: NameString<A>,
    pub flags: SyncFlags,
}

impl<A: Allocator + Clone> DefMutex<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(MutexOp::p, (NameString::p, SyncFlags::p))
            .map(|(name, flags)| Self { name, flags })
            .add_context("DefMutex")
            .parse(input, context, alloc)
    }
}

pub struct SyncFlags(u8);

impl SyncFlags {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        byte_data
            .map(Self)
            .add_context("SyncFlags")
            .parse(input, context, alloc)
    }
}

pub struct DefOpRegion<A: Allocator> {
    pub name: NameString<A>,
    pub space: RegionSpace,
    pub offset: TermArg<A>,
    pub len: TermArg<A>,
}

impl<A: Allocator + Clone> DefOpRegion<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let region_offset = TermArg::p; // => Integer
        let region_len = TermArg::p; // => Integer
        prefixed(
            OpRegionOp::p,
            (NameString::p, RegionSpace::p, region_offset, region_len).add_context("eh hi there"),
        )
        .map(|(name, space, offset, len)| Self {
            name,
            space,
            offset,
            len,
        })
        .add_context("DefOpRegion")
        .parse(input, context, alloc)
    }
}

#[derive(Debug)]
pub struct RegionSpace(u8);

impl RegionSpace {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        byte_data
            .map(Self)
            .add_context("RegionSpace")
            .parse(input, context, alloc)
    }
}

pub struct DefPowerRes<A: Allocator> {
    pub name: NameString<A>,
    pub system_level: u8,
    pub resource_order: u16,
    pub terms: Vec<TermObj<A>, A>,
}

impl<A: Allocator + Clone> DefPowerRes<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(
            PowerResOp::p,
            pkg((
                NameString::p,
                system_level,
                resource_order,
                many(TermObj::p),
            )),
        )
        .map(|(name, system_level, resource_order, terms)| Self {
            name,
            system_level,
            resource_order,
            terms,
        })
        .add_context("DefPowerRes")
        .parse(input, context, alloc)
    }
}

fn system_level<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    context: &mut Context,
    alloc: A,
) -> ParseResult<I, u8, E> {
    byte_data
        .add_context("system_level")
        .parse(input, context, alloc)
}

fn resource_order<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    context: &mut Context,
    alloc: A,
) -> ParseResult<I, u16, E> {
    word_data
        .add_context("resource_order")
        .parse(input, context, alloc)
}

#[allow(dead_code)]
fn proc_id<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    context: &mut Context,
    alloc: A,
) -> ParseResult<I, u8, E> {
    byte_data
        .add_context("proc_id")
        .parse(input, context, alloc)
}

#[allow(dead_code)]
fn pblk_addr<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    context: &mut Context,
    alloc: A,
) -> ParseResult<I, u32, E> {
    dword_data
        .add_context("pblk_addr")
        .parse(input, context, alloc)
}

#[allow(dead_code)]
fn pblk_len<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    context: &mut Context,
    alloc: A,
) -> ParseResult<I, u8, E> {
    byte_data
        .add_context("pblk_len")
        .parse(input, context, alloc)
}

pub struct DefThermalZone<A: Allocator> {
    pub name: NameString<A>,
    pub terms: Vec<TermObj<A>, A>,
}

impl<A: Allocator + Clone> DefThermalZone<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(ThermalZoneOp::p, pkg((NameString::p, many(TermObj::p))))
            .map(|(name, terms)| Self { name, terms })
            .add_context("DefThermalZone")
            .parse(input, context, alloc)
    }
}

pub struct ExtendedAccessField {
    pub ty: AccessType,
    pub attrib: ExtendedAccessAttrib,
    pub len: AccessLength,
}

impl ExtendedAccessField {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(
            item(0x13),
            (AccessType::p, ExtendedAccessAttrib::p, AccessLength::p),
        )
        .map(|(ty, attrib, len)| Self { ty, attrib, len })
        .add_context("ExtendedAccessField")
        .parse(input, context, alloc)
    }
}

pub struct ExtendedAccessAttrib(u8);

impl ExtendedAccessAttrib {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        byte_data
            .map(Self)
            .add_context("ExtendedAccessAttrib")
            .parse(input, context, alloc)
    }
}

pub struct AccessLength(u8);

impl AccessLength {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        byte_data
            .map(Self)
            .add_context("AccessLength")
            .parse(input, context, alloc)
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
