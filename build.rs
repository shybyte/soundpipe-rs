fn main() {
    println!("cargo:rustc-flags=-l soundpipe");
    println!("cargo:rustc-flags=-l sndfile");
}
