#![no_std]
#![no_main]

use aya_ebpf::{
    macros::{map, sk_msg},
    maps::SockHash,
    programs::SkMsgContext,
};
use aya_log_ebpf::info;
use e19_sk_msg_common::SockKey;

#[map]
static XYZ: SockHash<SockKey> = SockHash::<SockKey>::with_max_entries(1024, 0);

#[sk_msg]
pub fn e19_sk_msg(ctx: SkMsgContext) -> u32 {
    match try_e19_sk_msg(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_e19_sk_msg(ctx: SkMsgContext) -> Result<u32, u32> {
    info!(&ctx, "received a message on the socket");
    Ok(0)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
