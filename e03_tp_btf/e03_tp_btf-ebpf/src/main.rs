#![no_std]
#![no_main]

use aya_ebpf::{
    macros::btf_tracepoint,
    programs::BtfTracePointContext,
};
use aya_log_ebpf::info;

#[btf_tracepoint(function="net_dev_queue")]
pub fn net_dev_queue(ctx: BtfTracePointContext) -> i32 {
    info!(&ctx, "tracepoint net_dev_queue called");
    match try_net_dev_queue(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

#[btf_tracepoint(function="netif_receive_skb")]
pub fn netif_receive_skb(ctx: BtfTracePointContext) -> i32 {
    info!(&ctx, "tracepoint netif_receive_skb called");
    match try_net_dev_queue(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_net_dev_queue(ctx: BtfTracePointContext) -> Result<i32, i32> {
    info!(&ctx, "tracepoint net_dev_queue called");
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
