//! Code supporting XML to TSV conversion.

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::Path,
    time::Instant,
};

use clap::{command, Parser};
use flate2::read::GzDecoder;
use quick_xml::reader::Reader;
use tracing::info;

// size of buffers sent across threads
const BUF_SIZE: usize = 256 * 1024;
// length of queue with buffers pre-filled in background thread
const QUEUE_LEN: usize = 5;

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

/// Output files written to.
pub struct OutputFiles {
    pub b37_small: File,
    pub b37_sv: File,
    pub b38_small: File,
    pub b38_sv: File,
}

impl OutputFiles {
    pub fn from_output_path(path: &str) -> Result<OutputFiles, anyhow::Error> {
        let path = Path::new(path);
        Ok(OutputFiles {
            b37_small: File::create(path.join("b37_small.tsv"))?,
            b37_sv: File::create(path.join("b37_sv.tsv"))?,
            b38_small: File::create(path.join("b38_small.tsv"))?,
            b38_sv: File::create(path.join("b38_sv.tsv"))?,
        })
    }
}

fn run_impl(reader: &mut impl Read, args: &Args) -> Result<(), anyhow::Error> {
    let reader = BufReader::new(reader);
    let mut xml_reader = Reader::from_reader(reader);
    let output_files = OutputFiles::from_output_path(&args.path_output)?;

    // scratch space for building ClinVarSet

    let mut buf = Vec::new();
    loop {
        match xml_reader.read_event_into(&mut buf)? {
            quick_xml::events::Event::Start(elem) => {
                match elem.name().as_ref() {
                    b"ClinVarSet" => {
                        elem.attributes().for_each(|a| {
                            let a = a.unwrap();
                            if a.key.local_name().as_ref() == b"ID" {
                                println!("ID = {}", std::str::from_utf8(a.value.as_ref()).unwrap());
                            }
                        })
                    },
                    _ => ()
                }
            }
            // quick_xml::events::Event::End(_) => todo!(),
            // quick_xml::events::Event::Empty(_) => todo!(),
            // quick_xml::events::Event::Text(_) => todo!(),
            // quick_xml::events::Event::CData(_) => todo!(),
            // quick_xml::events::Event::Decl(_) => todo!(),
            // quick_xml::events::Event::PI(_) => todo!(),
            quick_xml::events::Event::Eof => break,
            _ => (),
        }

        buf.clear();
    }

    Ok(())
}

pub fn run(args: &Args) -> Result<(), anyhow::Error> {
    let before_run = Instant::now();
    info!("starting xml-to-tsv");

    let file = File::open(&args.path_input_xml)?;
    if args.path_input_xml.ends_with(".gz") {
        let decoder = GzDecoder::new(file);

        thread_io::read::reader(BUF_SIZE, QUEUE_LEN, decoder, |reader| {
            run_impl(reader, args)
        })?;
    } else {
        let mut buf_reader = BufReader::new(file);
        run_impl(&mut buf_reader, args)?;
    }

    info!("xml-to-tsv ran for {:?}", before_run.elapsed());
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use pretty_assertions::assert_eq;
    use temp_testdir::TempDir;

    use super::{run, Args};

    // #[test]
    // fn smoke_test_small_var() -> Result<(), anyhow::Error> {
    //     let temp = TempDir::default();
    //     let args = Args {
    //         path_input_xml: "tests/data/clinvar-small-74722873.xml".to_owned(),
    //         path_output: temp.to_str().unwrap().to_owned(),
    //     };

    //     run(&args)?;

    //     let exp_b37_small = read_to_string("tests/data/clinvar-small-74722873.out.b37_small.tsv")?;
    //     let exp_b37_sv = read_to_string("tests/data/clinvar-small-74722873.out.b37_sv.tsv")?;
    //     let exp_b38_small = read_to_string("tests/data/clinvar-small-74722873.out.b38_small.tsv")?;
    //     let exp_b38_sv = read_to_string("tests/data/clinvar-small-74722873.out.b38_sv.tsv")?;

    //     let res_b37_small = read_to_string(temp.join("b37_small.tsv"))?;
    //     let res_b37_sv = read_to_string(temp.join("b37_sv.tsv"))?;
    //     let res_b38_small = read_to_string(temp.join("b38_small.tsv"))?;
    //     let res_b38_sv = read_to_string(temp.join("b38_sv.tsv"))?;

    //     assert_eq!(exp_b37_small, res_b37_small);
    //     assert_eq!(exp_b37_sv, res_b37_sv);
    //     assert_eq!(exp_b38_small, res_b38_small);
    //     assert_eq!(exp_b38_sv, res_b38_sv);

    //     Ok(())
    // }
}
