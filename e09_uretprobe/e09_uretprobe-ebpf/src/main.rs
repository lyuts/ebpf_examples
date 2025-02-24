#![no_std]
#![no_main]

use aya_ebpf::{macros::uretprobe, programs::RetProbeContext};
use aya_log_ebpf::info;

#[uretprobe]
pub fn e09_uretprobe(ctx: RetProbeContext) -> u32 {
    match try_e09_uretprobe(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_e09_uretprobe(ctx: RetProbeContext) -> Result<u32, u32> {
    info!(&ctx, "function getaddrinfo called by libc");
    Ok(0)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
