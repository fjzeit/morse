use std::fs::File;
use std::mem;
use super::morse;

pub fn to_sound(
    morse: &str,
    sample_rate: u32,
    words_per_minute: u32,
    amplitude: u32,
    frequency: u32,
    mut output: File,
) {
    const WORD_SIZE: f64 = 50f64; // PARIS
    let samples_per_unit =
        (sample_rate as f64 * 60f64 / WORD_SIZE / words_per_minute as f64) as u32;
    let single_sample_size = mem::size_of::<f32>() as u16;
    let head = wav_io::new_header(sample_rate, single_sample_size * 8, true, true);
    let mut samples: Vec<f32> = vec![];
    let mut phase : f32 = 0.0;

    let amplitude: f32 = amplitude as f32 / 100f32;

    for c in morse.chars() {
        let (tone, n) = match c {
            morse::DIT => (true, 1),
            morse::DAH => (true, 3),
            morse::SHORT_GAP => (false, 3),
            morse::MEDIUM_GAP => (false, 7),
            _ => panic!("Unexpected character: {}", c),
        };

        for _ in 0..n {
            write_sound(
                tone,
                &mut samples,
                samples_per_unit,
                sample_rate as usize,
                amplitude,
                frequency as f32,
                &mut phase
            );
        }
        write_sound(
            false,
            &mut samples,
            samples_per_unit,
            sample_rate as usize,
            amplitude,
            frequency as f32,
            &mut phase
        );
    }

    wav_io::write_to_file(&mut output, &head, &samples).unwrap();
}

fn write_sound(
    tone: bool,
    samples: &mut Vec<f32>,
    samples_per_unit: u32,
    sample_rate: usize,
    amplitude: f32,
    frequency: f32,
    phase: &mut f32,
) {
    if !tone {
        samples.extend(vec![0f32; samples_per_unit as usize]);
        return;
    }

    for t in 0..samples_per_unit {
        let angle = 2f32 * std::f32::consts::PI * t as f32 * frequency / sample_rate as f32 + *phase;
        let v = amplitude * angle.sin();
        samples.push(v);
    }

    // Update phase for the next call
    *phase += 2f32 * std::f32::consts::PI * samples_per_unit as f32 * frequency / sample_rate as f32;
    *phase %= 2f32 * std::f32::consts::PI; // Keep phase in [0, 2π]
}

