#![no_std]
#![no_main]

use aya_ebpf::{macros::cgroup_sysctl, programs::SysctlContext};
use aya_log_ebpf::info;

#[cgroup_sysctl]
pub fn e17_cgroup_sysctl(ctx: SysctlContext) -> i32 {
    match try_e17_cgroup_sysctl(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_e17_cgroup_sysctl(ctx: SysctlContext) -> Result<i32, i32> {
    info!(&ctx, "sysctl operation called");
    Ok(0)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
