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
