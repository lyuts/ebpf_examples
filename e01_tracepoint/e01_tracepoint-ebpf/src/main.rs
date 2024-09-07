#![no_std]
#![no_main]

mod bindings;
use bindings::{trace_entry, trace_event_raw_net_dev_template};

use aya_ebpf::{
    helpers::bpf_probe_read_kernel_str_bytes, macros::tracepoint, programs::TracePointContext,
    EbpfContext,
};
use aya_log_ebpf::{debug, info};

#[tracepoint]
pub fn e01_tracepoint(ctx: TracePointContext) -> u32 {
    match try_e01_tracepoint(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_e01_tracepoint(ctx: TracePointContext) -> Result<u32, u32> {
    info!(&ctx, "tracepoint net_dev_queue called");

    // Every trace event has trace_entry as its header, so we can peek into it and have conditional
    // logic based on what we discover, e.g. handle net_dev_queue and netif_receive_skb
    // differently, we just need to inspect the `type_` field.
    let trace_entry_header: trace_entry = unsafe {
        ctx.read_at::<trace_entry>(0).unwrap()
    };

    debug!(&ctx, "trace_entry: type = {}, flags = {}, preempt_count = {}, pid = {}",
           trace_entry_header.type_,
           trace_entry_header.flags,
           trace_entry_header.preempt_count,
           trace_entry_header.pid,
           );

    let event: trace_event_raw_net_dev_template = unsafe {
        match ctx.read_at::<trace_event_raw_net_dev_template>(0) {
            Ok(ev) => ev,
            Err(_) => {
                info!(&ctx, "unable to cast ");
                return Err(1);
            }
        }
    };
    info!(
        &ctx,
        "tracepoint event: type = {}, len = {}", event.ent.type_, event.len
    );

    unsafe {
        let mut buf = [0u8; 32];
        let offset = (event.__data_loc_name & 0xffff) as usize;
        let len = (event.__data_loc_name >> 16 & 0xffff) as usize - 1; // -1 is for null-termination.
        let bytes =
            bpf_probe_read_kernel_str_bytes(ctx.as_ptr().add(offset) as *const u8, &mut buf)
                .unwrap();
        let devname = core::str::from_utf8_unchecked(bytes);
        info!(&ctx, "tracepoint event: devname = {}", devname);
    }

    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
