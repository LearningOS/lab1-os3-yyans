



# 简答作业

2.1 

a0：kernel stack 地址

case1: start running app by __restore

case2: back to U after handling trap

2.2

sstatus: 提供trap前特权级信息

sepc: 记录 Trap 发生之前执行的最后一条指令的地址

sscratch: kernel  stack 和 user stack 交换的中介

2.3

x2是sp寄存器，应留着最后恢复

x4几轮core编号

2.4

sscratch->kernel stack, sp->user stack

2.5

CPU 会将当前的特权级按照 `sstatus` 的 `SPP` 字段设置为 U 或者 S ；

CPU 会跳转到 `sepc` 寄存器指向的那条指令，然后继续执行。

2.6

sp->kernel stack, sscratch->user stack

2.7

ecall