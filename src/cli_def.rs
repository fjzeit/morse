use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    EncodeText(EncodeTextArgs),
    EncodeSound(EncodeSoundArgs),
    DecodeText(DecodeTextArgs),
}

#[derive(Args, Debug)]
pub struct EncodeTextArgs {
    #[clap(help = "Message to translate")]
    pub message: String,
}

#[derive(Args, Debug)]
pub struct EncodeSoundArgs {
    #[arg(
        short,
        long,
        default_value = "40",
        help = "Words per minute (20-50)",
        value_parser = clap::value_parser!(u32).range(20..=50)
    )]
    pub wpm: u32,

    #[arg(
        short,
        long,
        default_value = "80",
        help = "Volume % (0-100)",
    )]
    pub volume: u32,

    #[arg(
        short,
        long,
        default_value = "1200",
        help = "Signal frequency",
    )]
    pub frequency: u32,

    #[arg(
        short,
        long,
        default_value = "44100",
        help = "Sample rate",
    )]
    pub sample_rate: u32,

    #[arg(
        short,
        long,
        default_value = "./out.wav",
        help = "File to save audio"
    )]
    pub output: String,

    #[clap(help = "Message to translate")]
    pub message: String,
}

#[derive(Args, Debug)]
pub struct DecodeTextArgs {
    #[clap(help = "Morse text to translate")]
    pub message: String,
}
