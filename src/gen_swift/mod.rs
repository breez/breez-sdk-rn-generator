use std::collections::HashSet;

use askama::Template;
use once_cell::sync::Lazy;
use uniffi_bindgen::interface::*;

use crate::generator::RNConfig;

pub use uniffi_bindgen::bindings::swift::gen_swift::*;

static IGNORED_FUNCTIONS: Lazy<HashSet<String>> = Lazy::new(|| {
    let list = vec!["connect", "default_config", "set_log_stream"];
    HashSet::from_iter(list.into_iter().map(|s| s.to_string()))
});

#[derive(Template)]
#[template(syntax = "rn", escape = "none", path = "mapper.swift")]
pub struct MapperGenerator<'a> {
    config: RNConfig,
    ci: &'a ComponentInterface,
}

impl<'a> MapperGenerator<'a> {
    pub fn new(config: RNConfig, ci: &'a ComponentInterface) -> Self {
        Self { config, ci }
    }
}

#[derive(Template)]
#[template(syntax = "rn", escape = "none", path = "module.swift")]
pub struct ModuleGenerator<'a> {
    config: RNConfig,
    ci: &'a ComponentInterface,
}

impl<'a> ModuleGenerator<'a> {
    pub fn new(config: RNConfig, ci: &'a ComponentInterface) -> Self {
        Self { config, ci }
    }
}

pub mod filters {

    use heck::*;
    use uniffi_bindgen::backend::CodeOracle;
    use uniffi_bindgen::backend::{CodeType, TypeIdentifier};

    use super::*;

    fn oracle() -> &'static SwiftCodeOracle {
        &SwiftCodeOracle
    }

    pub fn type_name(codetype: &impl CodeType) -> Result<String, askama::Error> {
        Ok(codetype.type_label(oracle()))
    }

    pub fn fn_name(nm: &str) -> Result<String, askama::Error> {
        Ok(oracle().fn_name(nm))
    }

    pub fn render_literal(
        literal: &Literal,
        codetype: &impl CodeType,
    ) -> Result<String, askama::Error> {
        Ok(codetype.literal(oracle(), literal))
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
            Type::Sequence(inner) => {
                let unboxed = inner.as_ref();
                let type_name = filters::type_name(unboxed)?;
                let var_name = filters::var_name(type_name.as_str())?;
                let var_name = filters::unquote(var_name.as_str())?;
                let as_array_statment = match unboxed {
                    Type::Record(_) => format!("arrayOf({var_name}List: {obj_prefix}{field_name}{optional_suffix})"),
                    Type::Enum(_) => format!("arrayOf({var_name}List: {obj_prefix}{field_name}{optional_suffix})"),
                    _ => format!("{obj_prefix}{field_name}")
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

    pub fn rn_convert_type(
        t: &TypeIdentifier,
        converted_var_name: &str,
    ) -> Result<String, askama::Error> {
        match t {
            Type::Optional(inner) => {
                let unboxed = inner.as_ref();
                let optional = match *unboxed {
                    Type::Int8
                    | Type::UInt8
                    | Type::Int16
                    | Type::UInt16
                    | Type::Int32
                    | Type::UInt32
                    | Type::Int64
                    | Type::UInt64 => format!(
                        "({} == 0 ? nil : {})",
                        converted_var_name, converted_var_name
                    ),
                    Type::Float32 | Type::Float64 => format!(
                        "({} == 0.0 ? nil : {})",
                        converted_var_name, converted_var_name
                    ),
                    Type::String => format!(
                        "({}.isEmpty ? nil : {})",
                        converted_var_name, converted_var_name
                    ),
                    _ => "".to_string(),
                };
                Ok(optional.to_string())
            }
            _ => Ok(converted_var_name.to_string()),
        }
    }

    pub fn rn_return_type(
        t: &TypeIdentifier,
        name: &str,
        optional: bool,
    ) -> Result<String, askama::Error> {
        let mut optional_suffix = "";
        if optional {
            optional_suffix = "!";
        }
        match t {
            Type::Enum(_) | Type::Record(_) => Ok(format!(
                "BreezSDKMapper.dictionaryOf({}: res{})",
                name, optional_suffix
            )),
            Type::Sequence(inner) => {
                let unboxed = inner.as_ref();
                match unboxed {
                    Type::Enum(_) | Type::Record(_) => Ok(format!(
                        "BreezSDKMapper.arrayOf({}List: res{})",
                        name, optional_suffix
                    )),
                    _ => Ok(format!("res{}", optional_suffix)),
                }
            }
            _ => Ok(format!("res{}", optional_suffix)),
        }
    }

    pub fn rn_type_name(
        t: &TypeIdentifier,
        ci: &ComponentInterface,
        optional: bool,
    ) -> Result<String, askama::Error> {
        let mut optional_suffix = "";
        if optional {
            optional_suffix = "?";
        }
        match t {
            Type::Record(_) => Ok(format!("[String: Any{}]", optional_suffix)),
            Type::Enum(inner) => {
                let enum_def = ci.get_enum_definition(inner).unwrap();
                match enum_def.is_flat() {
                    false => Ok(format!("[String: Any{}]", optional_suffix)),
                    true => Ok("String".into()),
                }
            }
            Type::Optional(inner) => {
                let unboxed = inner.as_ref();
                rn_type_name(unboxed, ci, optional)
            }
            Type::Sequence(inner) => {
                let unboxed = inner.as_ref();
                Ok(format!("[{}]", rn_type_name(unboxed, ci, optional)?))
            }
            t => {
                let name = filters::type_name(t)?;
                Ok(format!("{name}"))
            }
        }
    }

    pub fn inline_optional_field(
        t: &TypeIdentifier,
        ci: &ComponentInterface,
    ) -> Result<bool, askama::Error> {
        match t {
            Type::Optional(inner) => {
                let unboxed = inner.as_ref();
                inline_optional_field(unboxed, ci)
            }
            _ => {
                let mapped_name = filters::rn_type_name(t, ci, true)?;
                let type_name = filters::type_name(t)?;
                Ok(mapped_name == type_name)
            }
        }
    }

    pub fn render_from_map(
        t: &TypeIdentifier,
        ci: &ComponentInterface,
        map_var_name: &str,
    ) -> Result<String, askama::Error> {
        let res: String = match t {
            Type::UInt8 => format!("{map_var_name}").into(),
            Type::Int8 => format!("{map_var_name}").into(),
            Type::UInt16 => format!("{map_var_name}").into(),
            Type::Int16 => format!("{map_var_name}").into(),
            Type::UInt32 => format!("{map_var_name}").into(),
            Type::Int32 => format!("{map_var_name}").into(),
            Type::UInt64 => format!("{map_var_name}").into(),
            Type::Int64 => format!("{map_var_name}").into(),
            Type::Float32 => format!("{map_var_name}").into(),
            Type::Float64 => format!("{map_var_name}").into(),
            Type::Boolean => format!("{map_var_name}").into(),
            Type::String => format!("{map_var_name}").into(),
            Type::Timestamp => "".into(),
            Type::Duration => "".into(),
            Type::Object(_) => "".into(),
            Type::Record(_) => {
                let record_type_name = type_name(t)?;
                format!("try as{record_type_name}(data: {map_var_name})")
            }
            Type::Enum(inner) => {
                let enum_def = ci.get_enum_definition(inner).unwrap();
                match enum_def.is_flat() {
                    false => format!("try as{inner}(data: {map_var_name})").into(),
                    true => format!("try as{inner}(type: {map_var_name})").into(),
                }
            }
            Type::Error(_) => "".into(),
            Type::CallbackInterface(_) => "".into(),
            Type::Optional(inner) => {
                let unboxed = inner.as_ref();
                let inner_res = render_from_map(unboxed, ci, map_var_name)?;
                inner_res
            }
            Type::Sequence(inner) => {
                let unboxed = inner.as_ref();
                let element_type_name = type_name(unboxed)?;
                match unboxed {
                    Type::Record(_) => {
                        format!("try as{element_type_name}List(arr: {map_var_name})")
                    }
                    _ => format!("{map_var_name}").into(),
                }
            }
            Type::Map(_, _) => "".into(),
            Type::External { .. } => "".into(),
            Type::Custom { .. } => "".into(),
            Type::Unresolved { .. } => "".into(),
        };
        Ok(res.to_string())
    }

    pub fn var_name(nm: &str) -> Result<String, askama::Error> {
        Ok(format!("`{}`", nm.to_string().to_lower_camel_case()))
    }

    pub fn unquote(nm: &str) -> Result<String, askama::Error> {
        Ok(nm.trim_matches('`').to_string())
    }

    pub fn ignored_function(nm: &str) -> Result<bool, askama::Error> {
        Ok(IGNORED_FUNCTIONS.contains(nm))
    }

    pub fn list_arg(nm: &str) -> Result<String, askama::Error> {
        Ok(format!("`{nm}List`"))
    }

    pub fn temporary(nm: &str) -> Result<String, askama::Error> {
        Ok(format!("{nm}Tmp"))
    }
}
