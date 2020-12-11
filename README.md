# Chapter1 裸机应用（优先级1）
## 完成情况
- 可以实现裸机运行要求的2个app
- 使用ld文件对内核进行布局，具体的串口输出使用了opensbi来操作，内核直接调用opensbi提供的接口
- 2个系统调用基本为摆设，目前没有作用，因为内核和用户态还没有区分开，目前整个过程都是运行在s态的
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
hello world!
exit (:
sum = 15
exit (:
panic: 'end of rustmain'
```
## 主要动机
支持应用进行计算与结果输出。

在裸机上输出 Hello world，就像在其他 OS 上一样。

app列表：

- hello_world：输出字符串
- count_sum：累加一维数组的和，并输出结果

备注：不需要输入功能

## 内核应完成功能
内存地址空间：  
知道自己在内存的哪个位置。理解编译器生成的代码。

init：基本初始化  
主要是硬件加电后的硬件初始化，以前是OS做，后面给BIOS, bootloader等完成初步初始化。OS需要知道内存大小，IO分布。

write函数：输出字符串  
驱动串口的初始化，能够通过串口输出。

exit函数：表明程序结束

其它（不是主要的）：  
在 qemu/k210 平台上基于 RustSBI 跳转到内核，打印调试信息，支持内核堆内存分配。

## 章节分布
基本上和第二版/第三版一致。注意需要考虑上面的应用和功能。