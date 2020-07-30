use soundpipe_derive::UGenMacro;

use crate::ffi::{sp_adsr, sp_adsr_compute, sp_adsr_create, sp_adsr_destroy, sp_adsr_init};
use crate::soundpipe::Soundpipe;

#[derive(UGenMacro)]
pub struct Adsr {
    sp: Soundpipe,
    ffi: *mut sp_adsr,
}

impl Adsr {
    pub fn set_attack_time(&self, time_in_seconds: f32) {
        unsafe {
            (*self.ffi).atk = time_in_seconds;
        }
    }

    pub fn set_decay_time(&self, time_in_seconds: f32) {
        unsafe {
            (*self.ffi).dec = time_in_seconds;
        }
    }

    pub fn compute_internal(&self, input: &mut f32, output: &mut f32)  {
        unsafe {
            sp_adsr_compute(*self.sp.sp_ffi, self.ffi, input, output);
        }
    }

    pub fn compute(&self, input: f32) -> f32 {
        let mut out: f32 = 0.0;
        let mut input = input;
        self.compute_internal(&mut input, &mut out);
        out
    }
}
