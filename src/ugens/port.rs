use crate::ffi::{sp_port, sp_port_compute, sp_port_create, sp_port_destroy, sp_port_init};
use crate::soundpipe::Soundpipe;
use crate::ugens::ugen::UGen;

pub struct Port {
    sp: Soundpipe,
    ffi: *mut sp_port,
}

impl Port {
    pub fn new(sp: Soundpipe, htime: f32) -> Self {
        println!("Create Port");
        let mut result = Self {
            sp,
            ffi: std::ptr::null_mut(),
        };
        unsafe {
            sp_port_create(&mut result.ffi);
            sp_port_init(*result.sp.sp_ffi, result.ffi, htime);
        }
        result
    }

    pub fn compute_internal(&self, input: &mut f32, output: &mut f32) {
        unsafe {
            sp_port_compute(*self.sp.sp_ffi, self.ffi, input, output);
        }
    }

    pub fn compute(&self, input: f32) -> f32 {
        let mut out: f32 = 0.0;
        let mut input = input;
        self.compute_internal(&mut input, &mut out);
        out
    }
}


unsafe impl Send for Port {}

impl UGen for Port {}

impl Drop for Port {
    fn drop(&mut self) {
        unsafe {
            println!("Drop {}", stringify!(#name));
            sp_port_destroy(&mut self.ffi);
        }
    }
}