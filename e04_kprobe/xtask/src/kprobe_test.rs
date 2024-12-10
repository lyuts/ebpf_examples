use clap::Parser;
use libc::{c_char, if_indextoname, IF_NAMESIZE};
use std::ffi::CStr;

#[derive(Debug, Parser)]
pub struct Options {}

pub fn run(_opts: Options) -> Result<(), anyhow::Error> {
    let mut name_buf: [i8; IF_NAMESIZE] = [0; IF_NAMESIZE];
    unsafe {
        let _name: *mut c_char = if_indextoname(1, name_buf.as_mut_ptr());
    }
    let c_str = unsafe { CStr::from_ptr(name_buf.as_ptr()) };
    println!("==>> {}", c_str.to_string_lossy().into_owned());
    Ok(())
}
