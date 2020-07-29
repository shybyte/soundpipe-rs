pub trait UGen: Drop {}

pub trait MonoOscInternal: UGen {
    fn compute_internal(&self) -> f32;
}


pub trait MonoOsc: MonoOscInternal {
    fn compute(&self) -> f32 {
        self.compute_internal()
    }
}
