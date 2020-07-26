pub mod factory;
pub mod ffi;
pub mod soundpipe;
pub mod ugens;

pub use factory::Factory as SoundpipeFactory;
pub use soundpipe::Soundpipe;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
