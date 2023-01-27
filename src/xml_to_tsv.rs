//! Code supporting XML to TSV conversion.

use std::time::Instant;

use clap::{command, Parser};
use tracing::info;

/// Command line arguments for `sv bg-db-to-bin` sub command.
#[derive(Parser, Debug)]
#[command(about = "Convert ClinVAR XML to TSV TSV", long_about = None)]
pub struct Args {
    /// Path to the ClinVar XML file.
    #[arg(long, required = true)]
    pub path_input_xml: String,
    /// Output prefix path.
    #[arg(long, required = true)]
    pub path_output: String,
}

#[tracing::instrument]
pub fn run(args: &Args) -> Result<(), anyhow::Error> {
    let before_run = Instant::now();
    info!("starting xml-to-tsv");

    info!("xml-to-tsv ran for {:?}", before_run.elapsed());
    Ok(())
}
