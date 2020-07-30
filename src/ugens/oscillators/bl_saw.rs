use soundpipe_derive::{UGenMacro, OscillatorMacro};
use std::ptr::null_mut;

use crate::ffi::{sp_blsaw, sp_blsaw_compute, sp_blsaw_create, sp_blsaw_destroy, sp_blsaw_init};
use crate::soundpipe::Soundpipe;
use crate::ugens::oscillators::common::MonoOsc;

#[derive(UGenMacro,OscillatorMacro)]
pub struct BlSaw {
    sp: Soundpipe,
    ffi: *mut sp_blsaw,
}

impl BlSaw {
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

impl MonoOsc for BlSaw {}