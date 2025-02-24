#![no_std]
#![no_main]

use aya_ebpf::{macros::cgroup_skb, programs::SkBuffContext};
use aya_log_ebpf::info;

#[cgroup_skb]
pub fn e15_cgroup_skb(ctx: SkBuffContext) -> i32 {
    match try_e15_cgroup_skb(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_e15_cgroup_skb(ctx: SkBuffContext) -> Result<i32, i32> {
    info!(&ctx, "received a packet");
    Ok(0)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
