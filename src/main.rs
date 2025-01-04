use std::time::{Duration, Instant};

use rodio::{buffer::SamplesBuffer, OutputStream, Sink};

const FRAMES_PER_SECOND: u32 = 60;
const SAMPLE_RATE: u32 = 44_100;
const TOTAL_SECONDS: u32 = 3; // Run for X Seconds
const TOTAL_FRAMES: u32 = FRAMES_PER_SECOND * TOTAL_SECONDS;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let mut reader = hound::WavReader::open("CantinaBand3.wav").unwrap();
    println!("{:?}", reader.spec());
    let sound: Vec<_> = reader
        .samples::<i16>()
        .flat_map(|sample| {
            let sample = sample.unwrap();
            // Double it, because the sample rate is 22050hz
            [sample, sample]
        })
        .collect();

    sink.set_volume(0.5);

    let frame_duration = Duration::from_secs_f32(1.0 / FRAMES_PER_SECOND as f32);

    let mut game = Game {
        sink,
        samples_per_frame: SAMPLE_RATE / FRAMES_PER_SECOND,
        t: 0.0,
        sound,
        audio_frame: 0,
    };

    let mut last_frame = Instant::now();

    // Main game loop
    let mut frame_count = 0;
    loop {
        let now = Instant::now();
        if now.duration_since(last_frame) >= frame_duration {
            last_frame = now;

            // Update game state
            game.update();

            frame_count += 1;

            if frame_count > TOTAL_FRAMES {
                break;
            }
        }
    }
}

struct Game {
    sink: Sink,
    samples_per_frame: u32,
    t: f32,
    audio_frame: usize,
    sound: Vec<i16>,
}

impl Game {
    fn update(&mut self) {
        let mut audio = Vec::with_capacity(self.samples_per_frame as usize);

        // // Frequency of a4 in Hz
        // let frequency = 440.0;

        for _ in 0..self.samples_per_frame {
            // // Sine Wave Generation Code
            // self.t += frequency / SAMPLE_RATE as f32;

            // // Keep the phase within [0.0, 1.0] to avoid overflow
            // if self.t > 1.0 {
            //     self.t -= 1.0;
            // }

            // // Generate the FM-modulated sine wave sample
            // let v = (self.t * std::f32::consts::TAU).sin();
            // audio.push(v);

            audio.push(
                self.sound
                    .get(self.audio_frame)
                    .cloned()
                    .unwrap_or_default(),
            );
            self.audio_frame += 1;
        }

        self.sink.append(SamplesBuffer::new(1, SAMPLE_RATE, audio));
    }
}
