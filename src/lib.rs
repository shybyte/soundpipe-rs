use crate::ffi::sp_create;
use std::rc::Rc;
use crate::ffi::{sp_data, sp_destroy, sp_blsaw, sp_blsaw_create, sp_blsaw_init, sp_blsaw_compute, sp_blsaw_destroy};
use std::ptr::null_mut;

pub mod ffi;

#[derive(Clone)]
pub struct Soundpipe {
    sp_ffi: Rc<*mut sp_data>,
}

unsafe impl Send for Soundpipe {}

impl Soundpipe {
    pub fn new(sample_rate: i32) -> Self {
        let mut sp = null_mut();
        unsafe {
            sp_create(&mut sp);
            (*sp).sr = sample_rate;
        }
        Soundpipe { sp_ffi: Rc::new(sp) }
    }

    pub fn bl_saw(&self) -> BlSaw {
        BlSaw::new(self.clone())
    }
}


impl Drop for Soundpipe {
    fn drop(&mut self) {
        if let Some(sp) = Rc::get_mut(&mut self.sp_ffi) {
            unsafe {
                sp_destroy(sp);
            }
        }
    }
}

pub struct BlSaw {
    sp: Soundpipe,
    ffi: *mut sp_blsaw,
}

unsafe impl Send for BlSaw {}

impl BlSaw {
    fn new(sp: Soundpipe) -> Self {
        let mut result = BlSaw { sp: sp, ffi: null_mut() };
        unsafe {
            sp_blsaw_create(&mut result.ffi);
            sp_blsaw_init(*result.sp.sp_ffi, result.ffi);
        }
        result
    }

    pub fn set_freq(&self, freq: f32) {
        unsafe {
            *(*self.ffi).freq = freq;
        }
    }

    fn set_amp(&self, amp: f32) {
        unsafe {
            *(*self.ffi).amp = amp;
        }
    }

    pub fn compute(&self) -> f32 {
        let mut out: f32 = 0.0;
        let null = null_mut();
        unsafe {
            sp_blsaw_compute(*self.sp.sp_ffi, self.ffi, null, &mut out);
        }
        out
    }
}

impl Drop for BlSaw {
    fn drop(&mut self) {
        unsafe {
            sp_blsaw_destroy(&mut self.ffi);
        }
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
