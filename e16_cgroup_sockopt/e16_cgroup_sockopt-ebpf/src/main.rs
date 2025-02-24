#![no_std]
#![no_main]

use aya_ebpf::{macros::cgroup_sockopt, programs::SockoptContext};
use aya_log_ebpf::info;

#[cgroup_sockopt(setsockopt)]
pub fn e16_cgroup_sockopt(ctx: SockoptContext) -> i32 {
    match try_e16_cgroup_sockopt(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_e16_cgroup_sockopt(ctx: SockoptContext) -> Result<i32, i32> {
    info!(&ctx, "setsockopt called");
    Ok(0)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
