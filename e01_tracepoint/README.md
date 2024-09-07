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

In another terminal try inspecting the the list of eBPF programs. You should see yours running, e.g.:

```bash
user@dev:~$ sudo bpftool prog list | grep -A3 tracepoint
41: tracepoint  name e01_tracepoint  tag 9dd24d2903052fb4  gpl
        loaded_at 2024-09-07T12:44:11-0400  uid 0
        xlated 5888B  jited 3163B  memlock 8192B  map_ids 26,27,28
```

## What is this program doing?

The program serves as example for:
- attaching an eBPF program to multiple tracepoints, `net_dev_queue` and `netif_receive_skb`. These tracepoints are hit
  every time there is something available to be transmitted/received for a network device.
- decoding the event data that is passed into the program.

## What is a tracepoint?

A tracepoint is a predefined point or location in the code. Once it is reached a certain action is executed. Typically,
a custom hook is called. There are many tracepoints defined in the kernel code (1400+ tracepoints in Linux 6.5).
You can find defined tracepoints in `include/trace/events/` (alternatively, `trace-cmd list`, or `cat
/sys/kernel/tracing/available_events`).

```bash
user@dev:linux-6.5$ rg "(DEFINE|TRACE)_EVENT" include/trace/events/ | head
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

For instance, this example eBPF program sets up a tracepoint for `net_dev_queue`. This tracepoint is defined in
`include/trace/events.net.h`:

```c
DEFINE_EVENT(net_dev_template, net_dev_queue,

    TP_PROTO(struct sk_buff *skb),

    TP_ARGS(skb)
);
```

Something of interest here are the name of the tracepoint and the type of the argument supplied to it, i.e. data that
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


`bindings.rs` is a generated file that contains Rust bindings for types defined in the kernel source, such as data that
gets passed into eBPF program and all dependenecy types. Bindings are generated using the following command:

```
aya-tool generate  trace_event_raw_net_dev_template sk_buff > e01_tracepoint/e01_tracepoint-ebpf/src/bindings.rs
```

How do we know what type name to pass to `aya-tool generate`? If we look at `DEFINE_EVENT` for `net_dev_queue` from the
`include/trace/events.net.h` above you can notice `net_dev_template`. That is the type of event (or "context").
While browsing trough event definitions in `include/trace/events/*.h` one can notice events being defined using either
`TRACE_EVENT` or `DEFINE_EVENT`. The difference between them is that `TRACE_EVENT` defines an new event with a unique
event/context type, whereas `DEFINE_EVENT` creates a new tracing event from a template, hence `net_dev_template`.
`net_dev_template` is used by several other tracing events, so its definition is moved into a common template:

```c
DECLARE_EVENT_CLASS(net_dev_template,

    TP_PROTO(struct sk_buff *skb),

    TP_ARGS(skb),

    TP_STRUCT__entry(
        __field(    void *,     skbaddr     )
        __field(    unsigned int,   len     )
        __string(   name,       skb->dev->name  )
    ),

    TP_fast_assign(
        __entry->skbaddr = skb;
        __entry->len = skb->len;
        __assign_str(name, skb->dev->name);
    ),

    TP_printk("dev=%s skbaddr=%p len=%u",
        __get_str(name), __entry->skbaddr, __entry->len)
)

DEFINE_EVENT(net_dev_template, net_dev_queue,

    TP_PROTO(struct sk_buff *skb),

    TP_ARGS(skb)
);

DEFINE_EVENT(net_dev_template, netif_receive_skb,

    TP_PROTO(struct sk_buff *skb), /* E: Expected ')'

    TP_ARGS(skb)
);

DEFINE_EVENT(net_dev_template, netif_rx,

    TP_PROTO(struct sk_buff *skb), /* E: Expected ')'

    TP_ARGS(skb)
);
```

An alternative way to discovering the context type is using sysfs:

```bash
$ sudo cat /sys/kernel/debug/tracing/events/net/net_dev_queue/format
name: net_dev_queue
ID: 1311
format:
	field:unsigned short common_type;	offset:0;	size:2;	signed:0;         \
	field:unsigned char common_flags;	offset:2;	size:1;	signed:0;          \ trace_entry
	field:unsigned char common_preempt_count;	offset:3;	size:1;	signed:0;  /
	field:int common_pid;	offset:4;	size:4;	signed:1;                     /

	field:void * skbaddr;	offset:8;	size:8;	signed:0;
	field:unsigned int len;	offset:16;	size:4;	signed:0;
	field:__data_loc char[] name;	offset:20;	size:4;	signed:1;

print fmt: "dev=%s skbaddr=%p len=%u", __get_str(name), REC->skbaddr, REC->len
```

First 4 fields are separated, they actually represent `trace_entry` struct. Another useful hint is the last line, which
tells us that the `name` field is actually the name of the device. You can confirm that with the output of the program,
it prints which device has data to send.

## What does tracepoint eBPF program return?

https://github.com/iovisor/bcc/issues/139

## Can you attach an eBPF program to multiple tracepoints?

Yes.

```bash
$ grep -B2 attach e01_tracepoint/src/main.rs
    let program: &mut TracePoint = bpf.program_mut("e01_tracepoint").unwrap().try_into()?;
    program.load()?;
    program.attach("net", "net_dev_queue")?;
    program.attach("net", "netif_receive_skb")?;
```

How can you distinguish between tracep events you get in the eBPF program?

```rust
    let trace_entry_header: trace_entry = unsafe {
        ctx.read_at::<trace_entry>(0).unwrap()
    };

    match trace_entry_header.type_ {
    ...
    todo: figure out which constant to match the type against...
    ...
    }
```

# Further reading
- https://docs.kernel.org/trace/tracepoints.html
