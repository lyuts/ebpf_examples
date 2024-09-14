# e02_raw_tracepoint

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

The program serves as an example of a raw_tracepoint eBPF program. This program is very similar to e01_tracepoint, but
has still has subtle differences. We will mostly focus on them. This program being a raw tracepoint eBPF program means
that the kernel doesn't do any preprocessing of event's data for it. Event's data is also not augmented with trace_entry
metadata. This may mean the eBPF program is getting less information about the event, but at the same time, it takes
less cycles for the kernel to prepare the contex for the program, and so raw tracepoint typically has better
performance.

First, the context of a raw_tracepoint program is different. raw_tracepoint program takes in `bpf_raw_tracepoint_args`.

```bash
user@dev:linux-6.5$ grep -A2 bpf_raw_tracepoint_args include/uapi/linux/bpf.h
struct bpf_raw_tracepoint_args {
        __u64 args[0];
};
```

Useful reference: https://github.com/libbpf/libbpf/blob/master/src/libbpf.c#L6684-L6714 maps a program type to the type of the argument it is getting.

To figure out what these `args` point to we need to look at the definition of the trace event:

```bash
user@dev:linux-6.5$ grep -A5 net_dev_queue include/trace/events/net.h
DEFINE_EVENT(net_dev_template, net_dev_queue,

        TP_PROTO(struct sk_buff *skb),

        TP_ARGS(skb)
);
```

This tells us that we are actually getting a single argument of type `struct sk_buff *`.  Remember this is a raw
tracepoint program. Unlike previous example where we could just take the context and use it, we have to do additional
work to get data from the sk_buff. The program is getting a pointer which is pointing to kernel memory. This memory
isn't available to our program right away. That's why we are calling `bpf_probe_read_kernel` before we can use any of
the sk_buff data.

## Further reading
- https://mozillazg.com/2022/06/ebpf-libbpf-btf-powered-enabled-raw-tracepoint-common-questions-en.html#hidthe-difference-between-btf-raw-tracepoint-and-raw-tracepoint
- https://nakryiko.com/posts/bpf-core-reference-guide/#btf-enabled-bpf-program-types-with-direct-memory-reads
- https://yuki-nakamura.com/2024/08/12/writing-ebpf-rawtracepoint-program-with-rust-aya/
