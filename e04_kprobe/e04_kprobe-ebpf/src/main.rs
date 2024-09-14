#![no_std]
#![no_main]

use aya_ebpf::{macros::kprobe, programs::ProbeContext};
use aya_log_ebpf::info;

#[kprobe]
pub fn e04_kprobe(ctx: ProbeContext) -> u32 {
    match try_e04_kprobe(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_e04_kprobe(ctx: ProbeContext) -> Result<u32, u32> {
    info!(&ctx, "function try_to_wake_up called");
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
