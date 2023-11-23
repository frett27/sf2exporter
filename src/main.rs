use clap::Parser;

use rustysynth::SoundFont;
use rustysynth::Synthesizer;
use rustysynth::SynthesizerSettings;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;


// write in pcm format
fn write_pcm(left: &[f32], right: &[f32], path: &str) {
    let mut max: f32 = 0_f32;
    for t in 0..left.len() {
        if left[t].abs() > max {
            max = left[t].abs();
        }
        if right[t].abs() > max {
            max = right[t].abs();
        }
    }
    let a = 0.99_f32 / max;

    let mut buf: Vec<u8> = vec![0; 4 * left.len()];
    for t in 0..left.len() {
        let left_i16 = (a * left[t] * 32768_f32) as i16;
        let right_i16 = (a * right[t] * 32768_f32) as i16;

        let offset = 4 * t;
        buf[offset] = left_i16 as u8;
        buf[offset + 1] = (left_i16 >> 8) as u8;
        buf[offset + 2] = right_i16 as u8;
        buf[offset + 3] = (right_i16 >> 8) as u8;
    }

    let mut pcm = File::create(path).unwrap();
    pcm.write_all(&buf[..]).unwrap();
}

/// write in wav format
fn write_wav(left: &[f32], right: &[f32], path: &str) {
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(path, spec).unwrap();

    let mut max = 0_f32;

    for t in 0..left.len() {
        if left[t].abs() > max {
            max = left[t].abs();
        }
        if right[t].abs() > max {
            max = right[t].abs();
        }
    }

    let a = 0.99_f32 / max;
    for t in 0..left.len() {
        let left_i16 = (a * left[t] * 32768_f32) as i16;
        let right_i16 = (a * right[t] * 32768_f32) as i16;
        writer.write_sample(left_i16 as i16).unwrap();
        writer.write_sample(right_i16 as i16).unwrap();
    }
}

type FileWriter = fn(&[f32], &[f32], &str);

// render to a writer
fn write_file(sound_font: &Arc<SoundFont>, note: i32, filename: &str, writer: FileWriter) {
  
    // Create the synthesizer.
    let settings = SynthesizerSettings::new(44100);

    let mut synthesizer = Synthesizer::new(&sound_font, &settings).unwrap();

    // Play some notes (middle C, E, G).
    synthesizer.note_on(0, note, 127);

    // The output buffer (3 seconds).
    let sample_count = (20 * settings.sample_rate) as usize;
    let mut left: Vec<f32> = vec![0_f32; sample_count];
    let mut right: Vec<f32> = vec![0_f32; sample_count];

    // Render the waveform.
    synthesizer.render(&mut left[..], &mut right[..]);

    writer(&left[..], &right[..], filename);
}

/// Parameters definition for rendering
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, required = true)]
    sf2: String,

    #[arg(short, long, required = true)]
    output: String,

}

fn main() {
    let args = Args::parse();

    // Load the SoundFont.
    let mut sf2 = File::open(args.sf2).unwrap();
    let sound_font = Arc::new(SoundFont::new(&mut sf2).unwrap());

    let (renderer, extension) = match args.output.as_str() {
        "wav" => {
            (write_wav as FileWriter, "WAV")
        },
        "pcm" => {
            (write_pcm as FileWriter, "pcm")
        }
        _ => panic!("unknown format")
    };

    for i in 0..=127 {
        let filename = format!("DEFAULT_{}.{}", i, extension);
        println!("rendering {}", filename);
        write_file(&sound_font, i, &filename, renderer);
    }
}
