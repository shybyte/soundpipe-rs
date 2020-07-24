use crate::soundpipe::Soundpipe;
use crate::ugens::oscillators::bl_saw::BlSaw;

pub trait Factory {
    fn bl_saw(&self) -> BlSaw;
}

impl Factory for Soundpipe {
    fn bl_saw(&self) -> BlSaw {
        BlSaw::new(self.clone())
    }
}
