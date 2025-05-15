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
- `--wpm`, `-w` (default: 40): Words per minute (20-50)
- `--volume`, `-v` (default: 80): Volume % (0-100)
- `--frequency`, `-f` (default: 1200): Signal frequency (Hz)
- `--sample-rate`, `-s` (default: 44100): Sample rate (Hz)
- `--output`, `-o` (default: ./out.wav): Output WAV file path

### Decode Text

Decode Morse code back to text:

```sh
morse decode-text -- "... --- ..."
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