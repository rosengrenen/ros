use byte_parser::{
    branch::alt,
    combinator::{map, map_res},
    input::Input,
    parser::Parser,
    primitive::{byte, satisfy, take, take_one, take_while},
    sequence::tuple,
    ParserResult,
};

// pub fn parse_definition_block(mut input: &[u8]) {
//     // let mut offset = 0;
//     // take many of some things
//     loop {
//         let (new_input, ty) = parse_block_type(input);
//         input = new_input;
//         // sprintln!("Block type {:x} at offset {:x}", ty, offset);
//         match ty {
//             0x10 => {
//                 let (new_input, _) = parse_def_scope(input);
//                 input = new_input;
//             }
//             _ => {
//                 panic!("unknown block type {:x}", ty);
//             }
//         }
//     }
// }

// fn parse_block_type(input: &[u8]) -> ParseResult<u8> {
//     (&input[1..], input[0])
// }

// fn parse_def_scope(input: &[u8]) -> ParseResult<()> {
//     let (input, pkg_length) = parse_pkg_length(input);
//     parse_name_string(input);
//     (&input[pkg_length..], ())
// }

pub fn pkg_length<'input>(input: Input<'input>) -> ParserResult<'input, usize> {
    let (input, lead_byte, span) = take_one().parse(input)?;
    let extra_bytes = (lead_byte >> 6) as usize;
    if extra_bytes == 0 {
        return Ok((input, lead_byte as usize, span));
    }

    map_res(take(extra_bytes as usize), move |extra_bytes| {
        if lead_byte & 0b0011_0000 != 0 {
            return Err(());
        }

        let mut pkg_length = (lead_byte & 0xf) as usize;
        for (i, &b) in extra_bytes.iter().enumerate() {
            pkg_length |= (b as usize) << (i * 8 + 4);
        }

        Ok(pkg_length)
    })
    .parse(input)
}

// fn parse_name_string(input: &[u8]) -> ParseResult<()> {
//     if let Some((input, _)) = parse_root_char(input) {
//         return parse_name_path(input);
//     } else {
//         let (input, _) = parse_prefix_path(input);
//         return parse_name_path(input);
//     }
// }

// fn parse_root_char(input: &[u8]) -> Option<ParseResult<()>> {
//     if input[0] == b'\\' {
//         return Some((&input[1..], ()));
//     }

//     None
// }

#[derive(Debug)]
pub struct PrefixPath(usize);

pub fn prefix_path<'input>(input: Input<'input>) -> ParserResult<'input, PrefixPath> {
    map(take_while(|b| b == b'^'), |value| PrefixPath(value.len())).parse(input)
}

#[derive(Debug)]
pub enum NamePath {
    NameSeg(NameSeg),
    DualNamePath(DualNamePath),
    NullName(NullName),
}

pub fn name_path<'input>(input: Input<'input>) -> ParserResult<'input, NamePath> {
    alt((
        map(name_seg, |value| NamePath::NameSeg(value)),
        map(dual_name_path, |value| NamePath::DualNamePath(value)),
        map(null_name, |value| NamePath::NullName(value)),
    ))
    .parse(input)
}

pub struct DualNamePath([u8; 8]);

impl core::fmt::Debug for DualNamePath {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DualNamePath")
            .field(unsafe { &core::str::from_utf8_unchecked(&self.0) })
            .finish()
    }
}

fn dual_name_path<'input>(input: Input<'input>) -> ParserResult<'input, DualNamePath> {
    let (input, _, _) = byte(b'.').parse(input)?;
    let (input, (NameSeg(seg0), NameSeg(seg1)), span) = tuple((name_seg, name_seg)).parse(input)?;
    Ok((input, DualNamePath(concat_arrays(seg0, seg1)), span))
}

fn concat_arrays<T, const A: usize, const B: usize, const C: usize>(
    a: [T; A],
    b: [T; B],
) -> [T; C] {
    assert_eq!(A + B, C);
    let mut iter = a.into_iter().chain(b);
    core::array::from_fn(|_| iter.next().unwrap())
}

#[derive(Debug)]
pub struct NullName;

fn null_name<'input>(input: Input<'input>) -> ParserResult<'input, NullName> {
    map(byte(0), |_| NullName).parse(input)
}

pub struct NameSeg([u8; 4]);

impl core::fmt::Debug for NameSeg {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NameSeg")
            .field(unsafe { &core::str::from_utf8_unchecked(&self.0) })
            .finish()
    }
}

fn name_seg<'input>(input: Input<'input>) -> ParserResult<'input, NameSeg> {
    let (input, (lead, c0, c1, c2), span) =
        tuple((lead_name_char, name_char, name_char, name_char)).parse(input)?;
    Ok((input, NameSeg([lead, c0, c1, c2]), span))
}

fn lead_name_char<'input>(input: Input<'input>) -> ParserResult<'input, u8> {
    satisfy(|b| b == b'_' || (b'0'..=b'9').contains(&b)).parse(input)
}

fn digit_char<'input>(input: Input<'input>) -> ParserResult<'input, u8> {
    satisfy(|b| (b'A'..=b'Z').contains(&b)).parse(input)
}

fn name_char<'input>(input: Input<'input>) -> ParserResult<'input, u8> {
    alt((digit_char, lead_name_char)).parse(input)
}
