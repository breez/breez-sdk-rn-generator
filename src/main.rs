mod gen_kotlin;
mod gen_swift;
mod generator;
use camino::Utf8Path;
use clap::Parser;
use generator::RNBindingGenerator;

#[derive(Parser, Debug)]
pub(crate) struct Cli {
    #[clap(name = "binding_dir", short = 'b', long = "binding_dir")]
    pub(crate) binding_dir: String,
    #[clap(name = "out_dir", short = 'o', long = "out_dir")]
    pub(crate) out_dir: String,
}

fn main() {
    let cli = Cli::parse();
    let binding_dir = Utf8Path::new(cli.binding_dir.as_str());
    let udl_file = binding_dir.join(Utf8Path::new("src/breez_sdk.udl"));
    let config = binding_dir.join(Utf8Path::new("uniffi.toml"));
    let out_dir = Utf8Path::new(cli.out_dir.as_str());

    // React Native generator
    uniffi_bindgen::generate_external_bindings(
        RNBindingGenerator {},
        udl_file,
        Some(config),
        Some(out_dir),
    )
    .unwrap();
}
