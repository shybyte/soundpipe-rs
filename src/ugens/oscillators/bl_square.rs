use soundpipe_derive::{OscillatorMacro, UGenMacro};
use std::ptr::null_mut;

use crate::ffi::{
    sp_blsquare, sp_blsquare_compute, sp_blsquare_create, sp_blsquare_destroy, sp_blsquare_init,
};
use crate::soundpipe::Soundpipe;
use crate::ugens::oscillators::common::MonoOsc;

#[derive(UGenMacro, OscillatorMacro)]
pub struct BlSquare {
    sp: Soundpipe,
    ffi: *mut sp_blsquare,
}

impl BlSquare {
    pub fn set_freq(&mut self, freq: f32) {
        unsafe {
            *(*self.ffi).freq = freq;
        }
    }

    pub fn set_amp(&mut self, amp: f32) {
        unsafe {
            *(*self.ffi).amp = amp;
        }
    }

    pub fn set_width(&mut self, width: f32) {
        unsafe {
            *(*self.ffi).width = width;
        }
    }
}

impl MonoOsc for BlSquare {}
