#![no_std]
#![no_main]

mod bindings;

use aya_ebpf::{macros::kprobe, programs::ProbeContext};
use aya_log_ebpf::info;
use bindings::net;

#[kprobe]
pub fn e04_kprobe(ctx: ProbeContext) -> u32 {
    match try_e04_kprobe(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_e04_kprobe(ctx: ProbeContext) -> Result<u32, u32> {
    info!(&ctx, "function try_to_wake_up called");
    let net: *const net = ctx.arg(0).unwrap();
    // let name = ctx.arg(1)?;
    // let ifindex: i32 = ctx.arg(2).ok_or(1)?;
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
