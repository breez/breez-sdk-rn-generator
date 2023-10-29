use std::cell::RefCell;
use std::collections::{BTreeSet, HashSet};

use askama::Template;
use once_cell::sync::Lazy;
use uniffi_bindgen::interface::*;

pub use uniffi_bindgen::bindings::kotlin::gen_kotlin::*;

use crate::generator::RNConfig;

static IGNORED_FUNCTIONS: Lazy<HashSet<String>> = Lazy::new(|| {
    let list = vec!["connect", "set_log_stream"];
    HashSet::from_iter(list.into_iter().map(|s| s.to_string()))
});

#[derive(Template)]
#[template(syntax = "rn", escape = "none", path = "mapper.kt")]
pub struct MapperGenerator<'a> {
    config: RNConfig,
    ci: &'a ComponentInterface,
    // Track types used in sequences with the `add_sequence_type()` macro
    sequence_types: RefCell<BTreeSet<String>>,
}

impl<'a> MapperGenerator<'a> {
    pub fn new(config: RNConfig, ci: &'a ComponentInterface) -> Self {
        Self {
            config,
            ci,
            sequence_types: RefCell::new(BTreeSet::new()),
        }
    }

    // Helper to add a sequence type
    //
    // Call this inside your template to add a type used in a sequence.
    // This type is then added to the pushToArray helper.
    // Imports will be sorted and de-deuped.
    //
    // Returns an empty string so that it can be used inside an askama `{{ }}` block.
    fn add_sequence_type(&self, type_name: &str) -> &str {
        self.sequence_types
            .borrow_mut()
            .insert(type_name.to_owned());
        ""
    }

    pub fn sequence_types(&self) -> Vec<String> {
        let sequence_types = self.sequence_types.clone().into_inner();
        sequence_types.into_iter().collect()
    }
}

#[derive(Template)]
#[template(syntax = "rn", escape = "none", path = "module.kt")]
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

    fn oracle() -> &'static KotlinCodeOracle {
        &KotlinCodeOracle
    }

    pub fn type_name(codetype: &impl CodeType) -> Result<String, askama::Error> {
        Ok(codetype.type_label(oracle()))
    }

    pub fn error_name(nm: &str) -> Result<String, askama::Error> {
        Ok(oracle().error_name(nm))
    }

    pub fn fn_name(nm: &str) -> Result<String, askama::Error> {
        Ok(oracle().fn_name(nm))
    }

    pub fn render_to_array(type_name: &str, ci: &ComponentInterface) -> Result<String, askama::Error> {
        let res: Result<String, askama::Error> = match type_name {
            "Boolean" => Ok(format!("array.pushBoolean(value)").into()),
            "Double" => Ok(format!("array.pushDouble(value)").into()),
            "Int" => Ok(format!("array.pushInt(value)").into()),
            "ReadableArray" => Ok(format!("array.pushArray(value)").into()),
            "ReadableMap" => Ok(format!("array.pushMap(value)").into()),
            "String" => Ok(format!("array.pushString(value)").into()),
            "UByte" => Ok(format!("array.pushInt(value.toInt())").into()),
            "UInt" => Ok(format!("array.pushInt(value.toInt())").into()),
            "UShort" => Ok(format!("array.pushInt(value.toInt())").into()),
            "ULong" => Ok(format!("array.pushDouble(value.toDouble())").into()),
            _ => {
                match ci.get_type(type_name) {
                    Some(t) => match t {
                        Type::Enum(inner) => {
                            let enum_def = ci.get_enum_definition(&inner).unwrap();
                            match enum_def.is_flat() {
                                true => Ok(format!("array.pushString(value.name.lowercase())").into()),
                                false => Ok(format!("array.pushMap(readableMapOf(value))").into())
                            }
                        },
                        _ => Ok(format!("array.pushMap(readableMapOf(value))").into()),
                    },
                    None => unimplemented!("known type: {type_name}")
                }
            },
        };
        res
    }

    pub fn render_to_map(
        t: &TypeIdentifier,
        ci: &ComponentInterface,
        obj_name: &str,
        field_name: &str,
        optional: bool,
    ) -> Result<String, askama::Error> {
        let res: Result<String, askama::Error> = match t {
            Type::UInt8 => Ok(format!("{obj_name}.{field_name}").into()),
            Type::Int8 => Ok(format!("{obj_name}.{field_name}").into()),
            Type::UInt16 => Ok(format!("{obj_name}.{field_name}").into()),
            Type::Int16 => Ok(format!("{obj_name}.{field_name}").into()),
            Type::UInt32 => Ok(format!("{obj_name}.{field_name}").into()),
            Type::Int32 => Ok(format!("{obj_name}.{field_name}").into()),
            Type::UInt64 => Ok(format!("{obj_name}.{field_name}").into()),
            Type::Int64 => Ok(format!("{obj_name}.{field_name}").into()),
            Type::Float32 => Ok(format!("{obj_name}.{field_name}").into()),
            Type::Float64 => Ok(format!("{obj_name}.{field_name}").into()),
            Type::Boolean => Ok(format!("{obj_name}.{field_name}").into()),
            Type::String => Ok(format!("{obj_name}.{field_name}").into()),
            Type::Timestamp => unimplemented!("render_to_map: Timestamp is not implemented"),
            Type::Duration => unimplemented!("render_to_map: Duration is not implemented"),
            Type::Object(_) => unimplemented!("render_to_map: Object is not implemented"),
            Type::Record(_) => match optional {
                true => Ok(format!("{obj_name}.{field_name}?.let {{ readableMapOf(it) }}").into()),
                false => Ok(format!("readableMapOf({obj_name}.{field_name})").into()),
            },
            Type::Enum(inner) => {
                let enum_def = ci.get_enum_definition(inner).unwrap();
                match enum_def.is_flat() {
                    true => match optional {
                        true => Ok(format!(
                            "{obj_name}.{field_name}?.let {{ it.name.lowercase() }}"
                        )
                        .into()),
                        false => Ok(format!("{obj_name}.{field_name}.name.lowercase()").into()),
                    },
                    false => match optional {
                        true => Ok(
                            format!("{obj_name}.{field_name}?.let {{ readableMapOf(it) }}").into(),
                        ),
                        false => Ok(format!("readableMapOf({obj_name}.{field_name})").into()),
                    },
                }
            }
            Type::Error(_) => unimplemented!("render_to_map: Error is not implemented"),
            Type::CallbackInterface(_) => {
                unimplemented!("render_to_map: CallbackInterface is not implemented")
            }
            Type::Optional(inner) => {
                let unboxed = inner.as_ref();
                render_to_map(unboxed, ci, obj_name, field_name, true)
            }
            Type::Sequence(_) => match optional {
                true => Ok(format!(
                    "{obj_name}.{field_name}?.let {{ readableArrayOf(it) }}"
                )),
                false => Ok(format!("readableArrayOf({obj_name}.{field_name})")),
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
        field_name: &str,
        optional: bool,
    ) -> Result<String, askama::Error> {
        let mut mandatory_suffix = "";
        if !optional {
            mandatory_suffix = "!!"
        }
        let res: String = match t {
            Type::UInt8 => format!("{name}.getInt(\"{field_name}\").toUByte()").into(),
            Type::Int8 => format!("{name}.getInt(\"{field_name}\").toByte()").into(),
            Type::UInt16 => format!("{name}.getInt(\"{field_name}\").toUShort()").into(),
            Type::Int16 => format!("{name}.getInt(\"{field_name}\").toShort()").into(),
            Type::UInt32 => format!("{name}.getInt(\"{field_name}\").toUInt()").into(),
            Type::Int32 => format!("{name}.getInt(\"{field_name}\")").into(),
            Type::UInt64 => format!("{name}.getDouble(\"{field_name}\").toULong()").into(),
            Type::Int64 => format!("{name}.getDouble(\"{field_name}\").toLong()").into(),
            Type::Float32 => format!("{name}.getDouble(\"{field_name}\")").into(),
            Type::Float64 => format!("{name}.getDouble(\"{field_name}\")").into(),
            Type::Boolean => format!("{name}.getBoolean(\"{field_name}\")").into(),
            Type::String => format!("{name}.getString(\"{field_name}\"){mandatory_suffix}").into(),
            Type::Timestamp => "".into(),
            Type::Duration => "".into(),
            Type::Object(_) => "".into(),
            Type::Record(_) => {
                let record_type_name = type_name(t)?;
                format!(
                    "{name}.getMap(\"{field_name}\")?.let {{ as{record_type_name}(it)}}{mandatory_suffix}"
                )
                .into()
            }
            Type::Enum(inner) => {
                let enum_def = ci.get_enum_definition(inner).unwrap();
                match enum_def.is_flat() {
                    false => {
                        format!("{name}.getMap(\"{field_name}\")?.let {{ as{inner}(it)}}{mandatory_suffix}")
                            .into()
                    }
                    true => format!(
                        "{name}.getString(\"{field_name}\")?.let {{ as{inner}(it)}}{mandatory_suffix}"
                    )
                    .into(),
                }
            }
            Type::Error(_) => "".into(),
            Type::CallbackInterface(_) => "".into(),
            Type::Optional(inner) => {
                let unboxed = inner.as_ref();
                let inner_res = render_from_map(unboxed, ci, name, field_name, true)?;
                format!("if (hasNonNullKey({name}, \"{field_name}\")) {inner_res} else null")
            }
            Type::Sequence(inner) => {
                let unboxed = inner.as_ref();
                let element_type_name = type_name(unboxed)?;
                format!("{name}.getArray(\"{field_name}\")?.let {{ as{element_type_name}List(it) }}{mandatory_suffix}")
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

    pub fn ignored_function(nm: &str) -> Result<bool, askama::Error> {
        Ok(IGNORED_FUNCTIONS.contains(nm))
    }

    pub fn rn_convert_type(
        t: &TypeIdentifier,
        ci: &ComponentInterface,
    ) -> Result<String, askama::Error> {
        match t {
            Type::UInt8 | Type::UInt16 | Type::UInt32 => Ok(".toUInt()".to_string()),
            Type::Int64 => Ok(".toLong()".to_string()),
            Type::UInt64 => Ok(".toULong()".to_string()),
            Type::Float32 | Type::Float64 => Ok(".toFloat()".to_string()),
            Type::Optional(inner) => {
                let unboxed = inner.as_ref();
                let conversion = rn_convert_type(unboxed, ci).unwrap();
                let optional = match *unboxed {
                    Type::Int8
                    | Type::UInt8
                    | Type::Int16
                    | Type::UInt16
                    | Type::Int32
                    | Type::UInt32 => ".takeUnless { it == 0 }".to_string(),
                    Type::Int64 => ".takeUnless { it == 0L }".to_string(),
                    Type::UInt64 => ".takeUnless { it == 0UL }".to_string(),
                    Type::Float32 | Type::Float64 => ".takeUnless { it == 0.0 }".to_string(),
                    Type::String => ".takeUnless { it.isEmpty() }".to_string(),
                    _ => "".to_string(),
                };
                Ok(format!("{}{}", conversion, optional))
            }
            _ => Ok("".to_string()),
        }
    }

    pub fn rn_type_name(
        t: &TypeIdentifier,
        ci: &ComponentInterface,
    ) -> Result<String, askama::Error> {
        match t {
            Type::Int8 | Type::UInt8 | Type::Int16 | Type::UInt16 | Type::Int32 | Type::UInt32 => {
                Ok("Int".to_string())
            }
            Type::Int64 | Type::UInt64 | Type::Float32 | Type::Float64 => Ok("Double".to_string()),
            Type::String => Ok("String".to_string()),
            Type::Enum(inner) => {
                let enum_def = ci.get_enum_definition(inner).unwrap();
                match enum_def.is_flat() {
                    false => Ok("ReadableMap".to_string()),
                    true => Ok("String".to_string()),
                }
            }
            Type::Record(_) => Ok("ReadableMap".to_string()),
            Type::Optional(inner) => {
                let unboxed = inner.as_ref();
                rn_type_name(unboxed, ci)
            }
            Type::Sequence(_) => Ok("ReadableArray".to_string()),
            _ => Ok("".to_string()),
        }
    }

    pub fn temporary(nm: &str) -> Result<String, askama::Error> {
        Ok(format!("{nm}Tmp"))
    }
}
