// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use sprintf::parser::{ConversionType, FormatElement, parse_format_string};
use sprintf::{Printf, vsprintfp};

use crate::va_args::{VaArg, VaArgGet};

enum Arg {
    Int(i32),
    UInt(u32),
    Long(i64),
    ULong(u64),
    Size(usize),
    Char(char),
    Str(String),
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
            Arg::Char(v) => v,
            Arg::Str(v) => v,
            Arg::Double(v) => v,
        }
    }
}

pub fn format_c(fmt: &str, va: &[VaArg]) -> String {
    let elements = match parse_format_string(fmt) {
        Ok(elements) => elements,
        Err(e) => panic!("format_c: cannot parse {fmt:?}: {e:?}"),
    };
    let mut args: Vec<Arg> = Vec::with_capacity(va.len());
    let mut pos = 0;
    for element in &elements {
        if let FormatElement::Format(spec) = element {
            if spec.conversion_type == ConversionType::PercentSign {
                continue;
            }
            let arg = &va[pos];
            args.push(match spec.conversion_type {
                ConversionType::DecInt => match arg {
                    VaArg::Int(v) => Arg::Int(*v),
                    VaArg::UInt(v) => Arg::UInt(*v),
                    VaArg::Long(v) => Arg::Long(*v),
                    VaArg::ULong(v) => Arg::ULong(*v),
                    _ => panic!("format_c: integer conversion expects an integer argument"),
                },
                ConversionType::OctInt
                | ConversionType::HexIntLower
                | ConversionType::HexIntUpper => match arg {
                    VaArg::Int(v) => Arg::UInt(*v as u32),
                    VaArg::UInt(v) => Arg::UInt(*v),
                    VaArg::Long(v) => Arg::ULong(*v as u64),
                    VaArg::ULong(v) => Arg::ULong(*v),
                    VaArg::Ptr(p) => Arg::Size(p.to_int()),
                    VaArg::RawPtr(v) => Arg::Size(*v as usize),
                    VaArg::Double(_) => {
                        panic!("format_c: integer conversion expects an integer argument")
                    }
                },
                ConversionType::Char => Arg::Char(i32::get(arg) as u8 as char),
                ConversionType::String => match arg {
                    VaArg::Ptr(v) => Arg::Str(v.reinterpret_cast::<u8>().to_rust_string()),
                    _ => panic!("format_c: %s expects a string argument"),
                },
                ConversionType::DecFloatLower
                | ConversionType::DecFloatUpper
                | ConversionType::SciFloatLower
                | ConversionType::SciFloatUpper
                | ConversionType::CompactFloatLower
                | ConversionType::CompactFloatUpper => Arg::Double(f64::get(arg)),
                ConversionType::PercentSign => panic!("format_c: %% consumes no argument"),
            });
            pos += 1;
        }
    }
    let refs: Vec<&dyn Printf> = args.iter().map(Arg::as_printf).collect();
    match vsprintfp(&elements, &refs) {
        Ok(s) => s,
        Err(e) => panic!("format_c: cannot format {fmt:?}: {e:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Ptr;

    fn s(lit: &'static [u8]) -> VaArg {
        Ptr::<u8>::from_string_literal(lit).into()
    }

    #[test]
    fn integers() {
        let va: Vec<VaArg> = vec![(-5i32).into(), 7u32.into(), (-9i64).into(), u64::MAX.into()];
        assert_eq!(
            format_c("%d %u %ld %lu", &va),
            "-5 7 -9 18446744073709551615"
        );
        let va: Vec<VaArg> = vec![(-1i32).into(), 255u32.into(), 8i64.into(), 255u64.into()];
        assert_eq!(format_c("%x %X %lo %lx", &va), "ffffffff FF 10 ff");
        assert_eq!(
            format_c("[%5d|%-5d|%05d]", &[1i32.into(), 2i32.into(), 3i32.into()]),
            "[    1|2    |00003]"
        );
    }

    #[test]
    fn strings_chars_and_floats() {
        let va: Vec<VaArg> = vec![s(b"abc"), (b'z' as i32).into(), 2.5f64.into()];
        assert_eq!(format_c("%s %c %.2f", &va), "abc z 2.50");
        assert_eq!(
            format_c("%8s|%-8s|", &[s(b"hi"), s(b"yo")]),
            "      hi|yo      |"
        );
        assert_eq!(format_c("%e", &[1500.0f64.into()]), "1.500000e+03");
        assert_eq!(format_c("%g", &[0.5f64.into()]), "0.5");
    }

    #[test]
    fn percent_signs_consume_no_argument() {
        assert_eq!(format_c("100%% %d%%", &[3i32.into()]), "100% 3%");
        assert_eq!(format_c("no conversions", &[]), "no conversions");
    }

    #[test]
    fn raw_pointers_as_hex() {
        let va: Vec<VaArg> = vec![
            VaArg::RawPtr(std::ptr::null_mut()),
            VaArg::RawPtr(0x10 as *mut _),
        ];
        assert_eq!(format_c("%x %x", &va), "0 10");
    }

    #[test]
    #[should_panic(expected = "integer conversion expects an integer argument")]
    fn wrong_argument_type_panics() {
        format_c("%d", &[1.5f64.into()]);
    }
}
