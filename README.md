# Chapter3 分时多任务系统之一 非抢占式调度（优先级1）
## 完成情况
- 基本功能都已完成，采用yiled来放弃CPU，让下一个程序运行
- 使用吴一凡同学的3个函数来测试，目前只是实现非抢占调度，还未保障公平性
- 每一个程序都分配一个用户栈和内核栈，用Scheduler来记录目前程序的状态，从而调度
- Scheduler里的get_ptr实现了switch的调度功能
- 目前主要缺点就是内核代码需要随着用户程序的个数来进行修改，不太灵活，后续的章节会改进
```
OpenSBI v0.6
   ____                    _____ ____ _____
  / __ \                  / ____|  _ \_   _|
 | |  | |_ __   ___ _ __ | (___ | |_) || |
 | |  | | '_ \ / _ \ '_ \ \___ \|  _ < | |
 | |__| | |_) |  __/ | | |____) | |_) || |_
  \____/| .__/ \___|_| |_|_____/|____/_____|
        | |
        |_|

Platform Name          : QEMU Virt Machine
Platform HART Features : RV64ACDFIMSU
Platform Max HARTs     : 8
Current Hart           : 0
Firmware Base          : 0x80000000
Firmware Size          : 120 KB
Runtime SBI Version    : 0.2

MIDELEG : 0x0000000000000222
MEDELEG : 0x000000000000b109
PMP0    : 0x0000000080000000-0x000000008001ffff (A)
PMP1    : 0x0000000000000000-0xffffffffffffffff (A,R,W,X)
[S] mod interrupt init
AAAAAAAAAA [1/5]
[S] yield (:
BBBBBBBBBB [1/5]
[S] yield (:
AAAAAAAAAA [2/5]
[S] yield (:
BBBBBBBBBB [2/5]
[S] yield (:
AAAAAAAAAA [3/5]
[S] yield (:
BBBBBBBBBB [3/5]
[S] yield (:
AAAAAAAAAA [4/5]
[S] yield (:
BBBBBBBBBB [4/5]
[S] yield (:
AAAAAAAAAA [5/5]
[S] yield (:
BBBBBBBBBB [5/5]
[S] yield (:
Test write_a OK!
[S] exit (:
Test write_b OK!
[S] exit (:
CCCCCCCCCC [1/3]
[S] yield (:
CCCCCCCCCC [2/3]
[S] yield (:
CCCCCCCCCC [3/3]
[S] yield (:
Test write_c OK!
[S] exit (:
panic: '[S] all app end '
```
## 主要动机
提高整个应用的CPU利用率

多任务，因此需要实现任务切换，可采用如下方法：

- 批处理：在内存中放多个程序，执行完一个再执行下一个。当执行IO操作时，采用的是忙等的方式，效率差。
- 非抢占切换：CPU和I/O设备之间速度不匹配矛盾，程序之间的公平性。当一个程序主动要求暂停或退出时，换另外一个程序执行CPU计算。
>> 这时，可能需要引入中断（但中断不是本章主要的内容，如果不引入更好）。

## 用户程序
两个程序放置在一个不同的固定的物理地址上（这样不需要页表机制等虚存能力），完成的功能为：一个程序完成一些计算&输出，主动暂停，OS切换到另外一个程序执行，交替运行。

- count_multiplication：一维数组的乘法，并输出结果
- count_sum：累加一维数组的和，并输出结果
- [wyf 的具体实现]三个输出小程序
## 内核应完成功能
实现通过 sys_yield 交出当前任务的 CPU 所有权，通过 sys_exit 表明任务结束。需要为每个任务分配一个用户栈和内核栈，且需要实现类似 switch 用来任务切换的函数。

- ys_yield：让出CPU
- sys_exit：退出当前任务并让出 CPU
## 实现备注
重点是实现switch

当所有任务运行结束后退出内核