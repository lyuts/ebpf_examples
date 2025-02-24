#![no_std]
#![no_main]

use aya_ebpf::{macros::fentry, programs::FEntryContext};
use aya_log_ebpf::info;

#[fentry(function = "xyz")]
pub fn e06_fentry(ctx: FEntryContext) -> u32 {
    match try_e06_fentry(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_e06_fentry(ctx: FEntryContext) -> Result<u32, u32> {
    info!(&ctx, "function xyz called");
    Ok(0)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
