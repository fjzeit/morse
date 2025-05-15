use clap::Parser;
use morse::encoding;
use morse::encoding::encode::to_morse;
use morse::encoding::sound::to_sound;
use std::fs::File;

mod cli_def;

fn main() {
    let cli = cli_def::Cli::parse();

    match cli.command {
        cli_def::Commands::EncodeText(args) => {
            let morse = to_morse(&args.message).unwrap();
            println!("{}", morse);
        }
        cli_def::Commands::EncodeSound(args) => {
            let morse = to_morse(&args.message).unwrap();
            let file = File::create(args.output).unwrap();
            to_sound(&morse, args.sample_rate, args.wpm, args.volume, args.frequency, file);
        }
        cli_def::Commands::DecodeText(args) => {
            let text = encoding::decode::to_text(&args.message);
            println!("{}", text);
        }
    }
}
