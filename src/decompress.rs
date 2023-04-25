use super::error::Error;
use lzfse_rust::LzfseRingDecoder;
use std::fs::File;

#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
pub struct Decompress {
    #[arg(short, long)]
    input: Option<std::path::PathBuf>,

    #[arg(short, long)]
    output: Option<std::path::PathBuf>,
}

impl Decompress {
    pub fn execute(&self) -> Result<(), Error> {
        let input_path = self.input.as_ref().ok_or(Error::MissingInputFilePath)?;
        let output_path = self.output.as_ref().ok_or(Error::MissingOutputFilePath)?;

        let mut input_file = File::open(input_path)?;
        let mut output_file = File::create(output_path)?;

        let (n_raw_bytes, n_payload_bytes) =
            LzfseRingDecoder::default().decode(&mut input_file, &mut output_file)?;

        println!(
            "decompressed {} to {};\nraw bytes: {}, payload bytes: {}",
            input_path.display(),
            output_path.display(),
            n_raw_bytes,
            n_payload_bytes
        );

        Ok(())
    }
}
