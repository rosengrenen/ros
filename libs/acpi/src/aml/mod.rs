use crate::aml::util::item;

use self::util::{take, take_one};

pub mod util;

pub type ParseResult<I, O> = Result<(I, O), ()>;

pub fn term_list(mut input: &[u8]) -> ParseResult<&[u8], Vec<TermObj>> {
    let mut term_objs = Vec::new();

    while let Ok((next_input, term_obj)) = term_obj(input) {
        input = next_input;
        term_objs.push(term_obj)
    }

    panic!("{:x?}", &input[..16]);
    Ok((input, term_objs))
}

pub enum TermObj {
    Obj(Obj),
    Statement,
    Expr,
}

fn term_obj(input: &[u8]) -> ParseResult<&[u8], TermObj> {
    if let Ok((input, obj)) = obj(input) {
        return Ok((input, TermObj::Obj(obj)));
    }

    Err(())
}

pub enum Obj {
    NameSpaceModObj(NameSpaceModObj),
    NamedObj,
}

fn obj(input: &[u8]) -> ParseResult<&[u8], Obj> {
    if let Ok((input, name_space_mod_obj)) = name_space_mod_obj(input) {
        return Ok((input, Obj::NameSpaceModObj(name_space_mod_obj)));
    }

    Err(())
}

pub enum NameSpaceModObj {
    Alias,
    Name,
    Scope(Scope),
}

fn name_space_mod_obj(input: &[u8]) -> ParseResult<&[u8], NameSpaceModObj> {
    if let Ok((input, scope)) = scope(input) {
        return Ok((input, NameSpaceModObj::Scope(scope)));
    }

    Err(())
}

pub struct Scope {
    name: String,
    terms: Vec<TermObj>,
}

fn scope(input: &[u8]) -> ParseResult<&[u8], Scope> {
    let (input, _) = item(input, 0x10)?;
    let (input, pkg_len) = pkg_len(input)?;
    let (rest, input) = take(input, pkg_len)?;
    println!("package length {:?}", pkg_len);
    panic!("{:x?}", &input[..16]);
    let scope = Scope {};
    Ok((rest, scope))
}

//

fn pkg_len(input: &[u8]) -> ParseResult<&[u8], usize> {
    let (input, lead_byte) = take_one(input)?;
    let extra_bytes = (lead_byte >> 6) as usize;
    if extra_bytes == 0 {
        return Ok((input, lead_byte as usize - 1));
    }

    let (input, extra_bytes) = take(input, extra_bytes)?;
    if lead_byte & 0b0011_0000 != 0 {
        return Err(());
    }

    let mut pkg_length = (lead_byte & 0xf) as usize;
    for (i, &b) in extra_bytes.iter().enumerate() {
        pkg_length |= (b as usize) << (i * 8 + 4);
    }

    Ok((input, pkg_length - 1 - extra_bytes.len()))
}
