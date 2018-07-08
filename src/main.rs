extern crate rand;
extern crate rodio;

use std::f32;
use std::time::Duration;
use std::thread::sleep_ms;

use rand::prelude::*;
use rodio::{
    buffer::SamplesBuffer,
    dynamic_mixer::mixer,
    Sink,
    Source,
    source::SineWave,
    SpatialSink,
};


fn make_noise(duration: f64, alpha: f64) -> SamplesBuffer<i16> {
    let buffer: Vec<i16> = rand::distributions::Normal::new(0.0, 30000.0)
        .sample_iter(&mut thread_rng())
        .take((44100.0 * duration) as usize)
        .scan(0.0, |x1, x0| {
            *x1 = *x1 * (1.0 - alpha) + x0 * alpha;
            Some(*x1)
        })
        .map(|x| x as i16)
        .collect();
    SamplesBuffer::new(1, 44100, buffer)
}


fn play_sound() {
    println!("playing one sound...");
    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);
    let source = make_noise(2.0, 0.1);
    sink.append(source);
    sink.sleep_until_end();
}


fn play_sound2() {
    println!("playing two sounds, one after the other...");
    // TODO: Why can't we continue with another source after stopping the sink?
    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);
    sink.append(make_noise(2.0, 0.1));
    sleep_ms(100);
    sink.stop();
    sink.play();
    sink.append(make_noise(2.0, 0.9));
    sink.sleep_until_end();
}

fn play_multisink() {
    println!("playing two sounds simultaneously through separate sinks...");
    let device = rodio::default_output_device().unwrap();
    let sink1 = Sink::new(&device);
    let sink2 = Sink::new(&device);
    sink1.append(make_noise(2.0, 0.1));
    sleep_ms(500);
    sink2.append(SineWave::new(440).take_duration(Duration::new(1, 0)));
    sink1.sleep_until_end();
    sink2.sleep_until_end();
}

fn play_mixer() {
    println!("playing two sounds simultaneously through one sink (using the dynamic mixer)...");
    let device = rodio::default_output_device().unwrap();

    let (controller, mix) = mixer(1, 44100);

    let sink = Sink::new(&device);
    sink.append(mix);

    controller.add(SineWave::new(330).take_duration(Duration::new(1, 0)));
    sleep_ms(500);
    controller.add(SineWave::new(440).take_duration(Duration::new(1, 0)));
    sleep_ms(1500);
    /// TODO: This sound is never played because the mixer stops before it starts
    controller.add(SineWave::new(880).take_duration(Duration::new(1, 0)));

    sink.sleep_until_end();
}

fn play_spatial() {
    println!("playing a spatially moving sound using SpatialSink...");
    let device = rodio::default_output_device().unwrap();
    let mut sink = SpatialSink::new(&device,
                                [0.0, 2.0, 0.0],
                                [-1.0, 0.0, 0.0],
                                [1.0, 0.0, 0.0]);
    let source = make_noise(10.0, 0.1);
    sink.append(source);

    let mut t = 0.0;
    while !sink.empty() {
        // circle around the listener, one round in two seconds
        let x = 2.0 * f32::sin(t * f32::consts::PI);
        let y = 2.0 * f32::cos(t * f32::consts::PI);
        sink.set_emitter_position([x, y, 0.0]);

        sleep_ms(10);
        t += 10e-3;
    }
}


fn main() {
    println!("Hello, audio world!");

    //let file = File::open("beep.wav").unwrap();
    //let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    //let source = rodio::source::SineWave::new(440);

    /*play_sound();
    sleep_ms(500);

    play_sound2();
    sleep_ms(500);

    play_multisink();
    sleep_ms(500);

    play_mixer();
    sleep_ms(500);*/

    play_spatial();
    sleep_ms(500);

}
