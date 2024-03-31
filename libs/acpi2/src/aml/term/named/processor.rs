use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    data::{byte_data, dword_data},
    name::NameString,
    ops::ProcessorOp,
    parser::{fail, fail_if_not_empty, Input, ParseResult},
    pkg_len::pkg,
    term::TermObj,
};
use alloc::vec::Vec;

pub struct Processor<A: Allocator> {
    pub name: NameString<A>,
    pub proc_id: u8,
    pub pblk_addr: u32,
    pub pblk_len: u8,
    pub terms: Vec<TermObj<A>, A>,
}

impl<A: Allocator + Clone> Processor<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = ProcessorOp::parse(input)?;
        let (pkg_input, input) = pkg(input)?;
        let (processor, pkg_input) = fail(Self::parse_inner(pkg_input, context, alloc))?;
        fail_if_not_empty(pkg_input)?;
        Ok((processor, input))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (name, input) = NameString::parse(input, alloc.clone())?;
        context.push_scope(&name);
        let (proc_id, input) = byte_data(input)?;
        let (pblk_addr, input) = dword_data(input)?;
        let (pblk_len, mut input) = byte_data(input)?;
        let mut terms = Vec::new(alloc.clone());
        while let Ok((term, i)) = TermObj::parse(input, context, alloc.clone()) {
            terms.push(term).unwrap();
            input = i;
        }

        context.pop_scope();
        let this = Self {
            name,
            proc_id,
            pblk_addr,
            pblk_len,
            terms,
        };
        Ok((this, input))
    }
}

impl<A: Allocator> core::fmt::Debug for Processor<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Processor")
            .field("name", &self.name)
            .field("proc_id", &self.proc_id)
            .field("pblk_addr", &self.pblk_addr)
            .field("pblk_len", &self.pblk_len)
            .field("terms", &self.terms)
            .finish()
    }
}
