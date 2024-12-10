# e04_kprobe

## Prerequisites

1. Install bpf-linker: `cargo install bpf-linker`

## Build eBPF

```bash
cargo xtask build-ebpf
```

To perform a release build you can use the `--release` flag.
You may also change the target architecture with the `--target` flag.

## Build Userspace

```bash
cargo build
```

## Build eBPF and Userspace

```bash
cargo xtask build
```

## Run

```bash
RUST_LOG=info cargo xtask run
```

## What is this program doing?
This is an example of an eBPF program that attaches a kprobe. What's a kprobe?

```bash
user@dev:linux-6.5/Documentation$ grep Overview -A7 trace/kprobetrace.rst
Overview
--------
These events are similar to tracepoint-based events. Instead of tracepoints,
this is based on kprobes (kprobe and kretprobe). So it can probe wherever
kprobes can probe (this means, all functions except those with
__kprobes/nokprobe_inline annotation and those marked NOKPROBE_SYMBOL).
Unlike the tracepoint-based event, this can be added and removed
dynamically, on the fly.
```

The following will list available kprobes:

```bash
sudo cat /sys/kernel/debug/tracing/available_filter_functions
__traceiter_initcall_level
__probestub_initcall_level
__traceiter_initcall_start
__probestub_initcall_start
__traceiter_initcall_finish
__probestub_initcall_finish
trace_initcall_finish_cb
trace_initcall_start_cb
run_init_process
initcall_blacklisted
...
```

For the purpose of this example we are going to use the kprobe for
netdev_get_name.

```bash
user@dev:linux-6.5$ grep netdev_get_name net/core/dev.c
 *      netdev_get_name - get a netdevice name, knowing its ifindex.
int netdev_get_name(struct net *net, char *name, int ifindex)
```

To trigger execution of netdev_get_name function in the kernel, use the
auxiliary code in this exaple by running the following:
```bash
$ cargo xtask test
```
