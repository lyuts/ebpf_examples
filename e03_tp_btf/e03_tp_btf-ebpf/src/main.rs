#![no_std]
#![no_main]

mod bindings;

use aya_ebpf::{macros::btf_tracepoint, programs::BtfTracePointContext};
use aya_log_ebpf::info;
use bindings::{net_device, sk_buff};

#[btf_tracepoint(function = "net_dev_queue")]
pub fn net_dev_queue(ctx: BtfTracePointContext) -> i32 {
    info!(&ctx, "tracepoint net_dev_queue called");
    match try_net_dev_queue(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

#[btf_tracepoint(function = "netif_receive_skb")]
pub fn netif_receive_skb(ctx: BtfTracePointContext) -> i32 {
    info!(&ctx, "tracepoint netif_receive_skb called");
    match try_net_dev_queue(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_net_dev_queue(ctx: BtfTracePointContext) -> Result<i32, i32> {
    info!(&ctx, "tracepoint net_dev_queue called");
    unsafe {
        let skb: *const sk_buff = ctx.arg(1);
        // `__bindgen_anon_1`s are just the names of the fields that correspond to anonymous
        // unions in C structs definition.
        //
        // ```c
        // struct sk_buff {
        //     union {
        //         struct {
        //             /* These two members must be first to match sk_buff_head. */
        //             struct sk_buff      *next;
        //             struct sk_buff      *prev;
        //             union {
        //                 struct net_device   *dev;
        //                 /* Some protocols might use this space to store information,
        //                  * while device pointer would be NULL.
        //                  * UDP receive path is one user.
        //                  */
        //                 unsigned long       dev_scratch;
        //             };
        //         };
        //         struct rb_node      rbnode; /* used in netem, ip4 defrag, and tcp stack */
        //         struct list_head    list;
        //         struct llist_node   ll_node;
        //     };
        //         ...
        // };
        // ```
        //
        // Rust bindings can modified to give these fields descriptive names.
        let net_dev: *const net_device = (*skb)
            .__bindgen_anon_1
            .__bindgen_anon_1
            .__bindgen_anon_1
            .dev;
        // 16 bytes becase the `name` field in the net_device struct definition has a fixed size.
        let devname: &[u8] = core::slice::from_raw_parts((*net_dev).name.as_ptr() as *const u8, 16);
        info!(
            &ctx,
            "devname = {}",
            core::str::from_utf8_unchecked(devname)
        );
    };
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
