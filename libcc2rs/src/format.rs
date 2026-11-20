// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use sprintf::parser::{
    ConversionSpecifier, ConversionType, FormatElement, NumericParam, parse_format_string,
};
use sprintf::{Printf, vsprintfp};

use crate::va_args::{VaArg, VaArgGet};

enum Arg {
    Int(i32),
    UInt(u32),
    Long(i64),
    ULong(u64),
    Size(usize),
    Double(f64),
}

impl Arg {
    fn as_printf(&self) -> &dyn Printf {
        match self {
            Arg::Int(v) => v,
            Arg::UInt(v) => v,
            Arg::Long(v) => v,
            Arg::ULong(v) => v,
            Arg::Size(v) => v,
            Arg::Double(v) => v,
        }
    }
}

// Parses one conversion specification, like "%-5ld". Its bytes are ASCII
// unless the format is invalid.
fn parse_spec(spec: &[u8]) -> ConversionSpecifier {
    if let Ok(text) = std::str::from_utf8(spec)
        && let Ok(elements) = parse_format_string(text)
        && let [FormatElement::Format(spec)] = elements[..]
        && spec.width != NumericParam::FromArgument
        && spec.precision != NumericParam::FromArgument
    {
        return spec;
    }
    panic!("format_c: cannot parse {:?}", String::from_utf8_lossy(spec));
}

// Writes `s` padded with spaces to the width of `spec`.
fn pad(out: &mut Vec<u8>, spec: &ConversionSpecifier, s: &[u8]) {
    let NumericParam::Literal(width) = spec.width else {
        unreachable!()
    };
    let fill = (width as usize).saturating_sub(s.len());
    if !spec.left_adj {
        out.resize(out.len() + fill, b' ');
    }
    out.extend_from_slice(s);
    if spec.left_adj {
        out.resize(out.len() + fill, b' ');
    }
}

fn numeric_arg(conversion_type: ConversionType, arg: &VaArg) -> Arg {
    match conversion_type {
        ConversionType::DecInt => match arg {
            VaArg::Int(v) => Arg::Int(*v),
            VaArg::UInt(v) => Arg::UInt(*v),
            VaArg::Long(v) => Arg::Long(*v),
            VaArg::ULong(v) => Arg::ULong(*v),
            _ => panic!("format_c: integer conversion expects an integer argument"),
        },
        ConversionType::OctInt | ConversionType::HexIntLower | ConversionType::HexIntUpper => {
            match arg {
                VaArg::Int(v) => Arg::UInt(*v as u32),
                VaArg::UInt(v) => Arg::UInt(*v),
                VaArg::Long(v) => Arg::ULong(*v as u64),
                VaArg::ULong(v) => Arg::ULong(*v),
                VaArg::Ptr(p) => Arg::Size(p.to_int()),
                VaArg::RawPtr(v) => Arg::Size(*v as usize),
                VaArg::Double(_) | VaArg::Record(_) => {
                    panic!("format_c: integer conversion expects an integer argument")
                }
            }
        }
        _ => Arg::Double(f64::get(arg)),
    }
}

// Formats like C's printf. The format, the strings and the result are bytes,
// which need not be UTF-8, so %s and %c are formatted here; sprintf formats
// the numbers, which are ASCII.
pub fn format_c(fmt: &[u8], va: &[VaArg]) -> Vec<u8> {
    let mut out = Vec::with_capacity(fmt.len());
    let mut va = va.iter();
    let mut rest = fmt;
    while let Some(start) = rest.iter().position(|&b| b == b'%') {
        out.extend_from_slice(&rest[..start]);
        rest = &rest[start..];
        // The flags, width, precision and length precede the conversion.
        let len = 2 + rest[1..]
            .iter()
            .take_while(|b| b"#- +*.0123456789hlLqjzZt".contains(b))
            .count();
        let spec = parse_spec(&rest[..len.min(rest.len())]);
        rest = &rest[len..];
        if spec.conversion_type == ConversionType::PercentSign {
            out.push(b'%');
            continue;
        }
        let arg = va.next().expect("format_c: missing argument");
        match spec.conversion_type {
            ConversionType::Char => pad(&mut out, &spec, &[i32::get(arg) as u8]),
            ConversionType::String => {
                let VaArg::Ptr(p) = arg else {
                    panic!("format_c: %s expects a string argument")
                };
                let NumericParam::Literal(precision) = spec.precision else {
                    unreachable!()
                };
                p.reinterpret_cast::<i8>()
                    .with_c_bytes(|s| pad(&mut out, &spec, &s[..s.len().min(precision as usize)]))
            }
            conversion_type => {
                let arg = numeric_arg(conversion_type, arg);
                match vsprintfp(&[FormatElement::Format(spec)], &[arg.as_printf()]) {
                    Ok(s) => out.extend_from_slice(s.as_bytes()),
                    Err(e) => panic!("format_c: cannot format {spec:?}: {e:?}"),
                }
            }
        }
    }
    out.extend_from_slice(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Ptr;

    fn s(lit: &'static [u8]) -> VaArg {
        Ptr::<i8>::from_string_literal(lit).into()
    }

    #[test]
    fn integers() {
        let va: Vec<VaArg> = vec![(-5i32).into(), 7u32.into(), (-9i64).into(), u64::MAX.into()];
        assert_eq!(
            format_c(b"%d %u %ld %lu", &va),
            b"-5 7 -9 18446744073709551615"
        );
        let va: Vec<VaArg> = vec![(-1i32).into(), 255u32.into(), 8i64.into(), 255u64.into()];
        assert_eq!(format_c(b"%x %X %lo %lx", &va), b"ffffffff FF 10 ff");
        assert_eq!(
            format_c(b"[%5d|%-5d|%05d]", &[1i32.into(), 2i32.into(), 3i32.into()]),
            b"[    1|2    |00003]"
        );
    }

    #[test]
    fn strings_chars_and_floats() {
        let va: Vec<VaArg> = vec![s(b"abc"), (b'z' as i32).into(), 2.5f64.into()];
        assert_eq!(format_c(b"%s %c %.2f", &va), b"abc z 2.50");
        assert_eq!(
            format_c(b"%8s|%-8s|", &[s(b"hi"), s(b"yo")]),
            b"      hi|yo      |"
        );
        assert_eq!(format_c(b"%e", &[1500.0f64.into()]), b"1.500000e+03");
        assert_eq!(format_c(b"%g", &[0.5f64.into()]), b"0.5");
    }

    #[test]
    fn bytes_need_not_be_utf8() {
        // Widths and precisions count bytes, and invalid UTF-8 is copied.
        let va: Vec<VaArg> = vec![
            s(b"a\xe9"),
            s(b"b\xc3\xa9"),
            s(b"\xc3\xa9"),
            (0xe9i32).into(),
        ];
        assert_eq!(
            format_c(b"\xff[%s|%5s|%.1s|%-2c]", &va),
            b"\xff[a\xe9|  b\xc3\xa9|\xc3|\xe9 ]"
        );
    }

    #[test]
    fn percent_signs_consume_no_argument() {
        assert_eq!(format_c(b"100%% %d%%", &[3i32.into()]), b"100% 3%");
        assert_eq!(format_c(b"no conversions", &[]), b"no conversions");
    }

    #[test]
    fn raw_pointers_as_hex() {
        let va: Vec<VaArg> = vec![
            VaArg::RawPtr(std::ptr::null_mut()),
            VaArg::RawPtr(0x10 as *mut _),
        ];
        assert_eq!(format_c(b"%x %x", &va), b"0 10");
    }

    #[test]
    #[should_panic(expected = "integer conversion expects an integer argument")]
    fn wrong_argument_type_panics() {
        format_c(b"%d", &[1.5f64.into()]);
    }
}
