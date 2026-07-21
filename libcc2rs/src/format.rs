// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use sprintf::parser::{ConversionType, FormatElement, parse_format_string};
use sprintf::{Printf, vsprintfp};

use crate::va_args::{VaArg, VaArgGet};

pub fn format_c(fmt: &str, va: &[VaArg]) -> String {
    let elements = match parse_format_string(fmt) {
        Ok(elements) => elements,
        Err(e) => panic!("format_c: cannot parse {fmt:?}: {e:?}"),
    };
    let mut args: Vec<Box<dyn Printf>> = Vec::new();
    let mut pos = 0;
    for element in &elements {
        if let FormatElement::Format(spec) = element {
            if spec.conversion_type == ConversionType::PercentSign {
                continue;
            }
            let arg = &va[pos];
            args.push(match spec.conversion_type {
                ConversionType::DecInt => match arg {
                    VaArg::Int(v) => Box::new(*v),
                    VaArg::UInt(v) => Box::new(*v),
                    VaArg::Long(v) => Box::new(*v),
                    VaArg::ULong(v) => Box::new(*v),
                    _ => panic!("format_c: integer conversion expects an integer argument"),
                },
                ConversionType::OctInt
                | ConversionType::HexIntLower
                | ConversionType::HexIntUpper => match arg {
                    VaArg::Int(v) => Box::new(*v as u32),
                    VaArg::UInt(v) => Box::new(*v),
                    VaArg::Long(v) => Box::new(*v as u64),
                    VaArg::ULong(v) => Box::new(*v),
                    VaArg::Ptr(p) => Box::new(p.to_int()),
                    VaArg::RawPtr(v) => Box::new(*v as usize),
                    VaArg::Double(_) => {
                        panic!("format_c: integer conversion expects an integer argument")
                    }
                },
                ConversionType::Char => Box::new(i32::get(arg) as u8 as char),
                ConversionType::String => match arg {
                    VaArg::Ptr(v) => Box::new(v.reinterpret_cast::<u8>().to_rust_string()),
                    _ => panic!("format_c: %s expects a string argument"),
                },
                ConversionType::DecFloatLower
                | ConversionType::DecFloatUpper
                | ConversionType::SciFloatLower
                | ConversionType::SciFloatUpper
                | ConversionType::CompactFloatLower
                | ConversionType::CompactFloatUpper => Box::new(f64::get(arg)),
                ConversionType::PercentSign => panic!("format_c: %% consumes no argument"),
            });
            pos += 1;
        }
    }
    let refs: Vec<&dyn Printf> = args.iter().map(|arg| arg.as_ref()).collect();
    match vsprintfp(&elements, &refs) {
        Ok(s) => s,
        Err(e) => panic!("format_c: cannot format {fmt:?}: {e:?}"),
    }
}
