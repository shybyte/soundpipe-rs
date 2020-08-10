use crate::soundpipe::Soundpipe;
use crate::ugens::effects::revsc::Revsc;
use crate::ugens::envelopes::adsr::Adsr;
use crate::ugens::oscillators::bl_saw::BlSaw;
use crate::ugens::oscillators::bl_square::BlSquare;
use crate::ugens::port::Port;

pub trait Factory {
    fn bl_saw(&self) -> BlSaw;
    fn bl_square(&self) -> BlSquare;
    fn adsr(&self) -> Adsr;
    fn revsc(&self) -> Revsc;
    fn port(&self, htime: f32) -> Port;
}

impl Factory for Soundpipe {
    fn bl_saw(&self) -> BlSaw {
        BlSaw::new(self.clone())
    }

    fn bl_square(&self) -> BlSquare {
        BlSquare::new(self.clone())
    }

    fn adsr(&self) -> Adsr {
        Adsr::new(self.clone())
    }

    fn revsc(&self) -> Revsc {
        Revsc::new(self.clone())
    }

    fn port(&self, htime: f32) -> Port {
        Port::new(self.clone(), htime)
    }
}
