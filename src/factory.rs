use crate::soundpipe::Soundpipe;
use crate::ugens::effects::revsc::Revsc;
use crate::ugens::envelopes::adsr::Adsr;
use crate::ugens::oscillators::bl_saw::BlSaw;

pub trait Factory {
    fn bl_saw(&self) -> BlSaw;
    fn adsr(&self) -> Adsr;
    fn revsc(&self) -> Revsc;
}

impl Factory for Soundpipe {
    fn bl_saw(&self) -> BlSaw {
        BlSaw::new(self.clone())
    }

    fn adsr(&self) -> Adsr {
        Adsr::new(self.clone())
    }

    fn revsc(&self) -> Revsc {
        Revsc::new(self.clone())
    }
}
