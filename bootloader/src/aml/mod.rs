pub mod parser;

use crate::sprintln;

pub fn parse_definition_block(mut input: &[u8]) {
    // let mut offset = 0;
    // take many of some things
    loop {
        let (new_input, ty) = parse_block_type(input);
        input = new_input;
        // sprintln!("Block type {:x} at offset {:x}", ty, offset);
        match ty {
            0x10 => {
                let (new_input, _) = parse_def_scope(input);
                input = new_input;
            }
            _ => {
                panic!("unknown block type {:x}", ty);
            }
        }
    }
}

fn parse_block_type(input: &[u8]) -> ParseResult<u8> {
    (&input[1..], input[0])
}

fn parse_def_scope(input: &[u8]) -> ParseResult<()> {
    let (input, pkg_length) = parse_pkg_length(input);
    parse_name_string(input);
    (&input[pkg_length..], ())
}

fn parse_pkg_length(input: &[u8]) -> ParseResult<usize> {
    let extra_bytes_count = (input[0] >> 6) as usize;
    let pkg_length = if extra_bytes_count == 0 {
        input[0] as usize
    } else {
        assert_eq!(input[0] & 0b0011_0000, 0);
        let mut pkg_length = (input[0] & 0xf) as usize;
        for i in 1..=extra_bytes_count {
            pkg_length |= (input[i] as usize) << (i * 8 - 4);
        }

        pkg_length
    };
    (
        &input[extra_bytes_count + 1..],
        pkg_length - (extra_bytes_count + 1),
    )
}

fn parse_name_string(input: &[u8]) -> ParseResult<()> {
    if let Some((input, _)) = parse_root_char(input) {
        return parse_name_path(input);
    } else {
        let (input, _) = parse_prefix_path(input);
        return parse_name_path(input);
    }
}

fn parse_root_char(input: &[u8]) -> Option<ParseResult<()>> {
    if input[0] == b'\\' {
        return Some((&input[1..], ()));
    }

    None
}

fn parse_prefix_path(mut input: &[u8]) -> ParseResult<usize> {
    let mut i = 0;
    loop {
        if input[i] == b'^' {
            i += 1;
            input = &input[1..];
            continue;
        }

        break;
    }

    (input, i)
}

fn parse_name_path(input: &[u8]) -> ParseResult<()> {
    if let Some((input, name_seg)) = parse_name_seg(input) {
        sprintln!("{}", core::str::from_utf8(&name_seg).unwrap());
        return (input, ());
    }

    if let Some((input, name_seg)) = parse_dual_name_path(input) {
        sprintln!("{}", core::str::from_utf8(&name_seg).unwrap());
        return (input, ());
    }

    if let Some((input, _)) = parse_null_name(input) {
        return (input, ());
    }

    unimplemented!();
}

fn parse_dual_name_path(mut input: &[u8]) -> Option<ParseResult<[u8; 8]>> {
    let mut total_bytes_read = 1;
    if input[0] != b'.' {
        return None;
    }

    input = &input[1..];

    let mut name_path = [0u8; 8];
    if let Some((new_input, name_seg)) = parse_name_seg(input) {
        name_path[0] = name_seg[0];
        name_path[1] = name_seg[1];
        name_path[2] = name_seg[2];
        name_path[3] = name_seg[3];
        input = new_input;
    }

    if let Some((new_input, name_seg)) = parse_name_seg(input) {
        name_path[4] = name_seg[0];
        name_path[5] = name_seg[1];
        name_path[6] = name_seg[2];
        name_path[7] = name_seg[3];
        input = new_input;
    }

    Some((input, name_path))
}

fn parse_null_name(input: &[u8]) -> Option<ParseResult<()>> {
    if input[0] == 0 {
        return Some((&input[1..], ()));
    }

    None
}

fn parse_name_seg(input: &[u8]) -> Option<ParseResult<[u8; 4]>> {
    let mut name_seg = [0; 4];

    let (input, char) = parse_lead_name_char(input)?;
    name_seg[0] = char;
    let (input, char) = parse_name_char(input)?;
    name_seg[1] = char;
    let (input, char) = parse_name_char(input)?;
    name_seg[2] = char;
    let (input, char) = parse_name_char(input)?;
    name_seg[3] = char;

    Some((input, name_seg))
}

fn parse_lead_name_char(input: &[u8]) -> Option<ParseResult<u8>> {
    let b = input[0];
    if b == b'_' || (b >= b'A' && b <= b'Z') {
        return Some((&input[1..], b));
    }

    None
}

fn parse_digit_char(input: &[u8]) -> Option<ParseResult<u8>> {
    // What do we want to do? We want to take one byte, and then check if it's between 0 and 9
    let b = input[0];
    if b >= b'0' && b <= b'9' {
        return Some((&input[1..], b));
    }

    None
}

fn parse_name_char(input: &[u8]) -> Option<ParseResult<u8>> {
    if let Some(r) = parse_digit_char(input) {
        return Some(r);
    };

    parse_lead_name_char(input)
}

type ParseResult<'a, T> = (&'a [u8], T);
