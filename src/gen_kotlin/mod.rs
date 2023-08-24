use std::cell::RefCell;
use std::collections::BTreeSet;

use askama::Template;
use uniffi_bindgen::interface::*;

pub use uniffi_bindgen::bindings::kotlin::gen_kotlin::*;

use crate::generator::RNConfig;

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

pub mod filters {
    use heck::*;
    use uniffi_bindgen::backend::{CodeType, TypeIdentifier};

    use super::*;

    fn oracle() -> &'static KotlinCodeOracle {
        &KotlinCodeOracle
    }

    pub fn type_name(codetype: &impl CodeType) -> Result<String, askama::Error> {
        Ok(codetype.type_label(oracle()))
    }

    pub fn render_to_array(type_name: &str) -> Result<String, askama::Error> {
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
            _ => Ok(format!("array.pushMap(readableMapOf(value))").into()),
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
        optional: bool,
    ) -> Result<String, askama::Error> {
        let mut mandatory_suffix = "";
        if !optional {
            mandatory_suffix = "!!"
        }
        let res: String = match t {
            Type::UInt8 => format!("data.getInt(\"{name}\").toUByte()").into(),
            Type::Int8 => format!("data.getInt(\"{name}\").toByte()").into(),
            Type::UInt16 => format!("data.getInt(\"{name}\").toUShort()").into(),
            Type::Int16 => format!("data.getInt(\"{name}\").toShort()").into(),
            Type::UInt32 => format!("data.getInt(\"{name}\").toUInt()").into(),
            Type::Int32 => format!("data.getInt(\"{name}\")").into(),
            Type::UInt64 => format!("data.getInt(\"{name}\").toULong()").into(),
            Type::Int64 => format!("data.getInt(\"{name}\").toLong()").into(),
            Type::Float32 => format!("data.getDouble(\"{name}\")").into(),
            Type::Float64 => format!("data.getDouble(\"{name}\")").into(),
            Type::Boolean => format!("data.getBoolean(\"{name}\")").into(),
            Type::String => format!("data.getString(\"{name}\"){mandatory_suffix}").into(),
            Type::Timestamp => "".into(),
            Type::Duration => "".into(),
            Type::Object(_) => "".into(),
            Type::Record(_) => {
                let record_type_name = type_name(t)?;
                format!(
                    "data.getMap(\"{name}\")?.let {{ as{record_type_name}(it)}}{mandatory_suffix}"
                )
                .into()
            }
            Type::Enum(inner) => {
                let enum_def = ci.get_enum_definition(inner).unwrap();
                match enum_def.is_flat() {
                    false => {
                        format!("data.getMap(\"{name}\")?.let {{ as{inner}(it)}}{mandatory_suffix}")
                            .into()
                    }
                    true => format!(
                        "data.getString(\"{name}\")?.let {{ as{inner}(it)}}{mandatory_suffix}"
                    )
                    .into(),
                }
            }
            Type::Error(_) => "".into(),
            Type::CallbackInterface(_) => "".into(),
            Type::Optional(inner) => {
                let unboxed = inner.as_ref();
                let inner_res = render_from_map(unboxed, ci, name, true)?;
                inner_res
            }
            Type::Sequence(inner) => {
                let unboxed = inner.as_ref();
                let element_type_name = type_name(unboxed)?;
                format!("data.getArray(\"{name}\")?.let {{ as{element_type_name}List(it) }}{mandatory_suffix}")
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
