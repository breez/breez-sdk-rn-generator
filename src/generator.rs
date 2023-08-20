use anyhow::Result;
use camino::Utf8Path;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use uniffi_bindgen::{BindingGenerator, ComponentInterface};

use askama::Template;

use crate::gen_rn;

pub struct RNBindingGenerator {}

impl BindingGenerator for RNBindingGenerator {
    type Config = gen_rn::Config;

    fn write_bindings(
        &self,
        ci: ComponentInterface,
        config: Self::Config,
        out_dir: &Utf8Path,
    ) -> Result<()> {
        let res = self::gen_rn::RNWrapper::new(config.clone(), &ci)
            .render()
            .map_err(anyhow::Error::new)?;
        print!("{}", res);
        fs::create_dir_all(out_dir)?;
        let out_file = out_dir.join(Utf8Path::new(
            "android/src/main/java/com/breezsdk/BreezSDKMapper.kt",
        ));
        let mut f = File::create(&out_file)?;
        write!(f, "{}", res)?;
        if let Err(e) = Command::new("ktlint").arg("-F").arg(&out_file).output() {
            println!(
                "Warning: Unable to auto-format {} using ktlint: {:?}",
                out_file.file_name().unwrap(),
                e
            )
        }
        print!("{out_file}");
        Ok(())
    }
}
