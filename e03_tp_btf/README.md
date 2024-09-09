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

NOTE: this example relies on BTF, and expects `/sys/kernel/btf/vmlinux` to exist. This may not be available if you are
running examples in a virtual machine (e.g. Xen), and either will require reconfiguration or not work at all.

