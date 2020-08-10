use soundpipe::factory::Factory;
use soundpipe::ugens::oscillators::common::MonoOsc;
use soundpipe::Soundpipe;

fn main() {
    let soundpipe = Soundpipe::new(44100);
    let saw = soundpipe.bl_saw();
    let out = saw.compute();
    eprintln!("out = {:?}", out);
}
