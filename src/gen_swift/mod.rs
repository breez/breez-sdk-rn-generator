use std::cell::RefCell;
use std::collections::BTreeSet;

use askama::Template;
use uniffi_bindgen::interface::*;

pub use uniffi_bindgen::bindings::kotlin::gen_kotlin::*;

use crate::generator::RNConfig;

#[derive(Template)]
#[template(syntax = "rn", escape = "none", path = "wrapper.swift")]
pub struct Generator<'a> {
    config: RNConfig,
    ci: &'a ComponentInterface,
    // Track types used in sequences with the `add_sequence_type()` macro
    sequence_types: RefCell<BTreeSet<String>>,
}

impl<'a> Generator<'a> {
    pub fn new(config: RNConfig, ci: &'a ComponentInterface) -> Self {
        Self {
            config,
            ci,
            sequence_types: RefCell::new(BTreeSet::new()),
        }
    }
}

pub mod filters {
    use std::{option, fmt::format};

    use heck::*;
    use uniffi_bindgen::{backend::{CodeType, TypeIdentifier}, bindings::swift::gen_swift::SwiftCodeOracle};

    use super::*;

    fn oracle() -> &'static SwiftCodeOracle {
        &SwiftCodeOracle
    }

    pub fn type_name(codetype: &impl CodeType) -> Result<String, askama::Error> {
        Ok(codetype.type_label(oracle()))
    }

    pub fn render_to_map(
        t: &TypeIdentifier,
        ci: &ComponentInterface,
        obj_name: &str,
        field_name: &str,
        optional: bool,
    ) -> Result<String, askama::Error> {
        let type_name = filters::type_name(t)?;
        let type_name_str = type_name.as_str();
        let var_name = filters::unquote(filters::var_name(type_name_str)?.as_str())?;
        let mut obj_prefix = "".to_string();
        if !obj_name.is_empty() {
            obj_prefix = format!("{obj_name}.");
        }
        let mut optional_suffix = "";
        if optional {
            optional_suffix = "!";
        }
        let res: Result<String, askama::Error> = match t {
            Type::UInt8 => Ok(format!("{obj_prefix}{field_name}").into()),
            Type::Int8 => Ok(format!("{obj_prefix}{field_name}").into()),
            Type::UInt16 => Ok(format!("{obj_prefix}{field_name}").into()),
            Type::Int16 => Ok(format!("{obj_prefix}{field_name}").into()),
            Type::UInt32 => Ok(format!("{obj_prefix}{field_name}").into()),
            Type::Int32 => Ok(format!("{obj_prefix}{field_name}").into()),
            Type::UInt64 => Ok(format!("{obj_prefix}{field_name}").into()),
            Type::Int64 => Ok(format!("{obj_prefix}{field_name}").into()),
            Type::Float32 => Ok(format!("{obj_prefix}{field_name}").into()),
            Type::Float64 => Ok(format!("{obj_prefix}{field_name}").into()),
            Type::Boolean => Ok(format!("{obj_prefix}{field_name}").into()),
            Type::String => Ok(format!("{obj_prefix}{field_name}").into()),
            Type::Timestamp => unimplemented!("render_to_map: Timestamp is not implemented"),
            Type::Duration => unimplemented!("render_to_map: Duration is not implemented"),
            Type::Object(_) => unimplemented!("render_to_map: Object is not implemented"),
            Type::Record(_) => match optional {
                true => Ok(format!("{obj_prefix}{field_name} == nil ? nil : {{ dictionaryOf({var_name}: {obj_prefix}{field_name}!) }}").into()),
                false => Ok(format!("dictionaryOf({var_name}: {obj_prefix}{field_name})").into()),
            },
            Type::Enum(inner) => {
                let enum_def = ci.get_enum_definition(inner).unwrap();
                //let type_name = enum_def
                match enum_def.is_flat() {
                    true => match optional {
                        true => Ok(format!(
                            "valueOf( {var_name}:  {obj_prefix}{field_name}!)"
                        )
                        .into()),
                        false => Ok(format!("valueOf( {var_name}: {obj_prefix}{field_name})").into()),
                    },
                    false => match optional {
                        true => Ok(
                            format!("dictionaryOf({var_name}: {obj_prefix}{field_name}!)").into(),
                        ),
                        false => Ok(format!("dictionaryOf({var_name}: {obj_prefix}{field_name})").into()),
                    },
                }
            }
            Type::Error(_) => unimplemented!("render_to_map: Error is not implemented"),
            Type::CallbackInterface(_) => {
                unimplemented!("render_to_map: CallbackInterface is not implemented")
            }
            Type::Optional(inner) => {
                let unboxed = inner.as_ref();
                let inner_render = render_to_map(unboxed, ci, obj_name, field_name, true)?;
                Ok(format!("{obj_prefix}{field_name} == nil ? nil : {inner_render}"))
            }
            Type::Sequence(inner) => 
            {
                let unboxed = inner.as_ref();
                let type_name = filters::type_name(unboxed)?;
                let var_name = filters::var_name(type_name.as_str())?;
                let var_name = filters::unquote(var_name.as_str())?;
                let as_array_statment = match unboxed {                    
                    Type::Record(_) => format!("arrayOf({var_name}s: {obj_prefix}{field_name}{optional_suffix})"),
                    Type::Enum(_) => format!("arrayOf({var_name}s: {obj_prefix}{field_name}{optional_suffix})"),
                    _ => format!("{obj_prefix}{field_name} as? [{type_name}]")
                };
                Ok(as_array_statment)
            },
            Type::Map(_, _) => unimplemented!("render_to_map: Map is not implemented"),
            Type::External { .. } => {
                unimplemented!("render_to_map: External is not implemented")
            }
            Type::Custom { .. } => {
                unimplemented!("render_to_map: Custom is not implemented")
            }
            Type::Unresolved { .. } => {
                unimplemented!("render_to_map: Unresolved is not implemented")
            }
        };
        res
    }

    pub fn render_from_map(
        t: &TypeIdentifier,
        ci: &ComponentInterface,
        name: &str,
        optional: bool,
    ) -> Result<String, askama::Error> {        
        let res: String = match t {
            Type::UInt8 => format!("data[\"{name}\"] as! UInt8").into(),
            Type::Int8 => format!("data[\"{name}\"] as! Int8").into(),
            Type::UInt16 => format!("data[\"{name}\"] as! UInt16").into(),
            Type::Int16 => format!("data[\"{name}\"] as! .Int16").into(),
            Type::UInt32 => format!("data[\"{name}\"] as! UInt32").into(),
            Type::Int32 => format!("data[\"{name}\"] as! Int32").into(),
            Type::UInt64 => format!("data[\"{name}\"] as! UInt64").into(),
            Type::Int64 => format!("data[\"{name}\"] as! Int64").into(),
            Type::Float32 => format!("data[\"{name}\"] as! Double").into(),
            Type::Float64 => format!("data[\"{name}\"] as! Double").into(),
            Type::Boolean => format!("data[\"{name}\"] as! Bool").into(),
            Type::String => format!("data[\"{name}\"] as! String").into(),
            Type::Timestamp => "".into(),
            Type::Duration => "".into(),
            Type::Object(_) => "".into(),
            Type::Record(_) => {
                let record_type_name = type_name(t)?;
                format!("try as{record_type_name}(data: data[\"{name}\"] as! [String: Any?])")
            }
            Type::Enum(inner) => {
                let enum_def = ci.get_enum_definition(inner).unwrap();
                match enum_def.is_flat() {
                    false => {
                        format!("try as{inner}(data: data[\"{name}\"] as! [String: Any?])")
                            .into()
                    }
                    true => format!(
                        "try as{inner}(type: data[\"{name}\"] as! String)"
                    )
                    .into(),
                }
            }
            Type::Error(_) => "".into(),
            Type::CallbackInterface(_) => "".into(),
            Type::Optional(inner) => {
                let unboxed = inner.as_ref();
                let inner_res = render_from_map(unboxed, ci, name, true)?;                
                format!("data[\"{name}\"] == nil ? nil : {inner_res}")
            }
            Type::Sequence(inner) => {
                let unboxed = inner.as_ref();
                let element_type_name = type_name(unboxed)?;
                let camel_case_element = element_type_name.to_lower_camel_case();
                match unboxed {
                    Type::Record(_) => format!("try as{element_type_name}List(arr: data[\"{name}\"] as! [Any])"),                    
                    _ => format!("data[\"{name}\"] as! [{element_type_name}]").into(),
                }                
            }
            Type::Map(_, _) => "".into(),
            Type::External { .. } => "".into(),
            Type::Custom { .. } => "".into(),
            Type::Unresolved { .. } => "".into(),
        };
        Ok(res.to_string())
    }

    /// Get the idiomatic Kotlin rendering of a variable name.
    pub fn var_name(nm: &str) -> Result<String, askama::Error> {
        Ok(format!("`{}`", nm.to_string().to_lower_camel_case()))
    }

    pub fn unquote(nm: &str) -> Result<String, askama::Error> {
        Ok(nm.trim_matches('`').to_string())
    }
}