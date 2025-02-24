#![no_std]
#![no_main]

use aya_ebpf::{macros::lsm, programs::LsmContext};
use aya_log_ebpf::info;

#[lsm(hook = "path_mkdir")]
pub fn path_mkdir(ctx: LsmContext) -> i32 {
    match try_path_mkdir(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_path_mkdir(ctx: LsmContext) -> Result<i32, i32> {
    info!(&ctx, "lsm hook path_mkdir called");
    Ok(0)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
