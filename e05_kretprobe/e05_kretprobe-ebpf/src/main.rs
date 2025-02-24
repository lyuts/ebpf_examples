#![no_std]
#![no_main]

use aya_ebpf::{macros::kretprobe, programs::RetProbeContext};
use aya_log_ebpf::info;

#[kretprobe]
pub fn e05_kretprobe(ctx: RetProbeContext) -> u32 {
    match try_e05_kretprobe(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_e05_kretprobe(ctx: RetProbeContext) -> Result<u32, u32> {
    info!(&ctx, "kretprobe called");
    Ok(0)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
