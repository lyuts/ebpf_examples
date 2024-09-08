#![no_std]
#![no_main]

mod bindings;

use aya_ebpf::{
    bindings::bpf_raw_tracepoint_args,
    helpers::{bpf_probe_read, bpf_probe_read_kernel, bpf_probe_read_kernel_str_bytes},
    macros::raw_tracepoint,
    programs::RawTracePointContext,
    EbpfContext,
};
use aya_log_ebpf::info;
use bindings::sk_buff;

#[raw_tracepoint(tracepoint = "net_dev_queue")]
pub fn e02_raw_tracepoint(ctx: RawTracePointContext) -> i32 {
    match try_e02_raw_tracepoint(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_e02_raw_tracepoint(ctx: RawTracePointContext) -> Result<i32, i32> {
    info!(&ctx, "tracepoint net_dev_queue called. pid = {}", ctx.pid());
    //
    // ```
    // DEFINE_EVENT(net_dev_template, net_dev_queue,
    //
    //      TP_PROTO(struct sk_buff *skb),   <<<<< args
    //
    //      TP_ARGS(skb)
    // );
    // ```
    let tp_args: &bpf_raw_tracepoint_args =
        unsafe { &*{ ctx.as_ptr() as *mut bpf_raw_tracepoint_args } };
    let args = unsafe { tp_args.args.as_slice(1) };

    let skb: sk_buff = unsafe { bpf_probe_read_kernel(args[0] as *const sk_buff).unwrap() };

    info!(
        &ctx,
        "net_dev_queue event. mac_len = {}, hdr_len = {}", skb.mac_len, skb.hdr_len
    );

    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
