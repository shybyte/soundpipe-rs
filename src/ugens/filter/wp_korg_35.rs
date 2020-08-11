use soundpipe_derive::UGenMacro;

use crate::ffi::{sp_wpkorg35, sp_wpkorg35_compute, sp_wpkorg35_create, sp_wpkorg35_destroy, sp_wpkorg35_init};
use crate::soundpipe::Soundpipe;

#[derive(UGenMacro)]
pub struct WpKorg35 {
    sp: Soundpipe,
    ffi: *mut sp_wpkorg35,
}

impl WpKorg35 {
    pub fn set_cutoff(&self, cutoff: f32) {
        unsafe {
            (*self.ffi).cutoff = cutoff;
        }
    }

    /// resonance should be between 0.0 and 2.0
    /// defaul = 1.0
    pub fn set_res(&self, resonance: f32) {
        unsafe {
            (*self.ffi).res = resonance;
        }
    }

    pub fn set_saturation(&self, saturation: f32) {
        unsafe {
            (*self.ffi).saturation = saturation;
        }
    }

    pub fn compute_internal(&self, input: &mut f32, output: &mut f32) {
        unsafe {
            sp_wpkorg35_compute(*self.sp.sp_ffi, self.ffi, input, output);
        }
    }

    pub fn compute(&self, input: f32) -> f32 {
        let mut out: f32 = 0.0;
        let mut input = input;
        self.compute_internal(&mut input, &mut out);
        out
    }
}
