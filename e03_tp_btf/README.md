# e03_tp_btf

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

NOTE: this example relies on BTF, and expects `/sys/kernel/btf/vmlinux` to exist. This may not be available if you are
running examples in a virtual machine (e.g. Xen), and either will require reconfiguration or not work at all.

This is another example of an eBPF program that attaches to a tracepoint. The difference between tracepoint and
raw_tracepoint has been covered in e02. How is this one different from the other two? This example utilizes BTF, BPF
Type Format, encoding of the information about the types used in the kernel, so that eBPF programs can reference data in
a portable way. BTF enables CO-RE (compile once, run everywhere) approach of eBPF. For comparison, in previous examples
we referenced `sk_buff`, and the Rust bindings were generated on a machine with (at the time of writing) Linux 6.5. If
the layout of `sk_buff` changes in future it would be eBPF program author's responsibility to first, update it to make
it work on the newer versions of the kernel, and second, maintain backwards compatibility with different versions of the
kernel. With BTF, there is no such need, all access to BTF types is adjusted transparently to eBPF program and
difference between kernel versions are handled by eBPF itself.

## What's the cost of BTF?

