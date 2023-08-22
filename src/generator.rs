use anyhow::Result;
use askama::Template;
use camino::Utf8Path;
use serde::*;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use uniffi_bindgen::{BindingGenerator, BindingGeneratorConfig, ComponentInterface};

use crate::gen_kotlin;
use crate::gen_swift;

pub struct RNBindingGenerator {}

impl RNBindingGenerator {
    fn write_kotlin_bindings(
        &self,
        ci: &ComponentInterface,
        config: RNConfig,
        out_dir: &Utf8Path,
    ) -> Result<()> {
        // generate kotlin
        let kotlin_output = self::gen_kotlin::Generator::new(config.clone(), ci)
            .render()
            .map_err(anyhow::Error::new)?;
        let kotlin_out_file = out_dir.join(Utf8Path::new(
            "android/src/main/java/com/breezsdk/BreezSDKMapper.kt",
        ));
        let mut f = File::create(&kotlin_out_file)?;
        write!(f, "{}", kotlin_output)?;
        if let Err(e) = Command::new("ktlint")
            .arg("-F")
            .arg(&kotlin_out_file)
            .output()
        {
            println!(
                "Warning: Unable to auto-format {} using ktlint: {:?}",
                kotlin_out_file.file_name().unwrap(),
                e
            )
        }
        Ok(())
    }

    fn write_swift_bindings(
        &self,
        ci: &ComponentInterface,
        config: RNConfig,
        out_dir: &Utf8Path,
    ) -> Result<()> {
        let swift_output = self::gen_swift::Generator::new(config.clone(), &ci)
            .render()
            .map_err(anyhow::Error::new)?;
        let swift_out_file = out_dir.join(Utf8Path::new("ios/Sources/ios/BreezSDKMapper.swift"));
        print!("{}", swift_output);
        let mut f = File::create(&swift_out_file)?;
        write!(f, "{}", swift_output)?;
        if let Err(e) = Command::new("swiftformat")
            .arg(swift_out_file.as_str())
            .output()
        {
            println!(
                "Warning: Unable to auto-format {} using swiftformat: {e:?}",
                swift_out_file.file_name().unwrap(),
            );
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RNConfig {
    package_name: Option<String>,
}

impl RNConfig {}

impl BindingGeneratorConfig for RNConfig {
    fn get_entry_from_bindings_table(_bindings: &toml::value::Value) -> Option<toml::value::Value> {
        if let Some(table) = _bindings.as_table() {
            table.get("rn").map(|v| v.clone())
        } else {
            None
        }
    }

    fn get_config_defaults(ci: &ComponentInterface) -> Vec<(String, toml::value::Value)> {
        vec![
            (
                "package_name".to_string(),
                toml::value::Value::String(ci.namespace().to_string()),
            ),
            (
                "cdylib_name".to_string(),
                toml::value::Value::String(ci.namespace().to_string()),
            ),
        ]
    }
}

impl BindingGenerator for RNBindingGenerator {
    type Config = RNConfig;

    fn write_bindings(
        &self,
        ci: ComponentInterface,
        config: Self::Config,
        out_dir: &Utf8Path,
    ) -> Result<()> {
        fs::create_dir_all(out_dir)?;

        // generate kotlin
        self.write_kotlin_bindings(&ci, config.clone(), out_dir)?;

        // generate ios
        self.write_swift_bindings(&ci, config.clone(), out_dir)?;
        Ok(())
    }
}
