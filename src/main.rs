use clap::Parser;
use compress::Compress;
use decompress::Decompress;
use error::Error;
mod compress;
mod decompress;
mod error;

#[derive(Parser)]
#[command(name = "lzfse-cli")]
#[command(bin_name = "lzfse-cli")]
enum CLI {
    Compress(Compress),
    Decompress(Decompress),
}

impl CLI {
    fn execute() -> Result<(), Error> {
        match Self::parse() {
            CLI::Compress(compress) => compress.execute(),
            CLI::Decompress(decompress) => decompress.execute(),
        }
    }
}

fn main() -> Result<(), Error> {
    CLI::execute()
}
