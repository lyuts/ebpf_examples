#![no_std]
#![no_main]

use aya_ebpf::{macros::sock_ops, programs::SockOpsContext};
use aya_log_ebpf::info;

#[sock_ops]
pub fn e20_sock_ops(ctx: SockOpsContext) -> u32 {
    match try_e20_sock_ops(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_e20_sock_ops(ctx: SockOpsContext) -> Result<u32, u32> {
    info!(&ctx, "received TCP connection");
    Ok(0)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
