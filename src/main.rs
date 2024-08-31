use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct CLI {
    namespace: String,
    command: String,
}

struct Project {
    name: String,
}

fn main() -> Result<()> {
    let args = CLI::parse();

    // read the config file
    let _config = std::fs::read_to_string("./.rustyflow.toml")
        .with_context(|| format!("could not read config file"))?;
    // check dependencies

    println!(
        "namespace: {:?}, command: {:?}",
        args.namespace, args.command
    );
    Ok(())
}
