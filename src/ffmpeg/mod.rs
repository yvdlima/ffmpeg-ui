#![allow(
    non_camel_case_types,
    clashing_extern_declarations,
    non_upper_case_globals,
    non_snake_case,
    improper_ctypes
)]

use std::ffi::CStr;
use std::collections::HashMap;

use weblog::console_log;

pub mod avcodec;
pub mod avfilter;
pub mod avformat;
pub mod avutil;
pub mod swresample;
pub mod swscale;

pub fn get_versions_map() -> HashMap<String, String> {
    let mut lib_versions: HashMap<String , String> = HashMap::new();

    let avcodec_cstr = unsafe {
        let _version = avcodec::av_version_info();
        CStr::from_ptr(_version)
    };

    let avfilter_cstr = unsafe {
        let _version = avfilter::av_version_info();
        CStr::from_ptr(_version)
    };

    let avformat_cstr = unsafe {
        let _version = avformat::av_version_info();
        CStr::from_ptr(_version)
    };

    let avutil_cstr = unsafe {
        let _version = avutil::av_version_info();
        CStr::from_ptr(_version)
    };

    let swresample_cstr = unsafe {
        let _version = swresample::av_version_info();
        CStr::from_ptr(_version)
    };
    
    let swscale_cstr = unsafe {
        let _version = swscale::av_version_info();
        CStr::from_ptr(_version)
    };

    lib_versions.insert("avcodec".to_owned(), avcodec_cstr.to_str().unwrap().to_owned());
    lib_versions.insert("avfilter".to_owned(), avfilter_cstr.to_str().unwrap().to_owned());
    lib_versions.insert("avformat".to_owned(), avformat_cstr.to_str().unwrap().to_owned());
    lib_versions.insert("avutil".to_owned(), avutil_cstr.to_str().unwrap().to_owned());
    lib_versions.insert("swresample".to_owned(), swresample_cstr.to_str().unwrap().to_owned());
    lib_versions.insert("swscale".to_owned(), swscale_cstr.to_str().unwrap().to_owned());

    return lib_versions;
}

pub fn print_version() {
    
    let versions = get_versions_map();

    console_log!("FFmpeg libs versions -- ");

    for(libname, version) in versions {
        console_log!(libname,  "->",  version);    
    }
}
