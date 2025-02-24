#![no_std]
#![no_main]

use aya_ebpf::{macros::fexit, programs::FExitContext};
use aya_log_ebpf::info;

#[fexit(function = "xyz")]
pub fn e07_fexit(ctx: FExitContext) -> u32 {
    match try_e07_fexit(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_e07_fexit(ctx: FExitContext) -> Result<u32, u32> {
    info!(&ctx, "function xyz called");
    Ok(0)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
