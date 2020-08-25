# Soundpipe bindings for Rust

[Soundpipe](https://github.com/PaulBatchelor/Soundpipe) is a lightweight music DSP library implemented in C.
Unfortunately the project has disappeared in August 2020, so I have created a [Soundpipe - backup](https://github.com/shybyte/soundpipe). 

## Regenerate bindings using bindgen

```bash
cargo install bindgen
bindgen src/ffi/wrapper.h --whitelist-function 'sp_.*' -o src/ffi/ffi-generated.rs
```

TODO: Create proper soundpipe-sys crate which generates the bindings at compile time 
(https://rust-lang.github.io/rust-bindgen/library-usage.html, https://kornel.ski/rust-sys-crate).

## License

MIT

## Copyright

Copyright (c) 2020 Marco Stahl