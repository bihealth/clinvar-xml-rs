//! Common functionality.

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use byte_unit::Byte;
use clap_verbosity_flag::{InfoLevel, Verbosity};

use clap::Parser;
use flate2::read::GzDecoder;
use tracing::debug;

/// Commonly used command line arguments.
#[derive(Parser, Debug)]
pub struct Args {
    /// Verbosity of the program
    #[clap(flatten)]
    pub verbose: Verbosity<InfoLevel>,
}

/// Helper to print the current memory resident set size via `tracing`.
pub fn trace_rss_now() {
    let me = procfs::process::Process::myself().unwrap();
    let page_size = procfs::page_size().unwrap();
    debug!(
        "RSS now: {}",
        Byte::from_bytes((me.stat().unwrap().rss * page_size) as u128).get_appropriate_unit(true)
    );
}

/// Transparently open a file with gzip decoder.
pub fn open_maybe_gz(path: &str) -> Result<Box<dyn BufRead>, anyhow::Error> {
    if path.ends_with(".gz") {
        let file = File::open(path)?;
        let decoder = GzDecoder::new(file);
        let reader = BufReader::new(decoder);
        Ok(Box::new(reader))
    } else {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(Box::new(reader))
    }
}
