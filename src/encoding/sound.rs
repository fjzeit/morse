use std::f32::consts::PI;
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
    let mut phase: f32 = 0.0;

    let amplitude: f32 = amplitude as f32 / 100f32;

    for c in morse.chars() {
        let (tone, n) = match c {
            morse::DIT => (true, 1),
            morse::DAH => (true, 3),
            morse::SHORT_GAP => (false, 3),
            morse::MEDIUM_GAP => (false, 7),
            _ => panic!("Unexpected character: {}", c),
        };

            write_sound(
                tone,
                &mut samples,
                samples_per_unit,
                sample_rate as usize,
                amplitude,
                frequency as f32,
                &mut phase,
                n * samples_per_unit,
            );

        write_sound(
            false,
            &mut samples,
            samples_per_unit,
            sample_rate as usize,
            amplitude,
            frequency as f32,
            &mut phase,
            samples_per_unit,
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
    duration: u32
) {
    let fade_samples = (0.005 * sample_rate as f32).min(samples_per_unit as f32 * 0.1) as u32;
    let fade_samples = fade_samples.min(samples_per_unit / 2);

    for t in 0..duration {
        let angle = 2.0 * PI * t as f32 * frequency / sample_rate as f32 + *phase;
        let v = amplitude * angle.sin();

        let envelope = if tone {
            if t < fade_samples {
                (t as f32) / (fade_samples as f32)
            } else if t >= duration - fade_samples {
                ((duration - t) as f32) / (fade_samples as f32)
            } else {
                1.0
            }
        } else {
            0.0
        };

        samples.push(v * envelope);
    }

    *phase += 2.0 * PI * duration as f32 * frequency / sample_rate as f32;
    *phase %= 2.0 * PI;
}

