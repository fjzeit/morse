# Morse

A Rust command-line tool and library for encoding and decoding Morse code, with support for text and sound (WAV) output.

## Features

- Encode text to Morse code.
- Encode text to Morse code sound (WAV file).
- Decode Morse code back to text.

## Usage

### Encode Text

Print Morse code to stdout:

```sh
morse encode-text "Hello World"
```

### Encode Sound

Encode text to Morse code and output as a WAV file:

```sh
morse encode-sound "Hello World" --output hello.wav --wpm 20 --volume 80 --frequency 700 --sample-rate 44100
```

**Options for `encode-sound`:**
- `--wpm`, `-w` (default: 30): Words per minute
- `--volume`, `-v` (default: 50): Volume % (0-100)
- `--frequency`, `-f` (default: 800): Signal frequency (Hz)
- `--sample-rate`, `-s` (default: 48000): Sample rate (Hz)
- `--output`, `-o` (default: ./out.wav): Output WAV file path

### Decode Text

Decode Morse code back to text:

```sh
morse decode-text "... --- ..."
```

## Library Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
morse = { path = "." }
```

Example:

```rust
use morse::encoding::encode::to_morse;
use morse::encoding::decode::to_text;

let morse = to_morse("Hello").unwrap();
let text = to_text(".... . .-.. .-.. --- ");
```

## Supported Characters

A-Z, 0-9, and common punctuation: `& ' @ ) ( : , = ! . - % + " ? /` and space.

## Dependencies

- [clap](https://crates.io/crates/clap)
- [serde](https://crates.io/crates/serde)
- [wav_io](https://crates.io/crates/wav_io)
- [rstest](https://crates.io/crates/rstest) (for tests)

## Futures

I hope to add a sound-to-text option in the future.