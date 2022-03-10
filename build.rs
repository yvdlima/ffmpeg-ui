
use std::env;

pub fn main() {
    let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-lib=static=avcodec");
    println!("cargo:rustc-link-lib=static=avfilter");
    println!("cargo:rustc-link-lib=static=avformat");
    println!("cargo:rustc-link-lib=static=avutil");
    println!("cargo:rustc-link-lib=static=swscale");
    println!("cargo:rustc-link-lib=static=swresample");
    println!("cargo:rustc-link-lib=static=x264");
    println!("cargo:rustc-link-search=native={}/src/ffmpeg/lib", src_dir);
}