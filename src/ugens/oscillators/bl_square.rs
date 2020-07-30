use soundpipe_derive::{UGenMacro, OscillatorMacro};
use std::ptr::null_mut;

use crate::ffi::{
    sp_blsquare, sp_blsquare_compute, sp_blsquare_create, sp_blsquare_destroy, sp_blsquare_init,
};
use crate::soundpipe::Soundpipe;
use crate::ugens::oscillators::common::MonoOsc;

#[derive(UGenMacro,OscillatorMacro)]
pub struct BlSquare {
    sp: Soundpipe,
    ffi: *mut sp_blsquare,
}

impl BlSquare {
    pub fn set_freq(&self, freq: f32) {
        unsafe {
            *(*self.ffi).freq = freq;
        }
    }

    pub fn set_amp(&self, amp: f32) {
        unsafe {
            *(*self.ffi).amp = amp;
        }
    }
}

impl MonoOsc for BlSquare {}