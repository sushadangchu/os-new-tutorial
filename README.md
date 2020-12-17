# Chapter5 进程及重要系统调用（优先级1）
## 主要动机
应用以进程的方式进行运行，简化了应用开发的负担，OS也更好管理

引入重要的进程概念，整合Chapt1~4的内容抽象出进程，实现一系列相关机制及 syscall

## 用户程序
用户终端 user_shell 以及一些相应的测试

## 内核应完成功能
实现完整的子进程机制，初始化第一个用户进程 initproc。

## 新增系统调用
- sys_fork
- sys_wait(轮询版)
- sys_exec
- sys_getpid
- sys_yield更新
- sys_exit 更新
- sys_read：终端需要从串口读取命令