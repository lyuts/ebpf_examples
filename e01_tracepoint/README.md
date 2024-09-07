# e01_tracepoint

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

This example attaches an program to the `net_dev_queue` tracepoint. This tracepoint is hit every time there is something
available to be transmitted for a network device.

## What is a tracepoint?

A tracepoint is a predefined point or location in the code. Once it is reached a certain action is executed. Typically,
a custom hook is called. There are many tracepoints defined in the kernel code (600+ tracepoints in Linux 6.5).
Tracepoints are defined in `include/trace/events/`.

```
user@dev:linux-6.5$ rg DEFINE_EVENT include/trace/events/ | head
include/trace/events/sock.h:DEFINE_EVENT(sock_msg_length, sock_send_length,
include/trace/events/sock.h:DEFINE_EVENT(sock_msg_length, sock_recv_length,
include/trace/events/net.h:DEFINE_EVENT(net_dev_template, net_dev_queue,
include/trace/events/net.h:DEFINE_EVENT(net_dev_template, netif_receive_skb,
include/trace/events/net.h:DEFINE_EVENT(net_dev_template, netif_rx,
include/trace/events/net.h:DEFINE_EVENT(net_dev_rx_verbose_template, napi_gro_frags_entry,
include/trace/events/net.h:DEFINE_EVENT(net_dev_rx_verbose_template, napi_gro_receive_entry,
include/trace/events/net.h:DEFINE_EVENT(net_dev_rx_verbose_template, netif_receive_skb_entry,
include/trace/events/net.h:DEFINE_EVENT(net_dev_rx_verbose_template, netif_receive_skb_list_entry,
include/trace/events/net.h:DEFINE_EVENT(net_dev_rx_verbose_template, netif_rx_entry,
```

For instance, this example eBPF program sets up a tracepoint for net_dev_queue. This tracepoint is defined in
`include/trace/events.net.h`:

```c
DEFINE_EVENT(net_dev_template, net_dev_queue,

    TP_PROTO(struct sk_buff *skb),

    TP_ARGS(skb)
);
```

The points of interest here are the name of the tracepoint and the type of the argument supplied to it, i.e. data that
will available to our eBPF program.

As for the actual location of the tracepoint, it can be found in `net/core/dev.c` inside `__dev_queue_xmit` function:

```c
int __dev_queue_xmit(struct sk_buff *skb, struct net_device *sb_dev)
{
    ...

    if (!txq)
        txq = netdev_core_pick_tx(dev, skb, sb_dev);

    q = rcu_dereference_bh(txq->qdisc);

    trace_net_dev_queue(skb);
    if (q->enqueue) {
        rc = __dev_xmit_skb(skb, q, dev, txq);
        goto out;
    }
    ...
}
```

