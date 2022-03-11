认识 BPF
https://ebpf.io/zh-cn/
什么是 BPF
BPF 概述与能力
BPF 具有在内核事件来临时执行一小段代码的能力。
内核5.2 之前，机器指令数限制为 4096，后开放到 100 万条。
支持嵌套的 BPF 程序
JIT，支持与原生编译的内核态代码一样的执行性能

BPF 程序类型

1. 动态探针
- kprobe
- kretprobe
- uprobe
- uretprobe

2. 静态探针
- tracepoint
- USTD

3. XDP：当网卡驱动程序收到数据包时，执行 BPF 程序。
4. cgroup类型、套接字过滤、perf 事件程序等等。。。

```
#include<linux/bpf.h>
#define SEC(NAME) __attribute__((section(NAME), used))

SEC("tracepoint/syscalls/sys_enter_execve")

int bpf_prog(void *ctx) 
{
    char msg[] = "Hello BPF!";
    bpf_trace_printk(msg, sizeof(msg));
    return 0;
}

char _license[] SEC("license") = "GPL";
//   /sys/kernel/debug/tracing/trace_pipe
//   clang -O2 -target bpf -c bpf.c -o bpf.o
```

```

```
