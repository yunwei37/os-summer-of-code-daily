# 目录

应要求，对所有 lab report 进行整合；

<!-- TOC -->

- [目录](#目录)
- [lab0 学习报告](#lab0-学习报告)
  - [环境配置](#环境配置)
    - [安装 QEMU](#安装-qemu)
    - [安装 Rust 工具链](#安装-rust-工具链)
    - [尝试运行 rCore](#尝试运行-rcore)
  - [lab 0: 跑起来](#lab-0-跑起来)
    - [创建 Rust 项目](#创建-rust-项目)
    - [移除运行时环境依赖](#移除运行时环境依赖)
    - [生成内核镜像](#生成内核镜像)
    - [调整内存布局](#调整内存布局)
    - [重写程序入口点 _start](#重写程序入口点-_start)
    - [使用 QEMU 运行内核](#使用-qemu-运行内核)
    - [接口封装和代码整理](#接口封装和代码整理)
- [lab1 学习报告](#lab1-学习报告)
  - [中断的概念：](#中断的概念)
  - [RISC-V 与中断相关的寄存器和指令](#risc-v-与中断相关的寄存器和指令)
    - [与中断相关的寄存器](#与中断相关的寄存器)
    - [与中断相关的指令](#与中断相关的指令)
  - [程序运行状态](#程序运行状态)
  - [状态的保存与恢复](#状态的保存与恢复)
  - [时钟中断](#时钟中断)
    - [实现时钟中断的处理流程](#实现时钟中断的处理流程)
- [实验一：中断](#实验一中断)
- [lab2 学习报告](#lab2-学习报告)
  - [动态内存分配](#动态内存分配)
    - [支持动态内存分配的方法](#支持动态内存分配的方法)
    - [动态内存分配测试](#动态内存分配测试)
  - [物理内存探测](#物理内存探测)
  - [物理内存管理:](#物理内存管理)
    - [分配和回收:](#分配和回收)
- [实验二：内存分配](#实验二内存分配)
- [lab3 学习报告](#lab3-学习报告)
  - [从虚拟内存到物理内存](#从虚拟内存到物理内存)
    - [原理](#原理)
    - [Sv39](#sv39)
    - [页表项](#页表项)
    - [多级页表](#多级页表)
    - [页表基址](#页表基址)
    - [快表（TLB）](#快表tlb)
  - [修改内核](#修改内核)
  - [实现页表](#实现页表)
    - [页表项](#页表项-1)
  - [页表](#页表)
  - [实现内核重映射](#实现内核重映射)
    - [内存段 Segment](#内存段-segment)
    - [Mapping](#mapping)
    - [MemorySet](#memoryset)
  - [总结：空间的划分和管理](#总结空间的划分和管理)
  - [页面置换](#页面置换)
    - [算法](#算法)
    - [这里的实现](#这里的实现)
- [实验三：虚实地址转换](#实验三虚实地址转换)
- [lab4 学习报告](#lab4-学习报告)
  - [线程和进程](#线程和进程)
    - [线程的表示](#线程的表示)
    - [进程的表示](#进程的表示)
  - [线程的创建](#线程的创建)
    - [执行第一个线程](#执行第一个线程)
    - [设计 Context](#设计-context)
    - [在启动时不打开中断](#在启动时不打开中断)
  - [线程的切换](#线程的切换)
    - [修改中断处理](#修改中断处理)
    - [线程切换](#线程切换)
    - [上下文 Context 的保存和取出](#上下文-context-的保存和取出)
  - [内核栈](#内核栈)
    - [具体实现](#具体实现)
  - [调度器](#调度器)
    - [处理器抽象](#处理器抽象)
    - [调度器](#调度器-1)
    - [运行！](#运行)
  - [线程的结束](#线程的结束)
    - [解决办法](#解决办法)
  - [总结](#总结)
- [实验四（上）：线程](#实验四上线程)
- [实验四（下）：线程调度](#实验四下线程调度)
- [lab5 学习报告](#lab5-学习报告)
  - [设备树](#设备树)
    - [设备树：](#设备树-1)
    - [解析设备树](#解析设备树)
  - [virtio](#virtio)
    - [挂载到 QEMU](#挂载到-qemu)
    - [virtio 节点探测](#virtio-节点探测)
    - [virtio_drivers 库](#virtio_drivers-库)
  - [驱动和块设备驱动](#驱动和块设备驱动)
    - [驱动抽象](#驱动抽象)
    - [抽象块设备](#抽象块设备)
    - [virtio-blk 块设备驱动](#virtio-blk-块设备驱动)
  - [文件系统](#文件系统)
    - [Simple File System](#simple-file-system)
    - [测试](#测试)
  - [小结](#小结)
- [lab6 学习报告](#lab6-学习报告)
  - [构建用户程序框架](#构建用户程序框架)
    - [基础框架搭建](#基础框架搭建)
  - [打包为磁盘镜像](#打包为磁盘镜像)
  - [解析 ELF 文件并创建线程](#解析-elf-文件并创建线程)
    - [解析各个字段](#解析各个字段)
    - [加载数据到内存中](#加载数据到内存中)
    - [运行 Hello World](#运行-hello-world)
  - [实现系统调用](#实现系统调用)
    - [用户程序中调用系统调用](#用户程序中调用系统调用)
    - [避免忙等待](#避免忙等待)
    - [实现系统调用的思路](#实现系统调用的思路)
  - [处理文件描述符](#处理文件描述符)
    - [实现输入输出流](#实现输入输出流)
  - [条件变量](#条件变量)
    - [调整调度器](#调整调度器)
    - [实现条件变量](#实现条件变量)
  - [总结](#总结-1)
- [实验六：系统调用](#实验六系统调用)

<!-- /TOC -->

# lab0 学习报告

## 环境配置

本机环境（新安装的虚拟机系统，上一个系统是搞xv6的qemu，感觉可能实验环境会有冲突不如再开一个）：

>Linux ubuntu 5.4.0-26-generic #30-Ubuntu SMP Mon Apr 20 16:58:30 UTC 2020 x86_64 x86_64 x86_64 GNU/Linux

已换源至阿里云，但版本过低；

### 安装 QEMU

- 先 `sudo apt-get install build-essential` 安装编译工具链；
- 下载源码编译：
  
```
wget https://download.qemu.org/qemu-4.2.0.tar.xz
tar xvJf qemu-4.2.0.tar.xz
cd qemu-4.2.0
./configure --target-list=riscv32-softmmu,riscv64-softmmu
```

- 报错`ERROR: glib-2.48 gthread-2.0 is required to compile QEMU`：通过 `sudo apt-get install libglib2.0-dev` 安装;
- 报错 `ERROR: pixman >= 0.21.8 not present` ，通过 `sudo apt-get install libpixman-1-dev` 安装
- 测试


```
yunwei@ubuntu:~/rcore$ qemu-system-riscv64 --version
QEMU emulator version 4.2.0
Copyright (c) 2003-2019 Fabrice Bellard and the QEMU Project developers

```

### 安装 Rust 工具链

- 运行`curl https://sh.rustup.rs -sSf | sh`；

### 尝试运行 rCore

- `git clone https://github.com/rcore-os/rCore-Tutorial`
- `make`

```
Failed to execute tool: objcopy
No such file or directory (os error 2)
make[1]: *** [Makefile:28: target/riscv64imac-unknown-none-elf/debug/kernel.bin] Error 101

```

可能是我先做了下面导致的问题，之后再说；

## lab 0: 跑起来

### 创建 Rust 项目

创建文件夹，并创建项目：

```
mkdir Project     
cd Project     
echo "nightly-2020-06-27" > rust-toolchain
cargo new os
cd os
```

- Test：`cargo run`

移除标准库依赖:

`main.rs`

```rs
#![no_std]

use core::panic::PanicInfo;

/// 当 panic 发生时会调用该函数
/// 我们暂时将它的实现为一个死循环
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

```

`Cargo.toml`

```toml
[package]
name = "os"
version = "0.1.0"
authors = ["yunwei"]
edition = "2018"

# panic 时直接终止，因为我们没有实现堆栈展开的功能
[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

### 移除运行时环境依赖

`main.rs`

```rs
#![no_std]

use core::panic::PanicInfo;

/// 当 panic 发生时会调用该函数
/// 我们暂时将它的实现为一个死循环
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

```

- 运行：`rustup target add riscv64imac-unknown-none-elf`
- 运行：`cargo build --target riscv64imac-unknown-none-elf`

结果：

```
yunwei@ubuntu:~/Project/os$ cargo build --target riscv64imac-unknown-none-elf
   Compiling os v0.1.0 (/home/yunwei/Project/os)
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s

```

在 os/.cargo/config 里面添加了这个:

```
# 编译的目标平台
[build]
target = "riscv64imac-unknown-none-elf"
```

就可以 cargo build 啦

### 生成内核镜像

运行：

```
cargo install cargo-binutils
rustup component add llvm-tools-preview

```

- `rust-objdump --version`

```
@ubuntu:~/Project/os$ rust-objdump --version
LLVM (http://llvm.org/):
  LLVM version 10.0.1-rust-1.46.0-nightly
  Optimized build.
  Default target: x86_64-unknown-linux-gnu
  Host CPU: skylake

  Registered Targets:
    aarch64    - AArch64 (little endian)
    aarch64_32 - AArch64 (little endian ILP32)
    aarch64_be - AArch64 (big endian)
    arm        - ARM
    arm64      - ARM64 (little endian)
    arm64_32   - ARM64 (little endian ILP32)
    armeb      - ARM (big endian)
    avr        - Atmel AVR Microcontroller
    hexagon    - Hexagon
    mips       - MIPS (32-bit big endian)
    mips64     - MIPS (64-bit big endian)
    mips64el   - MIPS (64-bit little endian)
    mipsel     - MIPS (32-bit little endian)
    msp430     - MSP430 [experimental]
    nvptx      - NVIDIA PTX 32-bit
    nvptx64    - NVIDIA PTX 64-bit
    ppc32      - PowerPC 32
    ppc64      - PowerPC 64
    ppc64le    - PowerPC 64 LE
    riscv32    - 32-bit RISC-V
    riscv64    - 64-bit RISC-V
    sparc      - Sparc
    sparcel    - Sparc LE
    sparcv9    - Sparc V9
    systemz    - SystemZ
    thumb      - Thumb
    thumbeb    - Thumb (big endian)
    wasm32     - WebAssembly 32-bit
    wasm64     - WebAssembly 64-bit
    x86        - 32-bit X86: Pentium-Pro and above
    x86-64     - 64-bit X86: EM64T and AMD64

```

运行：

```
target/riscv64imac-unknown-none-elf/debug/os: ELF 64-bit LSB executable, UCB RISC-V, version 1 (SYSV), statically linked, with debug_info, not stripped

```

运行：(和教程里面并不完全一样)

```
yunwei@ubuntu:~/Project/os$ rust-objdump target/riscv64imac-unknown-none-elf/debug/os -x --arch-name=riscv64

target/riscv64imac-unknown-none-elf/debug/os:	file format ELF64-riscv

architecture: riscv64
start address: 0x0000000000011120

Program Header:
    PHDR off    0x0000000000000040 vaddr 0x0000000000010040 paddr 0x0000000000010040 align 2**3
         filesz 0x00000000000000e0 memsz 0x00000000000000e0 flags r--
    LOAD off    0x0000000000000000 vaddr 0x0000000000010000 paddr 0x0000000000010000 align 2**12
         filesz 0x0000000000000120 memsz 0x0000000000000120 flags r--
    LOAD off    0x0000000000000120 vaddr 0x0000000000011120 paddr 0x0000000000011120 align 2**12
         filesz 0x0000000000000004 memsz 0x0000000000000004 flags r-x
   STACK off    0x0000000000000000 vaddr 0x0000000000000000 paddr 0x0000000000000000 align 2**64
         filesz 0x0000000000000000 memsz 0x0000000000000000 flags rw-

Dynamic Section:
Sections:
Idx Name            Size     VMA              Type
  0                 00000000 0000000000000000 
  1 .text           00000004 0000000000011120 TEXT
  2 .debug_str      000003fd 0000000000000000 
  3 .debug_abbrev   00000113 0000000000000000 
  4 .debug_info     0000053c 0000000000000000 
  5 .debug_aranges  00000040 0000000000000000 
  6 .debug_ranges   00000030 0000000000000000 
  7 .debug_pubnames 000000a4 0000000000000000 
  8 .debug_pubtypes 00000308 0000000000000000 
  9 .debug_frame    00000050 0000000000000000 
 10 .debug_line     0000005b 0000000000000000 
 11 .comment        00000013 0000000000000000 
 12 .symtab         00000108 0000000000000000 
 13 .shstrtab       000000a5 0000000000000000 
 14 .strtab         0000002d 0000000000000000 

SYMBOL TABLE:
0000000000000000 l    df *ABS*	00000000 3gqd1qcioyc9uzqc
0000000000011120         .text	00000000 
0000000000011120         .text	00000000 
0000000000011120         .text	00000000 
0000000000011124         .text	00000000 
0000000000000000         .debug_info	00000000 
0000000000000000         .debug_ranges	00000000 
0000000000000000         .debug_frame	00000000 
0000000000000000         .debug_line	00000000 .Lline_table_start0
0000000000011120 g     F .text	00000004 _start

```

反汇编：

```
yunwei@ubuntu:~/Project/os$ rust-objdump target/riscv64imac-unknown-none-elf/debug/os -d --arch-name=riscv64

target/riscv64imac-unknown-none-elf/debug/os:	file format ELF64-riscv


Disassembly of section .text:

0000000000011120 _start:
   11120: 09 a0                        	j	2
   11122: 01 a0                        	j	0

```

生成镜像：

`rust-objcopy target/riscv64imac-unknown-none-elf/debug/os --strip-all -O binary target/riscv64imac-unknown-none-elf/debug/kernel.bin`

### 调整内存布局

可以参考：[https://sourceware.org/binutils/docs/ld/Scripts.html](https://sourceware.org/binutils/docs/ld/Scripts.html) 但还没去看

编写链接脚本：

```ld
/* 有关 Linker Script 可以参考：https://sourceware.org/binutils/docs/ld/Scripts.html */

/* 目标架构 */
OUTPUT_ARCH(riscv)

/* 执行入口 */
ENTRY(_start)

/* 数据存放起始地址 */
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    /* . 表示当前地址（location counter） */
    . = BASE_ADDRESS;

    /* start 符号表示全部的开始位置 */
    kernel_start = .;

    text_start = .;

    /* .text 字段 */
    .text : {
        /* 把 entry 函数放在最前面 */
        *(.text.entry)
        /* 要链接的文件的 .text 字段集中放在这里 */
        *(.text .text.*)
    }

    rodata_start = .;

    /* .rodata 字段 */
    .rodata : {
        /* 要链接的文件的 .rodata 字段集中放在这里 */
        *(.rodata .rodata.*)
    }

    data_start = .;

    /* .data 字段 */
    .data : {
        /* 要链接的文件的 .data 字段集中放在这里 */
        *(.data .data.*)
    }

    bss_start = .;

    /* .bss 字段 */
    .bss : {
        /* 要链接的文件的 .bss 字段集中放在这里 */
        *(.sbss .bss .bss.*)
    }

    /* 结束地址 */
    kernel_end = .;
}
```

在`os/.cargo/config`里面添加：

```
# 使用我们的 linker script 来进行链接
[target.riscv64imac-unknown-none-elf]
rustflags = [
    "-C", "link-arg=-Tsrc/linker.ld",
]

```

- `cargo build`
- `rust-objdump target/riscv64imac-unknown-none-elf/debug/os -h --arch-name=riscv64`

输出：

```
Idx Name            Size     VMA              Type
  0                 00000000 0000000000000000 
  1 .text           00000004 0000000080200000 TEXT
  2 .debug_str      000003fd 0000000000000000 
  3 .debug_abbrev   00000113 0000000000000000 
  4 .debug_info     0000053c 0000000000000000 
  5 .debug_aranges  00000040 0000000000000000 
  6 .debug_ranges   00000030 0000000000000000 
  7 .debug_pubnames 000000a4 0000000000000000 
  8 .debug_pubtypes 00000308 0000000000000000 
  9 .debug_frame    00000050 0000000000000000 
 10 .debug_line     0000005b 0000000000000000 
 11 .comment        00000013 0000000000000000 
 12 .symtab         000001b0 0000000000000000 
 13 .shstrtab       000000a5 0000000000000000 
 14 .strtab         0000007f 0000000000000000 

```

- `rust-objdump target/riscv64imac-unknown-none-elf/debug/os -d --arch-name=riscv64`

```
yunwei@ubuntu:~/Project/os$ rust-objdump target/riscv64imac-unknown-none-elf/debug/os -d --arch-name=riscv64

target/riscv64imac-unknown-none-elf/debug/os:	file format ELF64-riscv


Disassembly of section .text:

0000000080200000 text_start:
80200000: 09 a0                        	j	2
80200002: 01 a0                        	j	0

```

似乎并没有把错误处理函数编译出来；

### 重写程序入口点 _start

-  `OpenSBI` 固件（Firmware）
-  OpenSBI 所做的一件事情就是把 CPU 从 M Mode 切换到 S Mode，接着跳转到一个固定地址 0x80200000，开始执行内核代码。

### 使用 QEMU 运行内核

把 `main.rs` 换成：

```
//! # 全局属性
//! - `#![no_std]`  
//!   禁用标准库
#![no_std]
//!
//! - `#![no_main]`  
//!   不使用 `main` 函数等全部 Rust-level 入口点来作为程序入口
#![no_main]
//! # 一些 unstable 的功能需要在 crate 层级声明后才可以使用
//! - `#![feature(asm)]`  
//!   内嵌汇编
#![feature(asm)]
//!
//! - `#![feature(global_asm)]`
//!   内嵌整个汇编文件
#![feature(global_asm)]

// 汇编编写的程序入口，具体见该文件
global_asm!(include_str!("entry.asm"));

use core::panic::PanicInfo;

/// 当 panic 发生时会调用该函数
/// 我们暂时将它的实现为一个死循环
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// 在屏幕上输出一个字符，目前我们先不用了解其实现原理
pub fn console_putchar(ch: u8) {
    let _ret: usize;
    let arg0: usize = ch as usize;
    let arg1: usize = 0;
    let arg2: usize = 0;
    let which: usize = 1;
    unsafe {
        llvm_asm!("ecall"
             : "={x10}" (_ret)
             : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
             : "memory"
             : "volatile"
        );
    }
}

/// Rust 的入口函数
///
/// 在 `_start` 为我们进行了一系列准备之后，这是第一个被调用的 Rust 函数
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // 在屏幕上输出 "OK\n" ，随后进入死循环
    console_putchar(b'O');
    console_putchar(b'K');
    console_putchar(b'\n');

    loop {}
}

```

编译的时候会出现：

```
yunwei@ubuntu:~/Project/os$ cargo build
   Compiling os v0.1.0 (/home/yunwei/Project/os)
error[E0658]: use of unstable library feature 'llvm_asm': prefer using the new asm! syntax instead
  --> src/main.rs:38:9
   |
38 |         llvm_asm!("ecall"
   |         ^^^^^^^^
   |
   = note: see issue #70173 <https://github.com/rust-lang/rust/issues/70173> for more information
   = help: add `#![feature(llvm_asm)]` to the crate attributes to enable

error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
error: could not compile `os`.

To learn more, run the command again with --verbose.

```

根据编译器的提示，在头部添加：

`#![feature(llvm_asm)]`

编译通过；

尝试建立`makefile`：

```
TARGET      := riscv64imac-unknown-none-elf
MODE        := debug
KERNEL_FILE := target/$(TARGET)/$(MODE)/os
BIN_FILE    := target/$(TARGET)/$(MODE)/kernel.bin

OBJDUMP     := rust-objdump --arch-name=riscv64
OBJCOPY     := rust-objcopy --binary-architecture=riscv64

.PHONY: doc kernel build clean qemu run env

# 默认 build 为输出二进制文件
build: $(BIN_FILE) 

# 通过 Rust 文件中的注释生成 os 的文档
doc:
    @cargo doc --document-private-items

# 编译 kernel
kernel:
    @cargo build

# 生成 kernel 的二进制文件
$(BIN_FILE): kernel
    @$(OBJCOPY) $(KERNEL_FILE) --strip-all -O binary $@

# 查看反汇编结果
asm:
    @$(OBJDUMP) -d $(KERNEL_FILE) | less

# 清理编译出的文件
clean:
    @cargo clean

# 运行 QEMU
qemu: build
    @qemu-system-riscv64 \
            -machine virt \
            -nographic \
            -bios default \
            -device loader,file=$(BIN_FILE),addr=0x80200000

# 一键运行
run: build qemu
```

- 输入 `make run` ，报错： `Makefile:16: *** missing separator.  Stop.`

问题解决：

如果是直接复制的话，可能会把 `Makefile` 的 `tab` 键替换为空格键，而 `makefile` 的命令行，开头必须用 `tab` 键；比如把

```
doc:
    @cargo doc --document-private-items
```

@前面的空格替换为 tab 即可解决问题。

输出：

```
OpenSBI v0.5 (Oct  9 2019 12:03:04)
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
Firmware Size          : 116 KB
Runtime SBI Version    : 0.2

PMP0: 0x0000000080000000-0x000000008001ffff (A)
PMP1: 0x0000000000000000-0xffffffffffffffff (A,R,W,X)
OK

```

看起来是OK的；

执行 `kill`, 退出 qemu

### 接口封装和代码整理

可以考虑去看看 openSBI的文档：[https://github.com/riscv/riscv-sbi-doc/blob/master/riscv-sbi.adoc#legacy-sbi-extension-extension-ids-0x00-through-0x0f](https://github.com/riscv/riscv-sbi-doc/blob/master/riscv-sbi.adoc#legacy-sbi-extension-extension-ids-0x00-through-0x0f)

新建`os/src/sbi.rs`文件：

```rs
//! 调用 Machine 层的操作
// 目前还不会用到全部的 SBI 调用，暂时允许未使用的变量或函数
#![allow(unused)]

/// SBI 调用
#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
            : "memory"      // 如果汇编可能改变内存，则需要加入 memory 选项
            : "volatile");  // 防止编译器做激进的优化（如调换指令顺序等破坏 SBI 调用行为的优化）
    }
    ret
}

const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_SHUTDOWN: usize = 8;

/// 向控制台输出一个字符
///
/// 需要注意我们不能直接使用 Rust 中的 char 类型
pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

/// 从控制台中读取一个字符
///
/// 没有读取到字符则返回 -1
pub fn console_getchar() -> usize {
    sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0)
}

/// 调用 SBI_SHUTDOWN 来关闭操作系统（直接退出 QEMU）
pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    unreachable!()
}

```

新建`os/src/console.rs`文件：

```rs
//! 实现控制台的字符输入和输出
//! 
//! # 格式化输出
//! 
//! [`core::fmt::Write`] trait 包含
//! - 需要实现的 [`write_str`] 方法
//! - 自带实现，但依赖于 [`write_str`] 的 [`write_fmt`] 方法
//! 
//! 我们声明一个类型，为其实现 [`write_str`] 方法后，就可以使用 [`write_fmt`] 来进行格式化输出
//! 
//! [`write_str`]: core::fmt::Write::write_str
//! [`write_fmt`]: core::fmt::Write::write_fmt

use crate::sbi::*;
use core::fmt::{self, Write};

/// 一个 [Zero-Sized Type]，实现 [`core::fmt::Write`] trait 来进行格式化输出
/// 
/// ZST 只可能有一个值（即为空），因此它本身就是一个单件
struct Stdout;

impl Write for Stdout {
    /// 打印一个字符串
    /// 
    /// 对于每一个字符调用 [`console_putchar`]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

/// 打印由 [`core::format_args!`] 格式化后的数据
/// 
/// [`print!`] 和 [`println!`] 宏都将展开成此函数
/// 
/// [`core::format_args!`]: https://doc.rust-lang.org/nightly/core/macro.format_args.html
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

/// 实现类似于标准库中的 `print!` 宏
/// 
/// 使用实现了 [`core::fmt::Write`] trait 的 [`console::Stdout`]
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

/// 实现类似于标准库中的 `println!` 宏
/// 
/// 使用实现了 [`core::fmt::Write`] trait 的 [`console::Stdout`]
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

```

新建`os/src/panic.rs`文件：

```rs
//! 代替 std 库，实现 panic 和 abort 的功能

use core::panic::PanicInfo;
use crate::sbi::shutdown;

/// 打印 panic 的信息并 [`shutdown`]
///
/// ### `#[panic_handler]` 属性
/// 声明此函数是 panic 的回调
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    // `\x1b[??m` 是控制终端字符输出格式的指令，在支持的平台上可以改变文字颜色等等
    // 这里使用错误红
    // 需要全局开启 feature(panic_info_message) 才可以调用 .message() 函数
    // 参考：https://misc.flogisoft.com/bash/tip_colors_and_formatting
    println!("\x1b[1;31mpanic: '{}'\x1b[0m", info.message().unwrap());
    shutdown()
}

/// 终止程序
/// 
/// 调用 [`panic_handler`]
#[no_mangle]
extern "C" fn abort() -> ! {
    panic!("abort()")
}

```

新建`os/src/main.rs`文件：

```rs
//! # 全局属性
//! - `#![no_std]`  
//!   禁用标准库
#![no_std]
//!
//! - `#![no_main]`  
//!   不使用 `main` 函数等全部 Rust-level 入口点来作为程序入口
#![no_main]
//!
//! - `#![deny(missing_docs)]`  
//!   任何没有注释的地方都会产生警告：这个属性用来压榨写实验指导的学长，同学可以删掉了
#![warn(missing_docs)]
//! # 一些 unstable 的功能需要在 crate 层级声明后才可以使用
//! - `#![feature(asm)]`  
//!   内嵌汇编
#![feature(asm)]
//!
//! - `#![feature(global_asm)]`  
//!   内嵌整个汇编文件
#![feature(global_asm)]
//!
//! - `#![feature(llvm_asm)]`
//!   声明需要使用 llvm_asm 宏特性
#![feature(llvm_asm)]
//!
//! - `#![feature(panic_info_message)]`  
//!   panic! 时，获取其中的信息并打印
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod panic;
mod sbi;

// 汇编编写的程序入口，具体见该文件
global_asm!(include_str!("entry.asm"));

/// Rust 的入口函数
///
/// 在 `_start` 为我们进行了一系列准备之后，这是第一个被调用的 Rust 函数
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    println!("Hello rCore-Tutorial!");
    panic!("end of rust_main")
}

```

运行：`make run`:

```
OpenSBI v0.5 (Oct  9 2019 12:03:04)
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
Firmware Size          : 116 KB
Runtime SBI Version    : 0.2

PMP0: 0x0000000080000000-0x000000008001ffff (A)
PMP1: 0x0000000000000000-0xffffffffffffffff (A,R,W,X)
Hello rCore-Tutorial!
panic: 'end of rust_main'

```

看起来是好啦！

# lab1 学习报告

- RISC-V 中有关中断处理的寄存器和相关流程
- 如何保存上下文，使得中断处理流程前后，原本正在执行的程序感知不到发生了中断
- 处理最简单的断点中断和时钟中断

## 中断的概念：

- 异常：执行指令时产生的，通常无法预料的错误。
- 陷阱：一系列强行导致中断的指令
- 硬件中断：由 CPU 之外的硬件产生的异步中断

## RISC-V 与中断相关的寄存器和指令

Machine mode：

- 是 RISC-V 中的最高权限模式，一些底层操作的指令只能由机器态进行使用。
- 是所有标准 RISC-V 处理器都必须实现的模式。
- 默认所有中断实际上是交给机器态处理的，但是为了实现更多功能，机器态会将某些中断交由内核态处理。这些异常也正是我们编写操作系统所需要实现的。

Supervisor mode：

- 通常为操作系统使用，可以访问一些 supervisor 级别的寄存器，通过这些寄存器对中断和虚拟内存映射进行管理。
- Unix 系统中，大部分的中断都是内核态的系统调用。机器态可以通过异常委托机制（machine interrupt delegation）将一部分中断设置为不经过机器态，直接由内核态处理

关注：内核态可以使用的一些特权指令和寄存器

### 与中断相关的寄存器

自动填写：

- sepc：用来记录触发中断的指令的地址。
- scause：记录中断是否是硬件中断，以及具体的中断原因。
- stval：需要访问但是不在内存中的地址，包含信息

指导硬件处理中断：

- stvec：设置内核态中断处理流程的入口地址。分为基址 BASE 和模式 MODE：
  - MODE 为 0 表示 Direct 模式：跳转至 BASE 进行执行
  - MODE 为 1 表示 Vectored 模式：BASE + 4 * cause
- sstatus：控制全局中断使能等
- sie：控制具体类型中断的使能
- sip：记录每种中断是否被触发
- sscratch：sscratch 在用户态保存内核栈的地址

### 与中断相关的指令

- ecall：触发中断
- sret：从内核态返回用户态，同时将 pc 的值设置为 sepc
- ebreak：触发一个断点
- mret：从机器态返回内核态
- csrrw dst, csr, src：CSR Read Write
- csrr dst, csr：CSR Write
- csrc(i) csr, rs1：CSR Clear
- csrs(i) csr, rs1：CSR Set
  
## 程序运行状态

在处理中断之前，必须要保存所有可能被修改的寄存器，并且在处理完成后恢复。我们需要保存所有通用寄存器，sepc、scause 和 stval 这三个会被硬件自动写入的 CSR 寄存器，以及 sstatus。

在 os/src/interrupt/context.rs 中添加：

```rs
use riscv::register::{sstatus::Sstatus, scause::Scause};

#[repr(C)]
pub struct Context {
    pub x: [usize; 32],     // 32 个通用寄存器
    pub sstatus: Sstatus,
    pub sepc: usize
}

```

在 os/Cargo.toml 中添加依赖:

```
[dependencies]
riscv = { git = "https://github.com/rcore-os/riscv", features = ["inline-asm"] }

```

## 状态的保存与恢复

- 保存：先用栈上的一小段空间来把需要保存的全部通用寄存器和 CSR 寄存器保存在栈上，保存完之后在跳转到 Rust 编写的中断处理函数；
- 恢复：直接把备份在栈上的内容写回寄存器。由于涉及到了寄存器级别的操作，我们需要用汇编来实现。

新建：

os/src/interrupt.asm

```asm
# 宏：将寄存器存到栈上
.macro SAVE reg, offset
    sd  \reg, \offset*8(sp)
.endm

# 宏：将寄存器从栈中取出
.macro LOAD reg, offset
    ld  \reg, \offset*8(sp)
.endm

    .section .text
    .globl __interrupt
# 进入中断
# 保存 Context 并且进入 rust 中的中断处理函数 interrupt::handler::handle_interrupt()
__interrupt:
    # 在栈上开辟 Context 所需的空间
    addi    sp, sp, -34*8
    
    # 保存通用寄存器，除了 x0（固定为 0）
    SAVE    x1, 1
    # 将原来的 sp（sp 又名 x2）写入 2 位置
    addi    x1, sp, 34*8
    SAVE    x1, 2
    # 其他通用寄存器
    SAVE    x3, 3
    SAVE    x4, 4
    SAVE    x5, 5
    SAVE    x6, 6
    SAVE    x7, 7
    SAVE    x8, 8
    SAVE    x9, 9
    SAVE    x10, 10
    SAVE    x11, 11
    SAVE    x12, 12
    SAVE    x13, 13
    SAVE    x14, 14
    SAVE    x15, 15
    SAVE    x16, 16
    SAVE    x17, 17
    SAVE    x18, 18
    SAVE    x19, 19
    SAVE    x20, 20
    SAVE    x21, 21
    SAVE    x22, 22
    SAVE    x23, 23
    SAVE    x24, 24
    SAVE    x25, 25
    SAVE    x26, 26
    SAVE    x27, 27
    SAVE    x28, 28
    SAVE    x29, 29
    SAVE    x30, 30
    SAVE    x31, 31

    # 取出 CSR 并保存
    csrr    s1, sstatus
    csrr    s2, sepc
    SAVE    s1, 32
    SAVE    s2, 33

    # 调用 handle_interrupt，传入参数
    # context: &mut Context
    mv      a0, sp
    # scause: Scause
    csrr    a1, scause
    # stval: usize
    csrr    a2, stval
    jal  handle_interrupt

    .globl __restore
# 离开中断
# 从 Context 中恢复所有寄存器，并跳转至 Context 中 sepc 的位置
__restore:
    # 恢复 CSR
    LOAD    s1, 32
    LOAD    s2, 33
    csrw    sstatus, s1
    csrw    sepc, s2

    # 恢复通用寄存器
    LOAD    x1, 1
    LOAD    x3, 3
    LOAD    x4, 4
    LOAD    x5, 5
    LOAD    x6, 6
    LOAD    x7, 7
    LOAD    x8, 8
    LOAD    x9, 9
    LOAD    x10, 10
    LOAD    x11, 11
    LOAD    x12, 12
    LOAD    x13, 13
    LOAD    x14, 14
    LOAD    x15, 15
    LOAD    x16, 16
    LOAD    x17, 17
    LOAD    x18, 18
    LOAD    x19, 19
    LOAD    x20, 20
    LOAD    x21, 21
    LOAD    x22, 22
    LOAD    x23, 23
    LOAD    x24, 24
    LOAD    x25, 25
    LOAD    x26, 26
    LOAD    x27, 27
    LOAD    x28, 28
    LOAD    x29, 29
    LOAD    x30, 30
    LOAD    x31, 31

    # 恢复 sp（又名 x2）这里最后恢复是为了上面可以正常使用 LOAD 宏
    LOAD    x2, 2
    sret

```

进入中断处理流程:

新建 os/src/interrupt/handler.rs

```rs
use super::context::Context;
use riscv::register::stvec;

global_asm!(include_str!("./interrupt.asm"));

/// 初始化中断处理
///
/// 把中断入口 `__interrupt` 写入 `stvec` 中，并且开启中断使能
pub fn init() {
    unsafe {
        extern "C" {
            /// `interrupt.asm` 中的中断入口
            fn __interrupt();
        }
        // 使用 Direct 模式，将中断入口设置为 `__interrupt`
        stvec::write(__interrupt as usize, stvec::TrapMode::Direct);
    }
}

/// 中断的处理入口
/// 
/// `interrupt.asm` 首先保存寄存器至 Context，其作为参数和 scause 以及 stval 一并传入此函数
/// 具体的中断类型需要根据 scause 来推断，然后分别处理
#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) {
    panic!("Interrupted: {:?}", scause.cause());
}
```

os/src/interrupt/mod.rs：

```rs
//! 中断模块
//! 
//! 

mod handler;
mod context;

/// 初始化中断相关的子模块
/// 
/// - [`handler::init`]
/// - [`timer::init`]
pub fn init() {
    handler::init();
    println!("mod interrupt initialized");
}
```

编译：`cargo build`

报错:
`error[E0412]: cannot find type `Scause` in this scope`

在 handler.rs 中添加：

`use riscv::register::scause::Scause;`

运行：

```
PMP0: 0x0000000080000000-0x000000008001ffff (A)
PMP1: 0x0000000000000000-0xffffffffffffffff (A,R,W,X)
Hello rCore-Tutorial!
mod interrupt initialized
panic: 'Interrupted: Exception(Breakpoint)'

```

## 时钟中断

RISC-V 中将中断分为三种：

- 软件中断（Software Interrupt）
- 时钟中断（Timer Interrupt）
- 外部中断（External Interrupt）

新建：os/src/interrupt/timer.rs

```rs
//! 预约和处理时钟中断

use crate::sbi::set_timer;
use riscv::register::{time, sie, sstatus};

/// 初始化时钟中断
/// 
/// 开启时钟中断使能，并且预约第一次时钟中断
pub fn init() {
    unsafe {
        // 开启 STIE，允许时钟中断
        sie::set_stimer(); 
        // 开启 SIE（不是 sie 寄存器），允许内核态被中断打断
        sstatus::set_sie();
    }
    // 设置下一次时钟中断
    set_next_timeout();
}

```

设置时钟中断:

os/src/sbi.rs

```rs
/// 时钟中断的间隔，单位是 CPU 指令
static INTERVAL: usize = 100000;

/// 设置下一次时钟中断
/// 
/// 获取当前时间，加上中断间隔，通过 SBI 调用预约下一次中断
fn set_next_timeout() {
    set_timer(time::read() + INTERVAL);
}

/// 触发时钟中断计数
pub static mut TICKS: usize = 0;

/// 每一次时钟中断时调用
/// 
/// 设置下一次时钟中断，同时计数 +1
pub fn tick() {
    set_next_timeout();
    unsafe {
        TICKS += 1;
        if TICKS % 100 == 0 {
            println!("{} tick", TICKS);
        }
    }
}

```

### 实现时钟中断的处理流程

os/src/interrupt/handler.rs:

```rs
/// 中断的处理入口
/// 
/// `interrupt.asm` 首先保存寄存器至 Context，其作为参数和 scause 以及 stval 一并传入此函数
/// 具体的中断类型需要根据 scause 来推断，然后分别处理
#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) {
    match scause.cause() {
        // 断点中断（ebreak）
        Trap::Exception(Exception::Breakpoint) => breakpoint(context),
        // 时钟中断
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
        // 其他情况，终止当前线程
        _ => fault(context, scause, stval),
    }
}

/// 处理 ebreak 断点
/// 
/// 继续执行，其中 `sepc` 增加 2 字节，以跳过当前这条 `ebreak` 指令
fn breakpoint(context: &mut Context) {
    println!("Breakpoint at 0x{:x}", context.sepc);
    context.sepc += 2;
}

/// 处理时钟中断
/// 
/// 目前只会在 [`timer`] 模块中进行计数
fn supervisor_timer(_: &Context) {
    timer::tick();
}

/// 出现未能解决的异常
fn fault(context: &mut Context, scause: Scause, stval: usize) {
    panic!(
        "Unresolved interrupt: {:?}\n{:x?}\nstval: {:x}",
        scause.cause(),
        context,
        stval
    );
}

```

还需要在头部加上：

```rs
use super::timer;
use riscv::register::{
    scause::{Exception, Interrupt, Scause, Trap},
    sie, stvec,
};

```

最后：

- 给 Context 结构体加上 Debug trait；
- 在 os/interrupt/mod.rs 中引入 mod timer 并在 初始化 handler::init() 语句的后面加入 timer::init()；
- 在 main.rs 中间引入 loop{}; 循环

就能跑起来啦！

# 实验一：中断

关于实验题：

1. 简述：在 rust_main 函数中，执行 ebreak 命令后至函数结束前，sp 寄存器的值是怎样变化的？
2. 回答：lab1 分支中的 rust_main 函数存在什么问题？它会导致什么结果？
3. 实验 
   1. 如果程序访问不存在的地址，会得到 Exception::LoadFault。模仿捕获 ebreak 和时钟中断的方法，捕获 LoadFault（之后 panic 即可）。
   2. 在处理异常的过程中，如果程序想要非法访问的地址是 `0x0`，则打印 `SUCCESS!`。
   3. 添加或修改少量代码，使得运行时触发这个异常，并且打印出 `SUCCESS!`。

answer：

（由于practice里面已经有答案了，这里比较简略）

1. sp -34*8; 在中断处理完成后恢复；中间不变
2. 不结束
3. 实验：

handler.rs

```rs
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) {
    context.sepc = 0;
    match scause.cause() {
        // 断点中断（ebreak）
        Trap::Exception(Exception::Breakpoint) => breakpoint(context),
        Trap::Exception(Exception::LoadFault) => loadfault(context,stval),
        // 时钟中断
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
        // 其他情况，终止当前线程
        _ => fault(context, scause, stval),
    }
}

...

/// load
/// 
fn loadfault(context: &mut Context, stval: usize){
    if stval == 0 {
        println!("SUCCESS!");
    }
    panic!(
        "loadfault interrupt: {:x?}\nstval: {:x}",
        context,
        stval
    );
}

```

解法1，解法2. 已经有说明；

这里说一个解法三：

在 handle_interrupt 函数开头或结尾结尾加上：

`context.sepc = 0`

估计由于qemu底下是按四字节对齐访问触发异常的，因此breakpoint 那个 +2 可以忽略不计，只有 4 的倍数才会起作用；只要在错误流程处理的任何一个地方加上这句话或者类似的都能起到效果。

# lab2 学习报告

这部分的代码从上一章 lab1 的代码开始完成：

这一章的实验指导中:

- 实现动态内存的分配
- 了解 `QEMU` 虚拟的物理内存机制
- 通过页的方式对物理内存进行管理

lab2 的代码主要在 memory 文件夹中；算法部分在 algorithm 文件夹中

## 动态内存分配

我们的内核中也需要动态内存分配，典型的应用场景有：

- `Box<T>` ，类似 `malloc` 有；
- 引用计数 `Rc<T>`，原子引用计数 `Arc<T>`，主要用于在引用计数清零，即某对象不再被引用时，对该对象进行自动回收；
- 一些 std 中的数据结构，如 Vec 和 HashMap 等。

需要实现 `Trait GlobalAlloc` :

将这个类实例化，并使用语义项 #[global_allocator] 进行标记。

为了实现 Trait GlobalAlloc，需要支持这两个函数：

```RS
unsafe fn alloc(&self, layout: Layout) -> *mut u8;
unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout);
```

例如 memory/heap2.rs ：

```rs

/// 利用 VectorAllocator 的接口实现全局分配器的 GlobalAlloc trait
unsafe impl alloc::alloc::GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let offset = (*self.0.get())
            .as_mut()
            .unwrap()
            .alloc(layout.size(), layout.align())
            .expect("Heap overflow");
        &mut HEAP_SPACE[offset] as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let offset = ptr as usize - &HEAP_SPACE as *const _ as usize;
        (*self.0.get())
            .as_mut()
            .unwrap()
            .dealloc(offset, layout.size(), layout.align());
    }
}

```

Layout:

- size 表示要分配的字节数;
- align 则表示分配的虚拟地址的最小对齐要求，即分配的地址要求是 align 的倍数。

### 支持动态内存分配的方法

连续内存分配算法: 使用 Buddy System 来实现:

这里可以试着调用 Buddy System Allocator 的轮子：

操作系统动态分配内存所用的堆大小（8M）

- #[global_allocator]：可以为全局需要用到堆的地方分配空间。
- #[alloc_error_handler]：空间分配错误的回调

### 动态内存分配测试

例如：

```rs
    // 动态内存分配测试
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    let v = Box::new(5);
    assert_eq!(*v, 5);
    core::mem::drop(v);
```

思考：

动态分配的内存地址在哪个范围里？

（上面源代码的注释中提及了在bss段中，是一个静态的没有初始化的数组，算是内核代码的一部分。

这一部分是内核的堆分配，参考 memory/heap.rs:

```rs
#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

/// 初始化操作系统运行时堆空间
pub fn init() {
    // 告诉分配器使用这一段预留的空间作为堆
    unsafe {
        HEAP.lock()
            .init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}

```

## 物理内存探测

外设和内存的访问；

操作系统怎样知道物理内存所在的那段物理地址：

在 RISC-V 中，这个一般是由 Bootloader ，即 OpenSBI 来完成的。它来完成对于包括物理内存在内的各外设的扫描，将扫描结果以 DTB（Device Tree Blob）的格式保存在物理内存中的某个地方。随后 OpenSBI 会将其地址保存在 a1 寄存器中，给我们使用。

最后一块含义为 DRAM 的地址空间，这就是 OS 将要管理的 128 MB 的内存空间：

`0x80000000 	0x88000000 	DRAM 缺省 128MB，大小可配置`

首先建立一个 PhysicalAddress 的类，然后对其实现一系列的 usize 的加、减和输出等等操作；

直接将 DRAM 物理内存结束地址硬编码到内核中：记录下操作系统用到的地址结尾（即 linker script 中的 kernel_end）。

os/src/memory/config.rs：

```rs

lazy_static! {
    /// 内核代码结束的地址，即可以用来分配的内存起始地址
    ///
    /// 因为 Rust 语言限制，我们只能将其作为一个运行时求值的 static 变量，而不能作为 const
    pub static ref KERNEL_END_ADDRESS: PhysicalAddress = PhysicalAddress(kernel_end as usize);
}

extern "C" {
    /// 由 `linker.ld` 指定的内核代码结束位置
    ///
    /// 作为变量存在 [`KERNEL_END_ADDRESS`]
    fn kernel_end();
}

```

在各级文件中加入模块调用，并在 os/src/main.rs 尝试输出：

```rs
    // 注意这里的 KERNEL_END_ADDRESS 为 ref 类型，需要加 *
    println!("{}", *memory::config::KERNEL_END_ADDRESS);
```

## 物理内存管理:

通常，我们在分配物理内存时并不是以字节为单位，而是以一物理页(Frame)，即连续的 4 KB 字节为单位分配。我们希望用物理页号（Physical Page Number，PPN）来代表一物理页.

>物理页号与物理页形成一一映射

参考 os/src/memory/address.rs

在 os/src/memory/config.rs 中加入相关的设置：

```rs

/// 页 / 帧大小，必须是 2^n
pub const PAGE_SIZE: usize = 4096;

/// 可以访问的内存区域起始地址
pub const MEMORY_START_ADDRESS: PhysicalAddress = PhysicalAddress(0x8000_0000);
/// 可以访问的内存区域结束地址
pub const MEMORY_END_ADDRESS: PhysicalAddress = PhysicalAddress(0x8800_0000);

```

### 分配和回收:

为了方便管理所有的物理页，我们需要实现一个分配器可以进行分配和回收的操作：把区域的真实物理地址封装到了一个 FrameTracker 里面

- 分配器分配给我们 FrameTracker 作为一个帧的标识
- 我们利用 Rust 的 drop 机制在析构的时候自动实现回收。

封装一个物理页分配器：这个分配器将不涉及任何的具体算法。

具体的算法将用一个名为 Allocator 的 Rust trait 封装起来，而我们的 FrameAllocator 会依赖于具体的 trait 实现例化。


os/src/memory/frame/allocator.rs:

```rs
lazy_static! {
    /// 帧分配器
    pub static ref FRAME_ALLOCATOR: Mutex<FrameAllocator<AllocatorImpl>> = Mutex::new(FrameAllocator::new(Range::from(
            PhysicalPageNumber::ceil(PhysicalAddress::from(*KERNEL_END_ADDRESS))..PhysicalPageNumber::floor(MEMORY_END_ADDRESS),
        )
    ));
}

/// 帧分配 / 回收（取决于你用什么算法
pub struct FrameAllocator<T: Allocator> {
    /// 可用区间的起始
    start_ppn: PhysicalPageNumber,
    /// 分配器
    allocator: T,
}

impl<T: Allocator> FrameAllocator<T> {
    /// 创建对象
    pub fn new(range: impl Into<Range<PhysicalPageNumber>> + Copy) -> Self {
        FrameAllocator {
            start_ppn: range.into().start,
            allocator: T::new(range.into().len()),
        }
    }

    /// 分配帧，如果没有剩余则返回 `Err`
    pub fn alloc(&mut self) -> MemoryResult<FrameTracker> {
        self.allocator
            .alloc()
            .ok_or("no available frame to allocate")
            .map(|offset| FrameTracker(self.start_ppn + offset))
    }

    /// 将被释放的帧添加到空闲列表的尾部
    ///
    /// 这个函数会在 [`FrameTracker`] 被 drop 时自动调用，不应在其他地方调用
    pub(super) fn dealloc(&mut self, frame: &FrameTracker) {
        self.allocator.dealloc(frame.page_number() - self.start_ppn);
    }
}
```

有关具体的算法，我们封装了一个分配器需要的 Rust trait：

```rs
/// 分配器：固定容量，每次分配 / 回收一个元素
pub trait Allocator {
    /// 给定容量，创建分配器
    fn new(capacity: usize) -> Self;
    /// 分配一个元素，无法分配则返回 `None`
    fn alloc(&mut self) -> Option<usize>;
    /// 回收一个元素
    fn dealloc(&mut self, index: usize);
}
```

这一部分可以使用线段树，也可以不用；但似乎分配单个元素的时候线段树并没有多大意义；

使用 spin::Mutex<T> 对于 static mut 类型加锁以避免冲突；

把新写的模块加载进来，并在 main 函数中进行简单的测试：

```rs
    // 物理页分配
    for _ in 0..2 {
        let frame_0 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        let frame_1 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        println!("{} and {}", frame_0.address(), frame_1.address());
    }

```

思考，和上面的代码有何不同，我们的设计是否存在一些语法上的设计缺陷？

- 这样写会导致死锁。

# 实验二：内存分配

1. 回答：如果我们在实现这个堆的过程中使用 Vec 而不是 [u8]，会出现什么结果？

- 无限循环动态分配？（我觉得可以试试；

2. 回答：algorithm/src/allocator 下有一个 Allocator trait，我们之前用它实现了物理页面分配。这个算法的时间和空间复杂度是什么？

3.  实现基于线段树的物理页面分配算法：

（正在查找资料尝试完成；

在分配单个页面的时候线段树的意义也许并不是很大，因此代码内部这里还是采用了栈进行分配；

- 栈时间复杂度可以达到 O(1) 空间复杂度 O(N)
- 线段树在分配单个页面的时候时间复杂度 O(N)，空间复杂度 O(N) 而且常数更大；虽然最坏情况下的空间复杂度更好；
- 这里的线段树使用方式可以借助二分查找来类比

因此只有在分配多个连续页面的时候才是有意义的，而堆栈并不适用于这种场景；

如果可以做出改进的话，应当可以考虑分配多个页面；

这里先尝试实现了一个简单的分配单个页面的线段树算法（还可以优化；没有使用bitmap）；

进行了一下简单的计时对比：分别进行 10000 次分配和删除；

```rs
    let t = time::read();

    // 物理页分配
    let mut a = Vec::<crate::memory::frame::FrameTracker>::new();
    for _ in 0..10000 {
        let frame_0 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err),
        };
        let frame_1 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err),
        };
        //println!("{} and {}", frame_0.address(), frame_1.address());
        a.push(frame_0);
        a.push(frame_1);
        a.pop();
    }
    println!("time: {}",time::read() - t);
```

- SegmentTreeAllocator：time: 6322806
- StackedAllocator：time: 829291

同时也增加了部分测试代码：

1. 挑战实验（选做）

（正在查找资料尝试完成；

- 在 algorithm crate 中利用伙伴算法实现 VectorAllocator trait；
- 让堆本身也能够利用 Vec 来动态分配空间；

# lab3 学习报告

lab3 和 lab2 联系紧密，是其后续部分，在 lab2 中涉及通过页的方式对物理内存进行管理：

在 lab3 中主要涉及：

- 虚拟地址和物理地址的概念和关系
- 利用页表完成虚拟地址到物理地址的映射
- 实现内核的重映射 

这一部分的代码将会在 lab2 的实验结果上面继续添加；

## 从虚拟内存到物理内存

### 原理

在现代的操作系统中，为了让其他的程序能方便的运行在操作系统上，需要完成的一个很重要的抽象是「每个程序有自己的地址空间，且地址空间范围是一样的」，这将会减少了上层程序的大量麻烦，否则程序本身要维护自己需要的物理内存，这也会导致极大程度的不安全。

这个执行上看到的地址空间，就是虚拟内存。而访问虚拟内存的地址就是`虚拟地址（Virtual Address）`，与之对应的是`物理地址（Physical Address）`。

这样的设计会导致上层的应用程序可能会访问同一个值相等的虚拟地址，所以操作系统需要做的就是替这些程序维护这个虚拟地址到物理地址的映射。甚者，为了统一和连贯，内核自己本身访问内存也将会通过虚拟地址。

### Sv39

选择 RISC-V 本身硬件支持的 Sv39 模式作为页表的实现：

- 物理地址有 56 位
- 虚拟地址有 64 位，只有低 39 位有效。 63-39 位的值必须等于第 38 位的值。
- 物理页号为 44 位，每个物理页大小为 4KB
- 虚拟页号为 27 位，每个虚拟页大小也为 4KB
- 物理地址和虚拟地址的最后 12 位都表示页内偏移

### 页表项

`页表项（PTE，Page Table Entry）`是用来描述一个虚拟页号如何映射到物理页号的：

Sv39 模式里面的一个页表项大小为 64 位（即 8 字节）。其中第 53-10 共 44 位为一个物理页号，表示这个虚拟页号映射到的物理页号。后面的第 9-0 位则描述页的相关状态信息。

- V 表示这个页表项是否合法。如果为 0 表示不合法，此时页表项其他位的值都会被忽略。
- R,W,X 分别表示是否可读（Readable）、可写（Writable）和可执行（Executable）。
- 如果 R,W,X 均为 0，文档上说这表示这个页表项指向下一级页表。
- U 为 1 表示用户态运行的程序可以通过该页表项完成地址映射。需要将 S 态的状态寄存器 sstatus 上的 SUM (permit Supervisor User Memory access) 位手动设置为 1 才可以访问通过这些 U 为 1 的页表项进行映射的用户态内存空间。

### 多级页表

在 Sv39 模式中我们采用三级页表

### 页表基址

- 页表寄存器 satp：页表的基址（起始地址）一般会保存在一个特殊的寄存器中。

### 快表（TLB）

使用`快表（TLB, Translation Lookaside Buffer）`来作为虚拟页号到物理页号的映射的缓存。

- 需要使用 sfence.vma 指令刷新 TLB

## 修改内核

首先需要把内核的运行环境从物理地址空间转移到虚拟地址空间：将内核代码放在虚拟地址空间中以 0xffffffff80200000 开头的一段高地址空间中。

这是一种临时的线性映射：

os/src/linker.ld：

需要将起始地址修改为虚拟地址，增加 4K 对齐：

```rs
/* 目标架构 */
OUTPUT_ARCH(riscv)

/* 执行入口 */
ENTRY(_start)

/* 数据存放起始地址 */
BASE_ADDRESS = 0xffffffff80200000;

SECTIONS
{   
    /* . 表示当前地址（location counter） */
    . = BASE_ADDRESS;

    /* start 符号表示全部的开始位置 */
    kernel_start = .;

    . = ALIGN(4K);
    text_start = .;

    /* .text 字段 */
    .text : {
        /* 把 entry 函数放在最前面 */
        *(.text.entry)
        /* 要链接的文件的 .text 字段集中放在这里 */
        *(.text .text.*)
    }

    . = ALIGN(4K);
    rodata_start = .;

    /* .rodata 字段 */
    .rodata : {
        /* 要链接的文件的 .rodata 字段集中放在这里 */
        *(.rodata .rodata.*)
    }

    . = ALIGN(4K);
    data_start = .;

    /* .data 字段 */
    .data : {
        /* 要链接的文件的 .data 字段集中放在这里 */
        *(.data .data.*)
    }

    . = ALIGN(4K);
    bss_start = .;

    /* .bss 字段 */
    .bss : {
        /* 要链接的文件的 .bss 字段集中放在这里 */
        *(.sbss .bss .bss.*)
    }

    /* 结束地址 */
    . = ALIGN(4K);
    kernel_end = .;
}

```

修改 os/src/memory/config.rs 中的 KERNEL_END_ADDRESS 修改为虚拟地址并加入偏移量：

```rs
lazy_static! {
    /// 内核代码结束的地址，即可以用来分配的内存起始地址
    /// 
    /// 因为 Rust 语言限制，我们只能将其作为一个运行时求值的 static 变量，而不能作为 const
    pub static ref KERNEL_END_ADDRESS: VirtualAddress = VirtualAddress(kernel_end as usize); 
}

/// 内核使用线性映射的偏移量
pub const KERNEL_MAP_OFFSET: usize = 0xffff_ffff_0000_0000;

```

需要加入两个关于位操作的辅助 crate：

os/Cargo.toml：

```rs
bitflags = "1.2.1"
bit_field = "0.10.0"

```

os/src/memory/address.rs

对虚拟地址和虚拟页号这两个类进行了封装，同时也支持了一些诸如 VirtualAddress::from(PhysicalAddress) 的转换 trait（即一些加减偏移量等操作）：

（略过）

在启动时、在进入 rust_main 之前我们要完成一个从物理地址访存模式到虚拟访存模式的转换：我们要写一个简单的页表，完成这个线性映射：

os/src/entry.asm

```rs
    ......
_start:
    # 计算 boot_page_table 的物理页号
    lui t0, %hi(boot_page_table)
    li t1, 0xffffffff00000000
    sub t0, t0, t1
    srli t0, t0, 12
    # 8 << 60 是 satp 中使用 Sv39 模式的记号
    li t1, (8 << 60)
    or t0, t0, t1
    # 写入 satp 并更新 TLB
    csrw satp, t0
    sfence.vma

    # 加载栈地址
    lui sp, %hi(boot_stack_top)
    addi sp, sp, %lo(boot_stack_top)
    # 跳转至 rust_main
    lui t0, %hi(rust_main)
    addi t0, t0, %lo(rust_main)
    jr t0
    .....

    # 初始内核映射所用的页表
    .section .data
    .align 12
boot_page_table:
    .quad 0
    .quad 0
    # 第 2 项：0x8000_0000 -> 0x8000_0000，0xcf 表示 VRWXAD 均为 1
    .quad (0x80000 << 10) | 0xcf
    .zero 507 * 8
    # 第 510 项：0xffff_ffff_8000_0000 -> 0x8000_0000，0xcf 表示 VRWXAD 均为 1
    .quad (0x80000 << 10) | 0xcf
    .quad 0
```

上面的代码完成了：

- 把 CPU 的访问模式改为 Sv39，这里需要做的就是把一个页表的物理页号和 Sv39 模式写入 satp 寄存器，然后刷新 TLB。
- 先使用一种最简单的页表构造方法：将一个三级页表项的标志位 R,W,X 不设为全 0，可以将它变为表示 1GB 的一个大页。
- 有一个从 0x80000000 到 0x80000000 的映射，在跳转到 rust_main 之前（即 jr t0）之前，PC 的值都还是 0x802xxxxx 这样的地址，即使是写入了 satp 寄存器，但是 PC 的地址不会变。为了执行这段中间的尴尬的代码，我们在页表里面也需要加入这段代码的地址的映射。（过渡使用

还需要记得修改一下 allocator.rs:

```rs
lazy_static! {
    /// 帧分配器
    pub static ref FRAME_ALLOCATOR: Mutex<FrameAllocator<AllocatorImpl>> = Mutex::new(FrameAllocator::new(Range::from(
            PhysicalPageNumber::ceil(PhysicalAddress::from(*KERNEL_END_ADDRESS))..PhysicalPageNumber::floor(MEMORY_END_ADDRESS),
        )
    ));
}

```

make clean
make run

即可运行。

## 实现页表

首先构建了通过虚拟页号获得三级 VPN 的辅助函数： 

os/src/memory/address.rs：

```rs
impl VirtualPageNumber {
    /// 得到一、二、三级页号
    pub fn levels(self) -> [usize; 3] {
        [
            self.0.get_bits(18..27),
            self.0.get_bits(9..18),
            self.0.get_bits(0..9),
        ]
    }
}
```

### 页表项

页表项，其实就是对一个 usize（8 字节）的封装，同时我们可以用刚刚加入的 bit 级别操作的 crate 对其实现一些取出特定段的方便后续实现的函数：

新建一个 mapping 文件夹：

os/src/memory/mapping/page_table_entry.rs

对于页表项的一些功能函数：

```rs
use crate::memory::address::*;
use bit_field::BitField;
use bitflags::*;

/// Sv39 结构的页表项
#[derive(Copy, Clone, Default)]
pub struct PageTableEntry(usize);

/// Sv39 页表项中标志位的位置
const FLAG_RANGE: core::ops::Range<usize> = 0..8;
/// Sv39 页表项中物理页号的位置
const PAGE_NUMBER_RANGE: core::ops::Range<usize> = 10..54;

impl PageTableEntry {
    /// 将相应页号和标志写入一个页表项
    pub fn new(page_number: PhysicalPageNumber, flags: Flags) -> Self {
        Self(
            *0usize
                .set_bits(..8, flags.bits() as usize)
                .set_bits(10..54, page_number.into()),
        )
    }
    /// 清除
    pub fn clear(&mut self) {
        self.0 = 0;
    }
    /// 设置物理页号，同时根据 ppn 是否为 Some 来设置 Valid 位
    pub fn update_page_number(&mut self, ppn: Option<PhysicalPageNumber>) {
        if let Some(ppn) = ppn {
            self.0
                .set_bits(FLAG_RANGE, (self.flags() | Flags::VALID).bits() as usize)
                .set_bits(PAGE_NUMBER_RANGE, ppn.into());
        } else {
            self.0
                .set_bits(FLAG_RANGE, (self.flags() - Flags::VALID).bits() as usize)
                .set_bits(PAGE_NUMBER_RANGE, 0);
        }
    }
    /// 获取页号
    pub fn page_number(&self) -> PhysicalPageNumber {
        PhysicalPageNumber::from(self.0.get_bits(10..54))
    }
    /// 获取地址
    pub fn address(&self) -> PhysicalAddress {
        PhysicalAddress::from(self.page_number())
    }
    /// 获取标志位
    pub fn flags(&self) -> Flags {
        unsafe { Flags::from_bits_unchecked(self.0.get_bits(..8) as u8) }
    }
    /// 是否为空（可能非空也非 Valid）
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
    /// 是否指向下一级（RWX 全为0）
    pub fn has_next_level(&self) -> bool {
        let flags = self.flags();
        !(flags.contains(Flags::READABLE)
            || flags.contains(Flags::WRITABLE)
            || flags.contains(Flags::EXECUTABLE))
    }
}

impl core::fmt::Debug for PageTableEntry {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter
            .debug_struct("PageTableEntry")
            .field("value", &self.0)
            .field("page_number", &self.page_number())
            .field("flags", &self.flags())
            .finish()
    }
}

bitflags! {
    /// 页表项中的 8 个标志位
    #[derive(Default)]
    pub struct Flags: u8 {
        /// 有效位
        const VALID =       1 << 0;
        /// 可读位
        const READABLE =    1 << 1;
        /// 可写位
        const WRITABLE =    1 << 2;
        /// 可执行位
        const EXECUTABLE =  1 << 3;
        /// 用户位
        const USER =        1 << 4;
        /// 全局位，我们不会使用
        const GLOBAL =      1 << 5;
        /// 已使用位，用于替换算法
        const ACCESSED =    1 << 6;
        /// 已修改位，用于替换算法
        const DIRTY =       1 << 7;
    }
}

macro_rules! implement_flags {
    ($field: ident, $name: ident, $quote: literal) => {
        impl Flags {
            #[doc = "返回 `Flags::"]
            #[doc = $quote]
            #[doc = "` 或 `Flags::empty()`"]
            pub fn $name(value: bool) -> Flags {
                if value {
                    Flags::$field
                } else {
                    Flags::empty()
                }
            }
        }
    };
}

implement_flags! {USER, user, "USER"}
implement_flags! {READABLE, readable, "READABLE"}
implement_flags! {WRITABLE, writable, "WRITABLE"}
implement_flags! {EXECUTABLE, executable, "EXECUTABLE"}
```

## 页表

os/src/memory/mapping/page_table.rs

```rs
//! 单一页表页面（4K） [`PageTable`]，以及相应封装 [`FrameTracker`] 的 [`PageTableTracker`]
use super::page_table_entry::PageTableEntry;
use crate::memory::{address::*, config::PAGE_SIZE, frame::FrameTracker};
/// 存有 512 个页表项的页表
#[repr(C)]
pub struct PageTable {
    pub entries: [PageTableEntry; PAGE_SIZE / 8],
}

impl PageTable {
    /// 将页表清零
    pub fn zero_init(&mut self) {
        self.entries = [Default::default(); PAGE_SIZE / 8];
    }
}

/// 类似于 [`FrameTracker`]，用于记录某一个内存中页表
pub struct PageTableTracker(pub FrameTracker);

impl PageTableTracker {
    /// 将一个分配的帧清零，形成空的页表
    pub fn new(frame: FrameTracker) -> Self {
        let mut page_table = Self(frame);
        page_table.zero_init();
        page_table
    }
    /// 获取物理页号
    pub fn page_number(&self) -> PhysicalPageNumber {
        self.0.page_number()
    }
}

// PageTableEntry 和 PageTableTracker 都可以 deref 到对应的 PageTable

impl core::ops::Deref for PageTableTracker {
    type Target = PageTable;
    fn deref(&self) -> &Self::Target {
        self.0.address().deref_kernel()
    }
}

impl core::ops::DerefMut for PageTableTracker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.address().deref_kernel()
    }
}

impl PageTableEntry {
    pub fn get_next_table(&self) -> &'static mut PageTable {
        self.address().deref_kernel()
    }
}

```

## 实现内核重映射

由于各个段之间的访问权限是不同的，因此，我们考虑对这些段分别进行重映射，使得他们的访问权限被正确设置。

这个需求可以抽象为一段内存（可能是很多个虚拟页）通过一个方式映射到很多个物理页上，同时这个内存段将会有一个统一的属性和进一步高层次的管理。

### 内存段 Segment

内存段是一篇连续的虚拟页范围，其中的每一页通过线性映射（直接偏移到一个物理页）或者分配（其中的每个虚拟页调用物理页分配器分配一个物理页）。

- 内核使用线性映射
- 用户空间使用按帧分配映射

os/src/memory/mapping/segment.rs

用 enum 和 struct 来封装内存段映射的类型和内存段本身：

后面，上层需要做的是把一个 Segment 中没有建立物理页映射关系的全部虚拟页，都申请到物理页并建立映射关系（或者说线性映射没有这样的虚拟页，而分配映射需要把每个虚拟页都申请一个对应的物理页）；因此可以实现这样一个需要具体分配的迭代器。

```rs
//! 映射类型 [`MapType`] 和映射片段 [`Segment`]

use crate::memory::{address::*, mapping::Flags, range::Range};

/// 映射的类型
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MapType {
    /// 线性映射，操作系统使用
    Linear,
    /// 按帧分配映射
    Framed,
}

/// 一个映射片段（对应旧 tutorial 的 `MemoryArea`）
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Segment {
    /// 映射类型
    pub map_type: MapType,
    /// 所映射的虚拟地址
    pub range: Range<VirtualAddress>,
    /// 权限标志
    pub flags: Flags,
}

impl Segment {
    /// 遍历对应的物理地址（如果可能）
    pub fn iter_mapped(&self) -> Option<impl Iterator<Item = PhysicalPageNumber>> {
        match self.map_type {
            // 线性映射可以直接将虚拟地址转换
            MapType::Linear => Some(self.page_range().into().iter()),
            // 按帧映射无法直接获得物理地址，需要分配
            MapType::Framed => None,
        }
    }

    /// 将地址相应地上下取整，获得虚拟页号区间
    pub fn page_range(&self) -> Range<VirtualPageNumber> {
        Range::from(
            VirtualPageNumber::floor(self.range.start)..VirtualPageNumber::ceil(self.range.end),
        )
    }
}

```

### Mapping

对页表、内存段进行组合和封装，借助其中对页表的操作实现对内存段的映射：

```rs
#[derive(Default)]
/// 某个线程的内存映射关系
pub struct Mapping {
    /// 保存所有使用到的页表
    page_tables: Vec<PageTableTracker>,
    /// 根页表的物理页号
    root_ppn: PhysicalPageNumber,
    /// 所有分配的物理页面映射信息
    mapped_pairs: VecDeque<(VirtualPageNumber, FrameTracker)>,
}

impl Mapping {
    /// 创建一个有根节点的映射
    pub fn new() -> MemoryResult<Mapping> {
        let root_table = PageTableTracker::new(FRAME_ALLOCATOR.lock().alloc()?);
        let root_ppn = root_table.page_number();
        Ok(Mapping {
            page_tables: vec![root_table],
            root_ppn,
            mapped_pairs: VecDeque::new(),
        })
    }
}
```

实现对页表的查找，并利用该函数实现：
- 对虚拟页号到物理页号的映射：
- 对一个连续的 Segment 的映射：
- 页表的激活



```rs

impl Mapping {
    /// 将当前的映射加载到 `satp` 寄存器并记录
    pub fn activate(&self) {
        // satp 低 27 位为页号，高 4 位为模式，8 表示 Sv39
        let new_satp = self.root_ppn.0 | (8 << 60);
        unsafe {
            // 将 new_satp 的值写到 satp 寄存器
            llvm_asm!("csrw satp, $0" :: "r"(new_satp) :: "volatile");
            // 刷新 TLB
            llvm_asm!("sfence.vma" :::: "volatile");
        }
    }

    /// 创建一个有根节点的映射
    pub fn new() -> MemoryResult<Mapping> {
        let root_table = PageTableTracker::new(FRAME_ALLOCATOR.lock().alloc()?);
        let root_ppn = root_table.page_number();
        Ok(Mapping {
            page_tables: vec![root_table],
            root_ppn,
            mapped_pairs: VecDeque::new(),
        })
    }

    /// 加入一段映射，可能会相应地分配物理页面
    ///
    /// 未被分配物理页面的虚拟页号暂时不会写入页表当中，它们会在发生 PageFault 后再建立页表项。
    pub fn map(&mut self, segment: &Segment, init_data: Option<&[u8]>) -> MemoryResult<()> {
        match segment.map_type {
            // 线性映射，直接对虚拟地址进行转换
            MapType::Linear => {
                for vpn in segment.page_range().iter() {
                    self.map_one(vpn, Some(vpn.into()), segment.flags | Flags::VALID)?;
                }
                // 拷贝数据
                if let Some(data) = init_data {
                    unsafe {
                        (&mut *slice_from_raw_parts_mut(segment.range.start.deref(), data.len()))
                            .copy_from_slice(data);
                    }
                }
            }
            // 需要分配帧进行映射
            MapType::Framed => {
                for vpn in segment.page_range().iter() {
                    // 如果有初始化数据，找到相应的数据
                    let page_data = if init_data.is_none() || init_data.unwrap().is_empty() {
                        [0u8; PAGE_SIZE]
                    } else {
                        // 这里必须进行一些调整，因为传入的数据可能并非按照整页对齐

                        // 传入的初始化数据
                        let init_data = init_data.unwrap();
                        // 整理后将要返回的一整个页面的数据
                        let mut page_data = [0u8; PAGE_SIZE];

                        // 拷贝时必须考虑区间与整页不对齐的情况
                        //    start（仅第一页时非零）
                        //      |        stop（仅最后一页时非零）
                        // 0    |---data---|          4096
                        // |------------page------------|
                        let page_address = VirtualAddress::from(vpn);
                        let start = if segment.range.start > page_address {
                            segment.range.start - page_address
                        } else {
                            0
                        };
                        let stop = min(PAGE_SIZE, segment.range.end - page_address);
                        // 计算来源和目标区间并进行拷贝
                        let dst_slice = &mut page_data[start..stop];
                        let src_slice = &init_data[(page_address + start - segment.range.start)
                            ..(page_address + stop - segment.range.start)];
                        dst_slice.copy_from_slice(src_slice);

                        page_data
                    };

                    // 建立映射
                    let mut frame = FRAME_ALLOCATOR.lock().alloc()?;
                    // 更新页表
                    self.map_one(vpn, Some(frame.page_number()), segment.flags)?;
                    // 写入数据
                    (*frame).copy_from_slice(&page_data);
                    // 保存
                    self.mapped_pairs.push_back((vpn, frame));
                }
            }
        }
        Ok(())
    }

    /// 移除一段映射
    pub fn unmap(&mut self, segment: &Segment) {
        for vpn in segment.page_range().iter() {
            let entry = self.find_entry(vpn).unwrap();
            assert!(!entry.is_empty());
            // 从页表中清除项
            entry.clear();
        }
    }

    /// 找到给定虚拟页号的三级页表项
    ///
    /// 如果找不到对应的页表项，则会相应创建页表
    pub fn find_entry(&mut self, vpn: VirtualPageNumber) -> MemoryResult<&mut PageTableEntry> {
        // 从根页表开始向下查询
        // 这里不用 self.page_tables[0] 避免后面产生 borrow-check 冲突（我太菜了）
        let root_table: &mut PageTable = PhysicalAddress::from(self.root_ppn).deref_kernel();
        let mut entry = &mut root_table.entries[vpn.levels()[0]];
        for vpn_slice in &vpn.levels()[1..] {
            if entry.is_empty() {
                // 如果页表不存在，则需要分配一个新的页表
                let new_table = PageTableTracker::new(FRAME_ALLOCATOR.lock().alloc()?);
                let new_ppn = new_table.page_number();
                // 将新页表的页号写入当前的页表项
                *entry = PageTableEntry::new(Some(new_ppn), Flags::VALID);
                // 保存页表
                self.page_tables.push(new_table);
            }
            // 进入下一级页表（使用偏移量来访问物理地址）
            entry = &mut entry.get_next_table().entries[*vpn_slice];
        }
        // 此时 entry 位于第三级页表
        Ok(entry)
    }

    /// 查找虚拟地址对应的物理地址
    pub fn lookup(va: VirtualAddress) -> Option<PhysicalAddress> {
        let mut current_ppn;
        unsafe {
            llvm_asm!("csrr $0, satp" : "=r"(current_ppn) ::: "volatile");
            current_ppn ^= 8 << 60;
        }

        let root_table: &PageTable =
            PhysicalAddress::from(PhysicalPageNumber(current_ppn)).deref_kernel();
        let vpn = VirtualPageNumber::floor(va);
        let mut entry = &root_table.entries[vpn.levels()[0]];
        // 为了支持大页的查找，我们用 length 表示查找到的物理页需要加多少位的偏移
        let mut length = 12 + 2 * 9;
        for vpn_slice in &vpn.levels()[1..] {
            if entry.is_empty() {
                return None;
            }
            if entry.has_next_level() {
                length -= 9;
                entry = &mut entry.get_next_table().entries[*vpn_slice];
            } else {
                break;
            }
        }
        let base = PhysicalAddress::from(entry.page_number()).0;
        let offset = va.0 & ((1 << length) - 1);
        Some(PhysicalAddress(base + offset))
    }

    /// 为给定的虚拟 / 物理页号建立映射关系
    fn map_one(
        &mut self,
        vpn: VirtualPageNumber,
        ppn: Option<PhysicalPageNumber>,
        flags: Flags,
    ) -> MemoryResult<()> {
        // 定位到页表项
        let entry = self.find_entry(vpn)?;
        assert!(entry.is_empty(), "virtual address is already mapped");
        // 页表项为空，则写入内容
        *entry = PageTableEntry::new(ppn, flags);
        Ok(())
    }
}
```

### MemorySet

我们需要把内核的每个段根据不同的属性写入上面的封装的 Mapping 中，并把它作为一个新的结构 MemorySet 给后面的线程的概念使用：

os/src/memory/mapping/memory_set.rs

```rs
//! 一个线程中关于内存空间的所有信息 [`MemorySet`]
//!

use crate::memory::{
    address::*,
    config::*,
    mapping::{Flags, MapType, Mapping, Segment},
    range::Range,
    MemoryResult,
};
use alloc::{vec, vec::Vec};

/// 一个进程所有关于内存空间管理的信息
pub struct MemorySet {
    /// 维护页表和映射关系
    pub mapping: Mapping,
    /// 每个字段
    pub segments: Vec<Segment>,
}

impl MemorySet {
    /// 创建内核重映射
    pub fn new_kernel() -> MemoryResult<MemorySet> {
        // 在 linker.ld 里面标记的各个字段的起始点，均为 4K 对齐
        extern "C" {
            fn text_start();
            fn rodata_start();
            fn data_start();
            fn bss_start();
        }

        // 建立字段
        let segments = vec![
            // .text 段，r-x
            Segment {
                map_type: MapType::Linear,
                range: Range::from((text_start as usize)..(rodata_start as usize)),
                flags: Flags::READABLE | Flags::EXECUTABLE,
            },
            // .rodata 段，r--
            Segment {
                map_type: MapType::Linear,
                range: Range::from((rodata_start as usize)..(data_start as usize)),
                flags: Flags::READABLE,
            },
            // .data 段，rw-
            Segment {
                map_type: MapType::Linear,
                range: Range::from((data_start as usize)..(bss_start as usize)),
                flags: Flags::READABLE | Flags::WRITABLE,
            },
            // .bss 段，rw-
            Segment {
                map_type: MapType::Linear,
                range: Range::from(VirtualAddress::from(bss_start as usize)..*KERNEL_END_ADDRESS),
                flags: Flags::READABLE | Flags::WRITABLE,
            },
            // 剩余内存空间，rw-
            Segment {
                map_type: MapType::Linear,
                range: Range::from(*KERNEL_END_ADDRESS..VirtualAddress::from(MEMORY_END_ADDRESS)),
                flags: Flags::READABLE | Flags::WRITABLE,
            },
        ];
        let mut mapping = Mapping::new()?;

        // 每个字段在页表中进行映射
        for segment in segments.iter() {
            mapping.map(segment, None)?;
        }
        Ok(MemorySet {
            mapping,
            segments,
        })
    }

    /// 替换 `satp` 以激活页表
    ///
    /// 如果当前页表就是自身，则不会替换，但仍然会刷新 TLB。
    pub fn activate(&self) {
        self.mapping.activate();
    }

    /// 添加一个 [`Segment`] 的内存映射
    pub fn add_segment(&mut self, segment: Segment, init_data: Option<&[u8]>) -> MemoryResult<()> {
        // 检测 segment 没有重合
        assert!(!self.overlap_with(segment.page_range()));
        // 映射
        self.mapping.map(&segment, init_data)?;
        self.segments.push(segment);
        Ok(())
    }

    /// 移除一个 [`Segment`] 的内存映射
    ///
    /// `segment` 必须已经映射
    pub fn remove_segment(&mut self, segment: &Segment) -> MemoryResult<()> {
        // 找到对应的 segment
        let segment_index = self
            .segments
            .iter()
            .position(|s| s == segment)
            .expect("segment to remove cannot be found");
        self.segments.remove(segment_index);
        // 移除映射
        self.mapping.unmap(segment);
        Ok(())
    }

    /// 检测一段内存区域和已有的是否存在重叠区域
    pub fn overlap_with(&self, range: Range<VirtualPageNumber>) -> bool {
        for seg in self.segments.iter() {
            if range.overlap_with(&seg.page_range()) {
                return true;
            }
        }
        false
    }
}

```

另外，还需要在 os/src/memory/frame/frame_tracker.rs 中添加：

```rs

/// `FrameTracker` 可以 deref 得到对应的 `[u8; PAGE_SIZE]`
impl core::ops::Deref for FrameTracker {
    type Target = [u8; PAGE_SIZE];
    fn deref(&self) -> &Self::Target {
        self.page_number().deref_kernel()
    }
}

/// `FrameTracker` 可以 deref 得到对应的 `[u8; PAGE_SIZE]`
impl core::ops::DerefMut for FrameTracker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.page_number().deref_kernel()
    }
}

```

测试：

os/src/main.rs：

```rs

    interrupt::init();
    memory::init();

    let remap = memory::mapping::MemorySet::new_kernel().unwrap();
    remap.activate();

    println!("kernel remapped");

    panic!()

```

输出：

```rs
PMP0: 0x0000000080000000-0x000000008001ffff (A)
PMP1: 0x0000000000000000-0xffffffffffffffff (A,R,W,X)
mod interrupt initialized
mod memory initialized
kernel remapped
src/main.rs:59: 'explicit panic'

```

## 总结：空间的划分和管理

本章我们理清了虚拟地址和物理地址的概念和关系；并利用页表完成虚拟地址到物理地址的映射；最后实现了内核空间段的重映射。

数据结构相关（从上层往下）：

- MemorySet 每个进程所有关于内存空间管理的信息
- mapping 某个进程的内存映射关系
- segments 一个映射片段
- PageTableEntry 页表项
- PageTable 页表
- PageTableTracker 对页表的封装
- VirtualAddress 虚拟地址
- PhysicalAddress 物理地址
- FrameTracker 一个物理页的封装

封装可以利用生命周期特性：所有的封装都可以通过 Deref 获取原本的数据类型

## 页面置换

虚拟内存的一大优势是可以用有限的物理内存空间虚拟出近乎无限的虚拟内存空间，其原理就是只将一部分虚拟内存所对应的数据存放在物理内存中，而剩余的则存放在磁盘（外设）中。

当一个线程操作到那些不在物理内存中的虚拟地址时，就会产生缺页异常（Page Fault）。

此时操作系统会介入，交换一部分物理内存和磁盘中的数据，使得需要访问的内存数据被放入物理内存之中。操作系统还必须更新页表，并刷新缓存。

### 算法

通过一些置换算法，根据前一段时间的内存使用情况，来估计未来哪些地址会被使用，从而将这部分数据保留在物理内存中。

- LRU (Least Recently Used) 算法：将物理内存中最后访问时间最靠前的页面替换出去。

### 这里的实现

在磁盘中建立一个页面置换文件，来保存所有换出的页面：

user/Makefile

增加：

```Makefile

# 编译、打包、格式转换、预留空间
build: dependency
    @cargo build
    @echo Targets: $(patsubst $(SRC_DIR)/%.rs, %, $(SRC_FILES))
    @rm -rf $(OUT_DIR)
    @mkdir -p $(OUT_DIR)
    @cp $(BIN_FILES) $(OUT_DIR)
-->    @dd if=/dev/zero of=$(OUT_DIR)/SWAP_FILE bs=1M count=16
    @rcore-fs-fuse --fs sfs $(IMG_FILE) $(OUT_DIR) zip
    @qemu-img convert -f raw $(IMG_FILE) -O qcow2 $(QCOW_FILE)
    @qemu-img resize $(QCOW_FILE) +1G

```

os/src/fs/swap.rs

实现了一个类似于 FrameTracker 的 SwapTracker, 希望每个进程的 Mapping 都.能够像管理物理页面一样管理这些置换页面。SwapTracker 记录了一个被置换出物理内存的页面，并提供一些便捷的操作接口。

os/src/memory/mapping/swapper.rs

我们定义了一个置换算法的接口，并且实现了一个非常简单的置换算法：这里，Swapper 就替代了 Mapping 中的 mapped_pairs: Vec<(VirtualPageNumber, FrameTracker)> 的作用。

os/src/memory/mapping/mapping.rs

替换 Mapping 中的成员：

```rs
/// 某个进程的内存映射关系
pub struct Mapping {
    /// 保存所有使用到的页表
    page_tables: Vec<PageTableTracker>,
    /// 根页表的物理页号
    root_ppn: PhysicalPageNumber,
    /// 所有分配的物理页面映射信息
    mapped_pairs: SwapperImpl,
    /// 被换出的页面存储在虚拟内存文件中的 Tracker
    swapped_pages: HashMap<VirtualPageNumber, SwapTracker>,
}

```

实现内存置换：遇到缺页异常，找到需要访问的页号、需要访问的页面数据，并置换出一个物理内存中的页号、页面数据，将二者进行交换

```rs

impl Mapping {
    /// 处理缺页异常
    pub fn handle_page_fault(&mut self, stval: usize) -> MemoryResult<()> {
        let vpn = VirtualPageNumber::floor(stval.into());
        let swap_tracker = self
            .swapped_pages
            .remove(&vpn)
            .ok_or("stval page is not mapped")?;
        let page_data = swap_tracker.read();

        if self.mapped_pairs.full() {
            // 取出一个映射
            let (popped_vpn, mut popped_frame) = self.mapped_pairs.pop().unwrap();
            // print!("{:x?} -> {:x?}", popped_vpn, vpn);
            // 交换数据
            swap_tracker.write(&*popped_frame);
            (*popped_frame).copy_from_slice(&page_data);
            // 修改页表映射
            self.invalidate_one(popped_vpn)?;
            self.remap_one(vpn, popped_frame.page_number())?;
            // 更新记录
            self.mapped_pairs.push(vpn, popped_frame);
            self.swapped_pages.insert(popped_vpn, swap_tracker);
        } else {
            // 如果当前还没有达到配额，则可以继续分配物理页面。这种情况目前还不会出现
            // 添加新的映射
            let mut frame = FRAME_ALLOCATOR.lock().alloc()?;
            // 复制数据
            (*frame).copy_from_slice(&page_data);
            // 更新映射
            self.remap_one(vpn, frame.page_number())?;
            // 更新记录
            self.mapped_pairs.push(vpn, frame);
        }
        Ok(())
    }
}

```

令缺页异常调用上面的函数，就完成了页面置换的实现:

os/src/interrupt/handler.rs：

```rs

/// 处理缺页异常
///
/// todo: 理论上这里需要判断访问类型，并与页表中的标志位进行比对
fn page_fault(context: &mut Context, stval: usize) -> Result<*mut Context, String> {
    println!("page_fault");
    let current_thread = PROCESSOR.get().current_thread();
    let memory_set = &mut current_thread.process.write().memory_set;
    memory_set.mapping.handle_page_fault(stval)?;
    memory_set.activate();
    Ok(context)
}

```

这部分等看完文件系统再回过头来完善；

# 实验三：虚实地址转换

1. 原理：在 os/src/entry.asm 中，boot_page_table 的意义是什么？当执行 jal rust_main 时，不考虑缓存，硬件通过哪些地址找到了 rust_main 的第一条指令？

- 低地址的恒等映射：保证程序替换页表后的短暂时间内，pc 仍然可以顺着低地址去执行内存中的指令。
- 高地址：运行 main 函数

对应代码：

```
    # 初始内核映射所用的页表
    .section .data
    .align 12
boot_page_table:
    .quad 0
    .quad 0
    # 第 2 项：0x8000_0000 -> 0x8000_0000，0xcf 表示 VRWXAD 均为 1
    .quad (0x80000 << 10) | 0xcf
    .zero 507 * 8
    # 第 510 项：0xffff_ffff_8000_0000 -> 0x8000_0000，0xcf 表示 VRWXAD 均为 1
    .quad (0x80000 << 10) | 0xcf
    .quad 0
```

- 执行 jal rust_main 时，硬件首先需要加载 rust_main 对应的地址，大概是 0xffff_ffff_802x_xxxx。
- 然后，硬件先从 satp 高位置读取内存映射模式，再从 satp 低位置读取根页表页号，再访问根页表；
- 对于 rust_main 而言，一级页号是其 [30:38] 位，即 510。硬件此时定位到根页表的第 510 项
- 这一项的标志为 XWR，说明它指向一个大页而不是指向下一级页表；这一项的 V 位为 1，说明目标在内存中。
- 因此，硬件寻址到页基址 + 页内偏移
- 找到 rust_main

这一部分完全由硬件完成。

2. 为什么 Mapping 中的 page_tables 和 mapped_pairs 都保存了一些 FrameTracker？二者有何不同？

代码：

```rs
/// 某个线程的内存映射关系
pub struct Mapping {
    /// 保存所有使用到的页表
    page_tables: Vec<PageTableTracker>,
    /// 根页表的物理页号
    root_ppn: PhysicalPageNumber,
    /// 所有分配的物理页面映射信息
    mapped_pairs: VecDeque<(VirtualPageNumber, FrameTracker)>,
}
```
- page_tables 存放了所有页表所用到的页面，而 mapped_pairs 则存放了进程所用到的页面。 
  

3. 分析：假设某进程需要虚拟地址 A 到物理地址 B 的映射，这需要操作系统来完成。那么操作系统在建立映射时有没有访问 B？如果有，它是怎么在还没有映射的情况下访问 B 的呢？

- 建立映射不一定需要访问 B， 但可能会在建立映射的同时向页面中加载一些数据：
- 此时通过线性偏移量来访问
- 通过代码可以很清晰地看到这一点

建立映射的代码：

```rs
/// 加入一段映射，可能会相应地分配物理页面
    ///
    /// 未被分配物理页面的虚拟页号暂时不会写入页表当中，它们会在发生 PageFault 后再建立页表项。
    pub fn map(&mut self, segment: &Segment, init_data: Option<&[u8]>) -> MemoryResult<()> {
        match segment.map_type {
            // 线性映射，直接对虚拟地址进行转换
            MapType::Linear => {
                for vpn in segment.page_range().iter() {
                    self.map_one(vpn, Some(vpn.into()), segment.flags | Flags::VALID)?;
                }
                // 拷贝数据
                if let Some(data) = init_data {
                    unsafe {
                        (&mut *slice_from_raw_parts_mut(segment.range.start.deref(), data.len()))
                            .copy_from_slice(data);
                    }
                }
            }
            // 需要分配帧进行映射
            MapType::Framed => {
                for vpn in segment.page_range().iter() {
                    // 如果有初始化数据，找到相应的数据
                    let page_data = if init_data.is_none() || init_data.unwrap().is_empty() {
                        [0u8; PAGE_SIZE]
                    } else {
                        // 这里必须进行一些调整，因为传入的数据可能并非按照整页对齐

                        // 传入的初始化数据
                        let init_data = init_data.unwrap();
                        // 整理后将要返回的一整个页面的数据
                        let mut page_data = [0u8; PAGE_SIZE];

                        // 拷贝时必须考虑区间与整页不对齐的情况
                        //    start（仅第一页时非零）
                        //      |        stop（仅最后一页时非零）
                        // 0    |---data---|          4096
                        // |------------page------------|
                        let page_address = VirtualAddress::from(vpn);
                        let start = if segment.range.start > page_address {
                            segment.range.start - page_address
                        } else {
                            0
                        };
                        let stop = min(PAGE_SIZE, segment.range.end - page_address);
                        // 计算来源和目标区间并进行拷贝
                        let dst_slice = &mut page_data[start..stop];
                        let src_slice = &init_data[(page_address + start - segment.range.start)
                            ..(page_address + stop - segment.range.start)];
                        dst_slice.copy_from_slice(src_slice);

                        page_data
                    };

                    // 建立映射
                    let mut frame = FRAME_ALLOCATOR.lock().alloc()?;
                    // 更新页表
                    self.map_one(vpn, Some(frame.page_number()), segment.flags)?;
                    // 写入数据
                    (*frame).copy_from_slice(&page_data);
                    // 保存
                    self.mapped_pairs.push_back((vpn, frame));
                }
            }
        }
        Ok(())
    }

```

4. 实验：了解并实现时钟页面置换算法（或任何你感兴趣的算法），可以自行设计样例来比较性能（等等


先看看这个页面置换是怎么做的：

os/src/interrupt/handler.rs

```rs
/// 处理缺页异常
///
/// todo: 理论上这里需要判断访问类型，并与页表中的标志位进行比对
fn page_fault(context: &mut Context, scause: Scause, stval: usize) -> *mut Context {
    static mut COUNT: usize = 0;
    println!("page_fault {}", unsafe {
        COUNT += 1;
        COUNT
    });
    let current_thread = PROCESSOR.lock().current_thread();
    let memory_set = &mut current_thread.process.inner().memory_set;

    match memory_set.mapping.handle_page_fault(stval) {
        Ok(_) => {
            memory_set.activate();
            context
        }
        Err(msg) => fault(msg, scause, stval),
    }
}


```

os/src/memory/mapping/mapping.rs

```rs

/// 处理缺页异常
    pub fn handle_page_fault(&mut self, stval: usize) -> MemoryResult<()> {
        // 发生异常的访问页面
        let vpn = VirtualPageNumber::floor(stval.into());
        // 该页面如果在进程的内存中，则应该在 Swap 中，将其取出
        let swap_tracker: SwapTracker = self
            .swapped_pages
            .remove(&vpn)
            .ok_or("stval page is not mapped")?;
        // 读取该页面的数据
        let page_data = swap_tracker.read();

        if self.mapped_pairs.full() {
            // 取出一个映射
            let (popped_vpn, mut popped_frame) = self.mapped_pairs.pop().unwrap();
            // 交换数据
            swap_tracker.write(&*popped_frame);
            (*popped_frame).copy_from_slice(&page_data);
            // 修改页表映射
            self.invalidate_one(popped_vpn)?;
            let entry = self.remap_one(vpn, popped_frame.page_number())?;
            // 更新记录
            self.mapped_pairs.push(vpn, popped_frame, entry);
            self.swapped_pages.insert(popped_vpn, swap_tracker);
        } else {
            // 添加新的映射
            let mut frame = FRAME_ALLOCATOR.lock().alloc()?;
            // 复制数据
            (*frame).copy_from_slice(&page_data);
            // 更新映射
            let entry = self.remap_one(vpn, frame.page_number())?;
            // 更新记录
            self.mapped_pairs.push(vpn, frame, entry);
        }
        Ok(())
    }

```

通过查找，可以看到最多使用的物理页面数量设置是这样的：

```rs
/// 内核进程最多使用的物理页面数量
pub const KERNEL_PROCESS_FRAME_QUOTA: usize = 16;
/// 用户进程最多使用的物理页面数量
pub const USER_PROCESS_FRAME_QUOTA: usize = 16;
```

这里实现的时钟页面置换算法：

```rs

pub struct ClockSwapper {
    /// 记录映射和添加的顺序
    queue: Vec<(VirtualPageNumber, FrameTracker, usize)>,
    /// 映射数量上限
    quota: usize,
}


impl Swapper for ClockSwapper {

    fn new(quota: usize) -> Self {
        Self {
            queue: Vec::new(),
            quota,
        }
    }
    fn full(&self) -> bool {
        self.queue.len() == self.quota
    }
    fn pop(&mut self) -> Option<(VirtualPageNumber, FrameTracker)> {
        let len = self.queue.len()-1;
        let mut index:usize = 0;
        let mut isfound:bool = false;
        unsafe{
            loop{
                for i in 0..(len+1){
                    let pe = self.queue[i].2.clone() as *mut PageTableEntry;
                    //println!("{:?}",(*pe));
                    let mut flags = (*pe).flags().clone();
                    if flags.contains(Flags::ACCESSED) {
                        flags.set(Flags::ACCESSED,false);
                        //println!("{:?}",flags);
                        (*pe).set_flags(flags);
                    } else {
                        //println!("{:?}",flags);
                        index = i;
                        isfound = true;
                        break;
                    }
                }
                if isfound {
                    break;
                }
            }
        }
        println!("{:?}",index);
        let f = self.queue.remove(index);
        Some((f.0,f.1))
    }
    fn push(&mut self, vpn: VirtualPageNumber, frame: FrameTracker, entry: *mut PageTableEntry) {
        self.queue.push((vpn, frame, entry as usize));
    }
    fn retain(&mut self, predicate: impl Fn(&VirtualPageNumber) -> bool) {
        self.queue.retain(|(vpn, _, _)| predicate(vpn));
    }
}



```

测试：

中间打印的部分log:

```rs
0,PageTableEntry { value: 539831495, page_number: PhysicalPageNumber(527179), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
1,PageTableEntry { value: 539844743, page_number: PhysicalPageNumber(527192), flags: VALID | READABLE | WRITABLE | DIRTY }
page_fault 200
0,PageTableEntry { value: 539831495, page_number: PhysicalPageNumber(527179), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
1,PageTableEntry { value: 539830471, page_number: PhysicalPageNumber(527178), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
2,PageTableEntry { value: 539827399, page_number: PhysicalPageNumber(527175), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
3,PageTableEntry { value: 539832519, page_number: PhysicalPageNumber(527180), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
4,PageTableEntry { value: 539833543, page_number: PhysicalPageNumber(527181), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
5,PageTableEntry { value: 539834567, page_number: PhysicalPageNumber(527182), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
6,PageTableEntry { value: 539835591, page_number: PhysicalPageNumber(527183), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
7,PageTableEntry { value: 539836615, page_number: PhysicalPageNumber(527184), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
8,PageTableEntry { value: 539837639, page_number: PhysicalPageNumber(527185), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
9,PageTableEntry { value: 539838663, page_number: PhysicalPageNumber(527186), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
10,PageTableEntry { value: 539839687, page_number: PhysicalPageNumber(527187), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
11,PageTableEntry { value: 539840711, page_number: PhysicalPageNumber(527188), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
12,PageTableEntry { value: 539841735, page_number: PhysicalPageNumber(527189), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
13,PageTableEntry { value: 539842759, page_number: PhysicalPageNumber(527190), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
14,PageTableEntry { value: 539843783, page_number: PhysicalPageNumber(527191), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
15,PageTableEntry { value: 539844807, page_number: PhysicalPageNumber(527192), flags: VALID | READABLE | WRITABLE | ACCESSED | DIRTY }
0,PageTableEntry { value: 539831431, page_number: PhysicalPageNumber(527179), flags: VALID | READABLE | WRITABLE | DIRTY }
page_fault 201
0,PageTableEntry { value: 539830407, page_number: PhysicalPageNumber(527178), flags: VALID | READABLE | WRITABLE | DIRTY }
test passed

```

# lab4 学习报告

lab4 涉及：

- 线程和进程的概念以及运行状态的表示
- 线程的切换
- 对 CPU 进行抽象在上面完成对线程的调度 

lab4 的代码将会在 lab3 完成的代码上面继续进行改动；

## 线程和进程

- 程序（Program）：从源代码经过编译器一系列处理（编译、链接、优化等）得到的可执行文件
- 进程（Process）：正在运行并使用计算机资源的程序
- 线程 (Thread) ：借助 CPU 和栈的执行流

首先，进程得到了操作系统提供的资源：程序的代码、数据段被加载到内存中，程序所需的虚拟内存空间被真正构建出来。同时操作系统还给进程分配了程序所要求的各种其他资源，如我们上面几个章节中提到过的页表、文件的资源。一个进程可以有多个线程，也可以如传统进程一样只有一个线程。

- 资源的分配单位：进程
- 执行的调度单位：线程

### 线程的表示

在这里我们提供一种基础的实现，每个线程会包括：

- 线程 ID
- 运行栈
- 线程执行上下文 （ Context 类型 ）
- 所属进程的记号
- 内核栈

新建一个 process 文件夹：

先添加一个 mod.rs：

```rs

mod config;
mod kernel_stack;
#[allow(clippy::module_inception)]
mod process;
mod processor;
mod thread;

use crate::interrupt::*;
use crate::memory::*;
use alloc::{sync::Arc, vec, vec::Vec};
use spin::{Mutex, RwLock};

pub use config::*;
pub use kernel_stack::KERNEL_STACK;
pub use process::Process;
pub use processor::PROCESSOR;
pub use thread::Thread;

```

再添加

os/src/process/thread.rs：

包含线程的数据结构：

```rs
//! 线程 [`Thread`]

use super::*;
use crate::fs::*;
use core::hash::{Hash, Hasher};

/// 线程 ID 使用 `isize`，可以用负数表示错误
pub type ThreadID = isize;

static mut THREAD_COUNTER: ThreadID = 0;

/// 线程的信息
pub struct Thread {
    /// 线程 ID
    pub id: ThreadID,
    /// 线程的栈
    pub stack: Range<VirtualAddress>,
    /// 所属的进程
    pub process: Arc<RwLock<Process>>,
    /// 用 `Mutex` 包装一些可变的变量
    pub inner: Mutex<ThreadInner>,
}

/// 线程中需要可变的部分
pub struct ThreadInner {
    /// 线程执行上下文
    ///
    /// 当且仅当线程被暂停执行时，`context` 为 `Some`
    pub context: Option<Context>,
    /// 是否进入休眠
    pub sleeping: bool,
    /// 是否已经结束
    pub dead: bool,
    /// 打开的文件
    pub descriptors: Vec<Arc<dyn INode>>,
}

```

### 进程的表示

进程只需要维护页面映射，并且存储一点额外信息：

- 用户态标识：我们会在后面进行区分内核态线程和用户态线程。
- 访存空间 MemorySet：进程中的线程会共享同一个页表，即可以访问的虚拟内存空间（简称：访存空间）。

创建：os/src/process/process.rs

进程的数据结构：

```rs
//! 进程 [`Process`]

use super::*;
use xmas_elf::ElfFile;

/// 进程的信息
pub struct Process {
    /// 是否属于用户态
    pub is_user: bool,
    /// 进程中的线程公用页表 / 内存映射
    pub memory_set: MemorySet,
}

```

实际上这部分有一些代码和第五章的部分内容交织在了一起。

## 线程的创建

第一个目标就是创建一个线程并且让他运行起来。

一个线程要开始运行，需要这些准备工作：

- 建立页表映射，需要包括以下映射空间：
  - 线程所执行的一段指令
  - 线程执行栈
  - 操作系统的部分内存空间
- 设置起始执行的地址
- 初始化各种寄存器，比如 sp
- 可选：设置一些执行参数（例如 argc 和 argv等 ）

思考：为什么线程即便与操作系统无关，也需要在内存中映射操作系统的内存空间呢？

- 简单来说，需要使用内核来处理中断。
- 我们会为每个进程的页表映射全部操作系统的内存，这些页表都标记为内核权限（即 U 位为 0）。

### 执行第一个线程

需要对 interrupt.asm 稍作修改：

os/src/interrupt.asm

```rs
    
    # 取出 CSR 并保存
    csrr    t0, sstatus
    csrr    t1, sepc
    SAVE    t0, 32
    SAVE    t1, 33
    # 调用 handle_interrupt，传入参数
    # context: &mut Context
    mv      a0, sp
    # scause: Scause
    csrr    a1, scause
    # stval: usize
    csrr    a2, stval
    jal handle_interrupt

    
    .globl __restore
# 离开中断
# 此时内核栈顶被推入了一个 Context，而 a0 指向它
# 接下来从 Context 中恢复所有寄存器，并将 Context 出栈（用 sscratch 记录内核栈地址）
# 最后跳转至恢复的 sepc 的位置
__restore:
    # 从 a0 中读取 sp
    # 思考：a0 是在哪里被赋值的？（有两种情况）
    mv      sp, a0
    # 恢复 CSR
    LOAD    t0, 32
    LOAD    t1, 33
    csrw    sstatus, t0
    csrw    sepc, t1
    # 将内核栈地址写入 sscratch
    addi    t0, sp, 36*8
    csrw    sscratch, t0


```

### 设计 Context

- 保存通用寄存器
- sepc
- sepc：执行 sret 指令后会跳转到这里，所以 sepc 应当存储线程的入口地址（执行的函数地址）
- sstatus

关于 sstatus 标志位的具体意义：

- spp：中断前系统处于内核态（1）还是用户态（0）
- sie：内核态是否允许中断。对用户态而言，无论 sie 取何值都开启中断
- spie：中断前是否开中断（用户态中断时可能 sie 为 0）

设计好 Context 之后，我们只需要将它应用到所有的寄存器上（即执行 __restore），就可以切换到第一个线程了。

os/src/process/processor.rs

```rs

/// 第一次开始运行
///
/// 从 `current_thread` 中取出 [`Context`]，然后直接调用 `interrupt.asm` 中的 `__restore`
/// 来从 `Context` 中继续执行该线程。
pub fn run(&mut self) -> ! {
    // interrupt.asm 中的标签
    extern "C" {
        fn __restore(context: usize);
    }
    /* 激活线程的页表，取得 Context。具体过程会在后面讲解 */
    unsafe {
        __restore(context);
    }
    unreachable!()
}

```

### 在启动时不打开中断

在操作系统初始化的过程中是不应该有中断的。所以，我们删去之前设置「开启中断」的代码。

os/interrupt/timer.rs

```rs

/// 初始化时钟中断
///
/// 开启时钟中断使能，并且预约第一次时钟中断
pub fn init() {
    unsafe {
        // 开启 STIE，允许时钟中断
        sie::set_stimer();
        // 开启 SIE（不是 sie 寄存器），允许内核态被中断打断
        // sstatus::set_sie();
    }
    // 设置下一次时钟中断
    set_next_timeout();
}

```

## 线程的切换

当发生中断时，在 __restore 时，a0 寄存器的值是 handle_interrupt 函数的返回值。也就是说，如果我们令 handle_interrupt 函数返回另一个线程的 *mut Context，就可以在时钟中断后跳转到这个线程来执行。

从这个原理出发，我们就可以完成线程的切换工作

### 修改中断处理

在线程切换时（即时钟中断时），handle_interrupt 函数需要将上一个线程的 Context 保存起来，然后将下一个线程的 Context 恢复并返回。

os/src/interrupt/handler.rs

可以看到，当发生断点中断时，直接返回原来的上下文（修改一下 sepc）；而如果是时钟中断的时候，我们通过执行 PROCESSOR.get().tick(context) 函数得到的返回值作为上下文。

```rs
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) -> *mut Context {
    /* ... */
    // 根据中断类型来处理，返回的 Context 必须位于放在内核栈顶
    match scause.cause() {
        // 断点中断（ebreak）
        Trap::Exception(Exception::Breakpoint) => breakpoint(context),
        // 系统调用
        Trap::Exception(Exception::UserEnvCall) => syscall_handler(context),
        // 时钟中断
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
        // 外部中断（键盘输入）
        Trap::Interrupt(Interrupt::SupervisorExternal) => supervisor_external(context),
        // 其他情况，无法处理
        _ => fault("unimplemented interrupt type: {:x?}", scause, stval),
    }
}

/// 处理 ebreak 断点
fn breakpoint(context: &mut Context) -> *mut Context {
    println!("Breakpoint at 0x{:x}", context.sepc);
    context.sepc += 2;
    context
}

/// 处理时钟中断
fn supervisor_timer(context: &mut Context) -> *mut Context {
    timer::tick();
    PROCESSOR.get().tick(context)
}

```

### 线程切换

关于 Processor::tick 函数是如何实现的：

os/src/process/processor.rs

```rs

/// 在一个时钟中断时，替换掉 context
pub fn tick(&mut self, context: &mut Context) -> *mut Context {
    // 向调度器询问下一个线程
    if let Some(next_thread) = self.scheduler.get_next() {
        if next_thread == self.current_thread() {
            // 没有更换线程，直接返回 Context
            context
        } else {
            // 准备下一个线程
            let next_context = next_thread.run();
            let current_thread = self.current_thread.replace(next_thread).unwrap();
            // 储存当前线程 Context
            current_thread.park(*context);
            // 返回下一个线程的 Context
            next_context
        }
    } else {
        panic!("all threads terminated, shutting down");
    }
}

```

### 上下文 Context 的保存和取出

os/src/process/thread.rs

在线程切换时，我们需要保存前一个线程的 Context，为此我们实现 Thread::park 函数。

```rs

/// 发生时钟中断后暂停线程，保存状态
pub fn park(&self, context: Context) {
    // 检查目前线程内的 context 应当为 None
    let mut slot = self.context.lock();
    assert!(slot.is_none());
    // 将 Context 保存到线程中
    slot.replace(context);
}

```

我们需要取出下一个线程的 Context，为此我们实现 Thread::run。启动一个线程除了需要 Context，还需要切换页表。这个操作我们也在这个方法中完成：

```rs

/// 准备执行一个线程
///
/// 激活对应进程的页表，并返回其 Context
pub fn run(&self) -> *mut Context {
    // 激活页表
    self.process.read().memory_set.activate();
    // 取出 Context
    let parked_frame = self.context.lock().take().unwrap();

    if self.process.read().is_user {
        // 用户线程则将 Context 放至内核栈顶
        KERNEL_STACK.push_context(parked_frame)
    } else {
        // 内核线程则将 Context 放至 sp 下
        let context = (parked_frame.sp() - size_of::<Context>()) as *mut Context;
        unsafe { *context = parked_frame };
        context
    }
}

```

思考：在 run 函数中，我们在一开始就激活了页表，会不会导致后续流程无法正常执行？

此时映射的操作系统内核空间是不变的

我们需要为sp指针准备好一个专门用于在内核态执行函数的内核栈。

## 内核栈

需求：

- 内核栈只会在中断时使用，而中断结束后就不再使用
- 只需要实现一个共用的内核栈就可以了
- 每个线程都需要能够在中断时第一时间找到内核栈的地址
- 我们将内核栈的地址存放到内核态使用的特权寄存器 sscratch 中


解决方案：

- 预留一段空间作为内核栈
- 运行线程时，在 sscratch 寄存器中保存内核栈指针
- 如果线程遇到中断，则从将 Context 压入 sscratch 指向的栈中（Context 的地址为 sscratch - size_of::<Context>()），同时用新的栈地址来替换 sp（此时 sp 也会被复制到 a0 作为 handle_interrupt 的参数） ，这一部分代码可以参考之前的 asm
- 从中断中返回时（__restore 时），a0 应指向被压在内核栈中的 Context。此时出栈 Context 并且将栈顶保存到 sscratch 中

### 具体实现

为内核栈预留空间：我们直接使用一个 static mut 来指定一段空间作为栈。

在我们创建线程时，需要使用的操作就是在内核栈顶压入一个初始状态 Context：

os/src/process/kernel_stack.rs

```rs

//! 内核栈 [`KernelStack`]
//!
//! 用户态的线程出现中断时，因为用户栈无法保证可用性，中断处理流程必须在内核栈上进行。
//! 所以我们创建一个公用的内核栈，即当发生中断时，会将 Context 写到内核栈顶。
//!
//! ### 线程 [`Context`] 的存放
//! > 1. 线程初始化时，一个 `Context` 放置在内核栈顶，`sp` 指向 `Context` 的位置
//! >   （即栈顶 - `size_of::<Context>()`）
//! > 2. 切换到线程，执行 `__restore` 时，将 `Context` 的数据恢复到寄存器中后，
//! >   会将 `Context` 出栈（即 `sp += size_of::<Context>()`），
//! >   然后保存 `sp` 至 `sscratch`（此时 `sscratch` 即为内核栈顶）
//! > 3. 发生中断时，将 `sscratch` 和 `sp` 互换，入栈一个 `Context` 并保存数据
//!
//! 容易发现，线程的 `Context` 一定保存在内核栈顶。因此，当线程需要运行时，
//! 从 [`Thread`] 中取出 `Context` 然后置于内核栈顶即可

use super::*;
use core::mem::size_of;

/// 内核栈
#[repr(align(16))]
#[repr(C)]
pub struct KernelStack([u8; KERNEL_STACK_SIZE]);

/// 公用的内核栈
pub static mut KERNEL_STACK: KernelStack = KernelStack([0; KERNEL_STACK_SIZE]);

impl KernelStack {
    /// 在栈顶加入 Context 并且返回新的栈顶指针
    pub fn push_context(&mut self, context: Context) -> *mut Context {
        // 栈顶
        let stack_top = &self.0 as *const _ as usize + size_of::<Self>();
        // Context 的位置
        let push_address = (stack_top - size_of::<Context>()) as *mut Context;
        unsafe {
            *push_address = context;
        }
        push_address
    }
}

```

os/src/interrupt.asm：

在这个汇编代码中，我们需要加入对 sscratch 的判断和使用,以及事后的恢复：

```rs

    ...
    # 因为线程当前的栈不一定可用，必须切换到内核栈来保存 Context 并进行中断流程
    # 因此，我们使用 sscratch 寄存器保存内核栈地址
    # 思考：sscratch 的值最初是在什么地方写入的？

    # 交换 sp 和 sscratch（切换到内核栈）
    csrrw   sp, sscratch, sp
    # 在内核栈开辟 Context 的空间
    addi    sp, sp, -36*8
    ...

    # 保存通用寄存器，除了 x0（固定为 0）
    SAVE    x1, 1


    ...

# 离开中断
# 此时内核栈顶被推入了一个 Context，而 a0 指向它
# 接下来从 Context 中恢复所有寄存器，并将 Context 出栈（用 sscratch 记录内核栈地址）
# 最后跳转至恢复的 sepc 的位置
__restore:
    # 从 a0 中读取 sp
    # 思考：a0 是在哪里被赋值的？（有两种情况）
    mv      sp, a0
    # 恢复 CSR
    LOAD    t0, 32
    LOAD    t1, 33
    csrw    sstatus, t0
    csrw    sepc, t1
    # 将内核栈地址写入 sscratch
    addi    t0, sp, 36*8
    csrw    sscratch, t0

```

思考：在栈的切换过程中，会不会导致一些栈空间没有被释放，或者被错误释放的情况？

应当是不会的；每个中断在压栈后都会出栈（除了最开始的线程

## 调度器

### 处理器抽象

我们已经可以创建和保存线程了，现在，我们再抽象出「处理器」来存放和管理线程池。同时，也需要存放和管理目前正在执行的线程（即中断前执行的线程，因为操作系统在工作时是处于中断、异常或系统调用服务之中）。

os/src/process/processor.rs

```rs
lazy_static! {
    /// 全局的 [`Processor`]
    pub static ref PROCESSOR: Lock<Processor> = Lock::new(Processor::default());
}

/// 线程调度和管理
#[derive(Default)]
pub struct Processor {
    /// 当前正在执行的线程
    current_thread: Option<Arc<Thread>>,
    /// 线程调度器，记录活跃线程
    scheduler: SchedulerImpl<Arc<Thread>>,
    /// 保存休眠线程
    sleeping_threads: HashSet<Arc<Thread>>,
}

```

注意到这里我们用了一个 Lock，它封装了 spin::Mutex，而在其基础上进一步关闭了中断。这是因为我们在内核线程中也有可能访问 PROCESSOR，但是此时我们不希望它被时钟打断，这样在中断处理中就无法访问 PROCESSOR 了，因为它已经被锁住。

感觉这里最好能提及一下 lock.rs:

```rs
//! 一个关闭中断的互斥锁 [`Lock`]

use spin::{Mutex, MutexGuard};

/// 关闭中断的互斥锁
#[derive(Default)]
pub struct Lock<T>(pub(self) Mutex<T>);

/// 封装 [`MutexGuard`] 来实现 drop 时恢复 sstatus
pub struct LockGuard<'a, T> {
    /// 在 drop 时需要先 drop 掉 [`MutexGuard`] 再恢复 sstatus
    guard: Option<MutexGuard<'a, T>>,
    /// 保存的关中断前 sstatus
    sstatus: usize,
}

```

### 调度器

调度器的算法有许多种，我们将它提取出一个 trait 作为接口

```rs

/// 线程调度器
///
/// 这里 `ThreadType` 就是 `Arc<Thread>`
pub trait Scheduler<ThreadType: Clone + Eq>: Default {
    /// 优先级的类型
    type Priority;
    /// 向线程池中添加一个线程
    fn add_thread(&mut self, thread: ThreadType);
    /// 获取下一个时间段应当执行的线程
    fn get_next(&mut self) -> Option<ThreadType>;
    /// 移除一个线程
    fn remove_thread(&mut self, thread: &ThreadType);
    /// 设置线程的优先级
    fn set_priority(&mut self, thread: ThreadType, priority: Self::Priority);
}

```

### 运行！

补充 Processor::run 的实现，让我们运行起第一个线程:

os/src/process/processor.rs: impl Processor

```rs

/// 第一次开始运行
pub fn run(&mut self) -> ! {
    // interrupt.asm 中的标签
    extern "C" {
        fn __restore(context: usize);
    }
    // 从 current_thread 中取出 Context
    if self.current_thread.is_none() {
        panic!("no thread to run, shutting down");
    }
    let context = self.current_thread().prepare();
    // 从此将没有回头
    unsafe {
        __restore(context as usize);
    }
    unreachable!()
}


```

os/src/main.rs

```rs
/// Rust 的入口函数
///
/// 在 `_start` 为我们进行了一系列准备之后，这是第一个被调用的 Rust 函数
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    memory::init();
    interrupt::init();

    {
        // 新建一个带有内核映射的进程。需要执行的代码就在内核中
        let process = Process::new_kernel().unwrap();

        for message in 0..8 {
            let thread = Thread::new(
                process.clone(),            // 使用同一个进程
                sample_process as usize,    // 入口函数
                Some(&[message]),           // 参数
            ).unwrap();
            PROCESSOR.get().add_thread(thread);
        }
    }

    PROCESSOR.unsafe_get().run();
    // 按照1讨论，应该这样
}

fn sample_process(message: usize) {
    for i in 0..1000000 {
        if i % 200000 == 0 {
            println!("thread {}", message);
        }
    }
}


```

尝试运行：

## 线程的结束

如果我们按照实验指导中的实现，应该会观察到：内核线程在运行完成后触发了 Exception::InstructionPageFault 而终止，其中访问的的地址 stval = 0。

### 解决办法

我们设计一个折衷的解决办法：内核线程将自己标记为“已结束”，同时触发一个普通的异常 ebreak。此时操作系统观察到线程的标记，便将其终止。

```rs

/// 内核线程需要调用这个函数来退出
fn kernel_thread_exit() {
    // 当前线程标记为结束
    PROCESSOR.get().current_thread().as_ref().inner().dead = true;
    // 制造一个中断来交给操作系统处理
    unsafe { llvm_asm!("ebreak" :::: "volatile") };
}

```

同时，在 os/src/main.rs 中：

```rs
// 设置线程的返回地址为 kernel_thread_exit
thread.as_ref().inner().context.as_mut().unwrap().set_ra(kernel_thread_exit as usize);

```

## 总结

本章做的事情：

- 理清线程和进程的概念
- 通过设置 Context，可以构造一个线程的初始状态
- 通过 __restore 标签，直接进入第一个线程之中
- 用 Context 来保存进程的状态，从而实现在时钟中断时切换线程
- 实现内核栈，提供安全的中断处理空间
- 实现调度器，完成线程的调度

相关数据结构：

- Processor
- Thread
- KernelStack


# 实验四（上）：线程

1. 原理：线程切换之中，页表是何时切换的？页表的切换会不会影响程序 / 操作系统的运行？为什么？

    这里实际上和实验指导中是不同的；

    在实验指导中：

    os/src/process/thread.rs: impl Thread

    ```rs
    /// 准备执行一个线程
    ///
    /// 激活对应进程的页表，并返回其 Context
    pub fn run(&self) -> *mut Context {
        // 激活页表
        self.process.read().memory_set.activate();
        // 取出 Context
        let parked_frame = self.context.lock().take().unwrap();

        if self.process.read().is_user {
            // 用户线程则将 Context 放至内核栈顶
            KERNEL_STACK.push_context(parked_frame)
        } else {
            // 内核线程则将 Context 放至 sp 下
            let context = (parked_frame.sp() - size_of::<Context>()) as *mut Context;
            unsafe { *context = parked_frame };
            context
        }
    }
    ```

    在 lab4 的分支代码中，应当是在 Process::prepare_next_thread() 中调用 Thread::prepare():

    ```rs
    /// 准备执行一个线程
    ///
    /// 激活对应进程的页表，并返回其 Context
    pub fn prepare(&self) -> *mut Context {
        // 激活页表
        self.process.write().memory_set.activate();
        // 取出 Context
        let parked_frame = self.inner().context.take().unwrap();
        // 将 Context 放至内核栈顶
        unsafe { KERNEL_STACK.push_context(parked_frame) }
    }
    ```

    它不会影响执行，因为在中断期间是操作系统正在执行，而操作系统所用到的内核线性映射是存在于每个页表中的。（这我记得重复了
    
    上下文 Context 的保存和取出：

    >思考：在 run 函数中，我们在一开始就激活了页表，会不会导致后续流程无法正常执行？
    >
    >不会，因为每一个进程的 MemorySet 都会映射操作系统的空间，否则在遇到中断的时候，将无法执行异常处理。

2. 设计：如果不使用 sscratch 提供内核栈，而是像原来一样，遇到中断就直接将上下文压栈，请举出（思路即可，无需代码）：

- 一种情况不会出现问题
- 一种情况导致异常无法处理（指无法进入 handle_interrupt）
- 一种情况导致产生嵌套异常（指第二个异常能够进行到调用 handle_interrupt，不考虑后续执行情况）
- 一种情况导致一个用户进程（先不考虑是怎么来的）可以将自己变为内核进程，或以内核态执行自己的代码

解答：

由于中断可能是多种原因导致的，也包含了不少错误等内容，因此可以从这些方向考虑：

- 只运行一个非常善意的线程，比如 loop {}：这种情况下和内核进程类似
- 线程把自己的 sp 搞丢了，比如 mv sp, x0。此时无法保存寄存器，也没有能够支持操作系统正常运行的栈：这种情况下出现错误之后就没有办法处理异常和恢复操作系统的状态，操作系统也会崩溃
- 运行两个线程。在两个线程切换的时候，会需要切换页表。但是此时操作系统运行在前一个线程的栈上，一旦切换，再访问栈就会导致缺页，因为每个线程的栈只在自己的页表中；
- 用户进程巧妙地设计 sp，使得它恰好落在内核的某些变量附近，于是在保存寄存器时就修改了变量的值。这相当于任意修改操作系统的控制信息；（这个回答我觉得和题目有点小关联不紧，不过可以提供思路）



1. 实验：当键盘按下 Ctrl + C 时，操作系统应该能够捕捉到中断。实现操作系统捕获该信号并结束当前运行的线程（你可能需要阅读一点在实验指导中没有提到的代码）

    这边有一个qemu特有的属性，需要按下 Ctrl + Alt + C 来在虚拟机中导入 Ctrl + C ，这个要注意一下；

    具体实现比较简单：

    在 `interrupt/handler.rs` 中，修改 `supervisor_external` 函数：

    ```rs
    /// 处理外部中断，只实现了键盘输入
    fn supervisor_external(context: &mut Context) -> Result<*mut Context, String> {
        let mut c = console_getchar();
        if c <= 255 {
            if c == 3 {
                PROCESSOR.get().kill_current_thread();
                PROCESSOR.get().prepare_next_thread();
            }else{
                if c == '\r' as usize {
                    c = '\n' as usize;
                }
                STDIN.push(c as u8);
            }
        }
        Ok(context)
    }


    ```

2. 实验：实现线程的 fork()。目前的内核线程不能进行系统调用，所以我们先简化地实现为“按 F 进入 fork”。fork 后应当为目前的线程复制一份几乎一样的拷贝，新线程与旧线程同属一个进程，公用页表和大部分内存空间，而新线程的栈是一份拷贝。

代码：

handler.rs

```rs

/// 处理外部中断，只实现了键盘输入
fn supervisor_external(context: &mut Context) -> Result<*mut Context, String> {
    let mut c = console_getchar();
    if c <= 255 {
        if c == 3 {
            PROCESSOR.get().kill_current_thread();
            PROCESSOR.get().prepare_next_thread();
        }
        else if c == 'f' as usize {
            PROCESSOR.get().fork_current_thread(context);
        }
        else{
            if c == '\r' as usize {
                c = '\n' as usize;
            }
            STDIN.push(c as u8);
        }
    }
    Ok(context)
}

```

processsor.rs

```rs

    /// 线程的 fork()
    /// fork 后应当为目前的线程复制一份几乎一样的拷贝，新线程与旧线程同属一个进程，公用页表和大部分内存空间，而新线程的栈是一份拷贝。
    pub fn fork_current_thread(&mut self, context: &Context){
        let thread = self.current_thread().fork(*context).unwrap();
        self.add_thread(thread);
    }

```

thread.rs

```rs

pub fn fork(&self, current_context: Context) -> MemoryResult<Arc<Thread>> {

        println!("enter fork");

        // 让所属进程分配并映射一段空间，作为线程的栈
        let stack = self.process
            .write()
            .alloc_page_range(STACK_SIZE, Flags::READABLE | Flags::WRITABLE)?;

        for i in 0..STACK_SIZE {
            *VirtualAddress(stack.start.0 + i).deref::<u8>() = *VirtualAddress(self.stack.start.0 + i).deref::<u8>()
        }

        let mut context = current_context.clone();

        context.set_sp( usize::from(stack.start) -  usize::from(self.stack.start) + current_context.sp()  );

        // 打包成线程
        let thread = Arc::new(Thread {
            id: unsafe {
                THREAD_COUNTER += 1;
                THREAD_COUNTER
            },
            stack,
            process: Arc::clone(&self.process),
            inner: Mutex::new(ThreadInner {
                context: Some(context),
                sleeping: false,
                descriptors: vec![STDIN.clone(), STDOUT.clone()],
            }),
        });

        Ok(thread)
    }

```

测试用户程序：

```rs

pub fn main() -> usize {
    println!("Hello world from user mode program!");
    for i in 0..10000000{
        if i%1000000 == 0 {
            println!("Hello world from user mode program!{}",i);
        }
    }
    0
}

```

输出：

```sh

Hello world from user mode program!6000000
Hello world from user mode program!7000000
Hello world from user mode program!6000000
Hello world from user mode program!8000000
Hello world from user mode program!7000000
Hello world from user mode program!9000000
Thread 3 exit with code 0
Hello world from user mode program!8000000
Hello world from user mode program!9000000
Thread 2 exit with code 0
src/process/processor.rs:101: 'all threads terminated, shutting down'
make[1]: Leaving directory '/home/yunwei/rCore-Tutorial-lab-4/os'

```

# 实验四（下）：线程调度

1. 实验：了解并实现 Stride Scheduling 调度算法，为不同线程设置不同优先级，使得其获得与优先级成正比的运行时间。

os/src/algorithm/src/scheduler/stride_scheduler.rs

```rs

//! 最高响应比优先算法的调度器 [`HrrnScheduler`]

use super::Scheduler;
use alloc::collections::LinkedList;

/// 将线程和调度信息打包
struct StrideThread<ThreadType: Clone + Eq> {
    /// ticket
    ticket: usize,
    /// stride
    pass: usize,
    /// 线程数据
    pub thread: ThreadType,
}

/// 采用 HRRN（最高响应比优先算法）的调度器
pub struct StrideScheduler<ThreadType: Clone + Eq> {
    /// max stride
    big_stride:usize,
    current_min:usize,
    /// 带有调度信息的线程池
    pool: LinkedList<StrideThread<ThreadType>>,
}

/// `Default` 创建一个空的调度器
impl<ThreadType: Clone + Eq> Default for StrideScheduler<ThreadType> {
    fn default() -> Self {
        Self {
            big_stride: 137,
            current_min: 0,
            pool: LinkedList::new(),
        }
    }
}

impl<ThreadType: Clone + Eq> Scheduler<ThreadType> for StrideScheduler<ThreadType> {

    fn add_thread(&mut self, thread: ThreadType, _priority: usize) {
            self.pool.push_back(StrideThread {
                ticket: _priority,
                pass: self.current_min,
                thread,
            })
    }

    fn get_next(&mut self) -> Option<ThreadType> {
        // 计时

        if let Some(best) = self.pool.iter_mut().min_by(|x, y| {
            (x.pass)
                .cmp(&(y.pass))
        }) {
            if best.ticket == 0 {
                best.pass += self.big_stride;
            }else{
                best.pass += self.big_stride / ( best.ticket + 1 );
            }
            self.current_min = best.pass;
            Some(best.thread.clone())
        } else {
            None
        }
    }

    fn remove_thread(&mut self, thread: &ThreadType) {
        // 移除相应的线程并且确认恰移除一个线程
        let mut removed = self.pool.drain_filter(|t| t.thread == *thread);
        assert!(removed.next().is_some() && removed.next().is_none());
    }

    fn set_priority(&mut self, _thread: ThreadType, _priority: usize) {
        for x in self.pool.iter_mut(){
            if x.thread == _thread {
                x.ticket = _priority as usize;
            }
        }
    }
}

```

修改一下 thread 的定义：

```rs
/// 线程中需要可变的部分
pub struct ThreadInner {
    /// 线程执行上下文
    ///
    /// 当且仅当线程被暂停执行时，`context` 为 `Some`
    pub context: Option<Context>,
    /// 是否进入休眠
    pub sleeping: bool,
    /// 是否已经结束
    pub dead: bool,
    /// priority
    pub priority: usize,
}
```

测试代码：

```rs
fn sample_process(id: usize) {
    for i in 0..4000000{
        if i%1000000 == 0 {
            println!("Hello world from kernel mode {} program!{}",id,i);
        }
    }
}
```

结果：

```

Hello world from kernel mode 2 program!0
Hello world from kernel mode 3 program!0
Hello world from kernel mode 4 program!0
Hello world from kernel mode 5 program!0
Hello world from kernel mode 6 program!0
Hello world from kernel mode 7 program!0
Hello world from kernel mode 8 program!0
Hello world from user mode program!
thread 9 exit with code 0
Hello world from kernel mode 1 program!0
Hello world from kernel mode 8 program!1000000
Hello world from kernel mode 7 program!1000000
Hello world from kernel mode 6 program!1000000
Hello world from kernel mode 5 program!1000000
Hello world from kernel mode 4 program!1000000
Hello world from kernel mode 8 program!2000000
Hello world from kernel mode 3 program!1000000
Hello world from kernel mode 7 program!2000000
Hello world from kernel mode 6 program!2000000
Hello world from kernel mode 2 program!1000000
Hello world from kernel mode 5 program!2000000
Hello world from kernel mode 8 program!3000000
Hello world from kernel mode 7 program!3000000
Hello world from kernel mode 4 program!2000000
Hello world from kernel mode 6 program!3000000
thread 8 exit
Hello world from kernel mode 5 program!3000000
Hello world from kernel mode 3 program!2000000
thread 7 exit
Hello world from kernel mode 1 program!1000000
thread 6 exit
Hello world from kernel mode 4 program!3000000
thread 5 exit
Hello world from kernel mode 2 program!2000000
Hello world from kernel mode 3 program!3000000
thread 4 exit
Hello world from kernel mode 2 program!3000000
thread 3 exit
Hello world from kernel mode 1 program!2000000
thread 2 exit
Hello world from kernel mode 1 program!3000000
thread 1 exit
```

2. 分析：
    - 在 Stride Scheduling 算法下，如果一个线程进入了一段时间的等待（例如等待输入，此时它不会被运行），会发生什么？
      - 如果在这种简单的实现下，有可能会出现其他线程等待该线程的情况；比如一个要获取输入的进程的优先级较高，要等它的 pass 经过多个时间片比其他的线程大的时候其他线程才会被调度。
    - 对于两个优先级分别为 9 和 1 的线程，连续 10 个时间片中，前者的运行次数一定更多吗？
     - 并不一定，因为有可能9的线程运行了一下就结束了。
    - 你认为 Stride Scheduling 算法有什么不合理之处？可以怎样改进？
      - 可能会出现等待进程的情况；
      - 也要注意，新加进去的进程的pass不能是0，否则会一直霸占着时间片；


# lab5 学习报告

lab5 涉及：

- 设备树的概念和读取
- virtio 总线协议
- 块设备驱动的实现
- 将块设备托管给文件系统 

这一部分其实在 lab4 的实验中就已经可以部分接触到了（笑

## 设备树

设备树涉及这样一个问题，我们从哪里读取设备信息？

在 RISC-V 中，这个一般是由 bootloader，即 OpenSBI 固件完成的：它来完成对于包括物理内存在内的各外设的扫描，将扫描结果以设备树二进制对象（DTB，Device Tree Blob）的格式保存在物理内存中的某个地方。而这个放置的物理地址将放在 a1 寄存器中。

将会把 HART ID （HART，Hardware Thread，硬件线程，可以理解为执行的 CPU 核）放在 a0 寄存器上。

使用这两个参数，只需要把 rust_main 函数增加两个参数即可：

os/src/main.rs

```rs

#[no_mangle]
pub extern "C" fn rust_main(_hart_id: usize, dtb_pa: PhysicalAddress) -> ! {
    memory::init();
    interrupt::init();
    drivers::init(dtb_pa);
    
    println!("{} {}",_hart_id,dtb_pa);

    ...
}

```

打印输出结果：

0 PhysicalAddress(0x82200000)

### 设备树：

每个设备在物理上连接到了父设备上最后再通过总线等连接起来构成一整个设备树，在每个节点上都描述了对应设备的信息，如支持的协议是什么类型等等。而操作系统就是通过这些节点上的信息来实现对设备的识别的。

一个设备节点上会有几个标准属性：

- compatible：该属性指的是该设备的编程模型
- model：指的是设备生产商给设备的型号
- reg：用 reg 段来自定义存储一些信息

### 解析设备树

可以直接调用 rCore 中 device_tree 库，然后遍历树上节点即可：

os/src/drivers/device_tree.rs

```rs

/// 递归遍历设备树
fn walk(node: &Node) {
    // 检查设备的协议支持并初始化
    if let Ok(compatible) = node.prop_str("compatible") {
        if compatible == "virtio,mmio" {
            virtio_probe(node);
        }
    }
    // 遍历子树
    for child in node.children.iter() {
        walk(child);
    }
}

/// 整个设备树的 Headers（用于验证和读取）
struct DtbHeader {
    magic: u32,
    size: u32,
}

```

在开始的时候，有一步来验证 Magic Number：

这一步是一个保证系统可靠性的要求，是为了验证这段内存到底是不是设备树。

```rs
/// 遍历设备树并初始化设备
pub fn init(dtb_va: VirtualAddress) {
    let header = unsafe { &*(dtb_va.0 as *const DtbHeader) };
    // from_be 是大小端序的转换（from big endian）
    let magic = u32::from_be(header.magic);
    if magic == DEVICE_TREE_MAGIC {
        let size = u32::from_be(header.size);
        // 拷贝数据，加载并遍历
        let data = unsafe { slice::from_raw_parts(dtb_va.0 as *const u8, size as usize) };
        if let Ok(dt) = DeviceTree::load(data) {
            walk(&dt.root);
        }
    }
}

```

## virtio

### 挂载到 QEMU

为了让 QEMU 挂载上我们虚拟的存储设备，我们这里选了 QEMU 支持的 virtio 协议。

virtio：

VirtIO is a standardized interface which allows virtual machines access to simplified "virtual" devices, such as block devices, network adapters and consoles. Accessing devices through VirtIO on a guest VM improves performance over more traditional "emulated" devices, as VirtIO devices require only the bare minimum setup and configuration needed to send and receive data, while the host machine handles the majority of the setup and maintenance of the actual physical hardware.

virtio 起源于 virtio: Towards a De-Facto Standard For Virtual I/O Devices 这篇论文，主要针对于半虚拟化技术中对通用设备的抽象。

以 virtio 为中心的总线下又挂载了 virtio-blk（块设备）总线、virtio-net（网络设备）总线、virtio-pci（PCI 设备）总线等，本身就构成一个设备树。

需要在 QEMU 运行的时候加入选项：

os/Makefile

```makefile

IMG_FILE    := $(USER_BUILD)/disk.img


# 运行 QEMU
qemu: build
    @qemu-system-riscv64 \
            -machine virt \
            -nographic \
            -bios default \
            -device loader,file=$(BIN_FILE),addr=0x80200000 \
            -drive file=$(TEST_IMG),format=raw,id=sfs \      # 模拟存储设备
            -device virtio-blk-device,drive=sfs              # 以 virtio Block Device 的形式挂载到 virtio 总线上

```

### virtio 节点探测

进一步来区分上面提到的那些 virtio 设备：

os/src/drivers/bus/virtio_mmio.rs

由 walk 调用：

```rs

/// 从设备树的某个节点探测 virtio 协议具体类型
pub fn virtio_probe(node: &Node) {
    // reg 属性中包含了描述设备的 Header 的位置
    let reg = match node.prop_raw("reg") {
        Some(reg) => reg,
        _ => return,
    };
    let pa = PhysicalAddress(reg.as_slice().read_be_u64(0).unwrap() as usize);
    let va = VirtualAddress::from(pa);
    let header = unsafe { &mut *(va.0 as *mut VirtIOHeader) };
    // 目前只支持某个特定版本的 virtio 协议
    if !header.verify() {
        return;
    }
    // 判断设备类型
    match header.device_type() {
        DeviceType::Block => virtio_blk::add_driver(header),
        device => println!("unrecognized virtio device: {:?}", device),
    }
}

```

- 从设备树节点的 reg 信息中可以读出设备更详细信息的放置位置；即内存映射读写 MMIO（Memory Mapped I/O）
- 为了访问这段地址，我们需要把它加到页表里面

### virtio_drivers 库

在 cargo.toml 中添加：

```toml
device_tree = { git = "https://github.com/rcore-os/device_tree-rs" }
virtio-drivers = { git = "https://github.com/rcore-os/virtio-drivers" }
```

我们使用 rCore 中的 virtio_drivers 库，这个库会帮我们通过 MMIO 的方式对设备进行交互，同时我们也需要给这个库提供一些诸如申请物理内存、物理地址虚拟转换等接口。

os/src/drivers/bus/virtio_mmio.rs

```rs

lazy_static! {
    /// 用于放置给设备 DMA 所用的物理页（[`FrameTracker`]）
    pub static ref TRACKERS: RwLock<BTreeMap<PhysicalAddress, FrameTracker>> =
        RwLock::new(BTreeMap::new());
}

/// 为 DMA 操作申请连续 pages 个物理页（为 [`virtio_drivers`] 库
#[no_mangle]
extern "C" fn virtio_dma_alloc(pages: usize) -> PhysicalAddress {
    let mut pa: PhysicalAddress = Default::default();
    let mut last: PhysicalAddress = Default::default();
    for i in 0..pages {
        let tracker: FrameTracker = FRAME_ALLOCATOR.lock().alloc().unwrap();
        if i == 0 {
            pa = tracker.address();
        } else {
            assert_eq!(last + PAGE_SIZE, tracker.address());
        }
        last = tracker.address();
        TRACKERS.write().insert(last, tracker);
    }
    return pa;
}

/// 为 DMA 操作释放对应的之前申请的连续的物理页（为 [`virtio_drivers`] 库提供）
#[no_mangle]
extern "C" fn virtio_dma_dealloc(pa: PhysicalAddress, pages: usize) -> i32 {
    for i in 0..pages {
        TRACKERS.write().remove(&(pa + i * PAGE_SIZE));
    }
    0
}

/// 将物理地址转为虚拟地址（为 [`virtio_drivers`] 库提供）
#[no_mangle]
extern "C" fn virtio_phys_to_virt(pa: PhysicalAddress) -> VirtualAddress {
    VirtualAddress::from(pa)
}

/// 将虚拟地址转为物理地址（为 [`virtio_drivers`] 库提供）
#[no_mangle]
extern "C" fn virtio_virt_to_phys(va: VirtualAddress) -> PhysicalAddress {
    Mapping::lookup(va).unwrap()
}

```

由于本身设备是通过直接内存访问DMA（Direct Memory Access）技术来实现数据传输的，CPU 只需要给出要传输哪些内容，放在哪段物理内存上面，把请求告诉设备，设备后面的操作就会利用 DMA 而不经过 CPU 直接传输，在传输结束之后，会通过中断请求 IRQ（Interrupt ReQuest）技术沿着设备树把"我做完了"这个信息告诉 CPU，CPU 会作为一个中断进一步处理。

- 之所以不是虚拟地址是因为 DMA 不会经过 CPU 的 MMU 技术。
- 现在，我们的 FRAME_ALLOCATOR 还只能分配一个帧出来，我们连续调用，暂时先假设他是连续的。

在 0x80000000 到 0x88000000 的区间的物理页有可能对应着两个虚拟页：启动时的线性映射；内核栈是以 Frame 为单位分配的。

## 驱动和块设备驱动

irtio-blk 设备，这种设备提供了以整块为粒度的读和写操作，一般对应到真实的物理设备是硬盘。

### 驱动抽象

在写块设备驱动之前，我们先抽象驱动的概念:

os/src/drivers/driver.rs:

```rs
/// 驱动类型
///
/// 目前只有块设备，可能还有网络、GPU 设备等
#[derive(Debug, Eq, PartialEq)]
pub enum DeviceType {
    Block,
}

/// 驱动的接口
pub trait Driver: Send + Sync {
    /// 设备类型
    fn device_type(&self) -> DeviceType;

    /// 读取某个块到 buf 中（块设备接口）
    fn read_block(&self, _block_id: usize, _buf: &mut [u8]) -> bool {
        unimplemented!("not a block driver")
    }

    /// 将 buf 中的数据写入块中（块设备接口）
    fn write_block(&self, _block_id: usize, _buf: &[u8]) -> bool {
        unimplemented!("not a block driver")
    }
}

lazy_static! {
    /// 所有驱动
    pub static ref DRIVERS: RwLock<Vec<Arc<dyn Driver>>> = RwLock::new(Vec::new());
}

```

不过这样写还是为了方便未来的扩展x

### 抽象块设备

有了驱动的概念，我们就可以进一步抽象块设备：

这里漏了文件夹标签

os/src/drivers/block/mod.rs

这里所谓的 BlockDevice 其实就是一个 Driver 的引用。而且利用 rcore-fs 中提供的 BlockDevice trait 实现了为文件系统的接口，实际上是对上传文件系统的连接。

```rs

use super::driver::Driver;
use alloc::sync::Arc;
use rcore_fs::dev;

pub mod virtio_blk;

/// 块设备抽象（驱动的引用）
pub struct BlockDevice(pub Arc<dyn Driver>);

/// 为 [`BlockDevice`] 实现 [`rcore-fs`] 中 [`BlockDevice`] trait
///
/// 使得文件系统可以通过调用块设备的该接口来读写
impl dev::BlockDevice for BlockDevice {
    /// 每个块的大小（取 2 的对数）
    ///
    /// 这里取 512B 是因为 virtio 驱动对设备的操作粒度为 512B
    const BLOCK_SIZE_LOG2: u8 = 9;

    /// 读取某个块到 buf 中
    fn read_at(&self, block_id: usize, buf: &mut [u8]) -> dev::Result<()> {
        match self.0.read_block(block_id, buf) {
            true => Ok(()),
            false => Err(dev::DevError),
        }
    }

    /// 将 buf 中的数据写入块中
    fn write_at(&self, block_id: usize, buf: &[u8]) -> dev::Result<()> {
        match self.0.write_block(block_id, buf) {
            true => Ok(()),
            false => Err(dev::DevError),
        }
    }

    /// 执行和设备的同步
    ///
    /// 因为我们这里全部为阻塞 I/O 所以不存在同步的问题
    fn sync(&self) -> dev::Result<()> {
        Ok(())
    }
}

```

### virtio-blk 块设备驱动

通过调用现成的库完成 virtio-blk 的驱动:

os/src/drivers/block/virtio_blk.rs

```rs
use super::super::driver::{DeviceType, Driver, DRIVERS};
use alloc::sync::Arc;
use spin::Mutex;
use virtio_drivers::{VirtIOBlk, VirtIOHeader};

/// virtio 协议的块设备驱动
struct VirtIOBlkDriver(Mutex<VirtIOBlk<'static>>);

/// 为 [`VirtIOBlkDriver`] 实现 [`Driver`] trait
///
/// 调用了 [`virtio_drivers`] 库，其中规定的块大小为 512B
impl Driver for VirtIOBlkDriver {
    /// 设备类型
    fn device_type(&self) -> DeviceType {
        DeviceType::Block
    }

    /// 读取某个块到 buf 中
    fn read_block(&self, block_id: usize, buf: &mut [u8]) -> bool {
        self.0.lock().read_block(block_id, buf).is_ok()
    }

    /// 将 buf 中的数据写入块中
    fn write_block(&self, block_id: usize, buf: &[u8]) -> bool {
        self.0.lock().write_block(block_id, buf).is_ok()
    }
}

/// 将从设备树中读取出的设备信息放到 [`static@DRIVERS`] 中
pub fn add_driver(header: &'static mut VirtIOHeader) {
    let virtio_blk = VirtIOBlk::new(header).expect("failed to init blk driver");
    let driver = Arc::new(VirtIOBlkDriver(Mutex::new(virtio_blk)));
    DRIVERS.write().push(driver);
}

```

目前 virtio-drivers 库中的代码虽然调用了 DMA，但是返回时还是阻塞的逻辑，我们这里为了简化也没有设计 IRQ 的响应机制。

关于驱动部分的总结：

设计模式（从上往下）：

- trait BlockDevice
- lockDevice(pub Arc Driver)
- trait Driver
- struct VirtioBlkDriver

其中 Driver 作为一个核心 trait 为上提供实现，上层也就是 Driver 的使用侧（设备的抽象），而下层则是 Driver 的实现侧（设备的实现）。

## 文件系统

之前我们在加载 QEMU 的时候引入了一个磁盘镜像文件，这个文件的打包是由 rcore-fs-fuse 工具 来完成的.

接下来我们需要让操作系统理解块设备里面的文件系统。

### Simple File System

因为文件系统本身比较庞大，我们这里还是采用了 rCore 中的文件系统模块 rcore-fs：

[https://github.com/rcore-os/rcore-fs](https://github.com/rcore-os/rcore-fs)

我们这里选择最简单的 Simple File System

具体实现：

os/src/fs/mod.rs

根目录将会在我们第一次使用 ROOT_INODE 时进行初始化，而初始化的方式是找到全部设备驱动中的第一个存储设备作为根目录。

```rs

lazy_static! {
    /// 根文件系统的根目录的 INode
    pub static ref ROOT_INODE: Arc<dyn INode> = {
        // 选择第一个块设备
        for driver in DRIVERS.read().iter() {
            if driver.device_type() == DeviceType::Block {
                let device = BlockDevice(driver.clone());
                // 动态分配一段内存空间作为设备 Cache
                let device_with_cache = Arc::new(BlockCache::new(device, BLOCK_CACHE_CAPACITY));
                return SimpleFileSystem::open(device_with_cache)
                    .expect("failed to open SFS")
                    .root_inode();
            }
        }
        panic!("failed to load fs")
    };
}

```

BlockCache，该模块也是 rcore-fs 提供的，提供了一个存储设备在内存 Cache 的抽象，通过调用 BlockCache::new(device, BLOCK_CACHE_CAPACITY) 就可以把 device 自动变为一个有 Cache 的设备。

### 测试

os/src/fs/mod.rs

```rs

/// 触发 [`static@ROOT_INODE`] 的初始化并打印根目录内容
pub fn init() {
    ROOT_INODE.ls();
    println!("mod fs initialized");
}

```

os/src/fs/inode_ext.rs

为 [`INode`] 实现 trait [`INodeExt`] 以扩展功能

```rs

use super::*;

/// 为 [`INode`] 类型添加的扩展功能
pub trait INodeExt {
    /// 打印当前目录的文件
    fn ls(&self);

    /// 读取文件内容
    fn readall(&self) -> Result<Vec<u8>>;
}

impl INodeExt for dyn INode {
    fn ls(&self) {
        let mut id = 0;
        while let Ok(name) = self.get_entry(id) {
            println!("{}", name);
            id += 1;
        }
    }

    fn readall(&self) -> Result<Vec<u8>> {
        // 从文件头读取长度
        let size = self.metadata()?.size;
        // 构建 Vec 并读取
        let mut buffer = Vec::with_capacity(size);
        unsafe { buffer.set_len(size) };
        self.read_at(0, buffer.as_mut_slice())?;
        Ok(buffer)
    }
}

```

最后在主函数中测试初始化，然后测试在另一个内核线程中创建个文件夹：

os/src/main.rs

```rs
/// Rust 的入口函数
///
/// 在 `_start` 为我们进行了一系列准备之后，这是第一个被调用的 Rust 函数
#[no_mangle]
pub extern "C" fn rust_main(_hart_id: usize, dtb_pa: PhysicalAddress) -> ! {
    memory::init();
    interrupt::init();
    drivers::init(dtb_pa);
    fs::init();

    let process = Process::new_kernel().unwrap();

    PROCESSOR
        .get()
        .add_thread(Thread::new(process.clone(), simple as usize, Some(&[0])).unwrap());

    // 把多余的 process 引用丢弃掉
    drop(process);

    PROCESSOR.lock().run()
}

/// 测试任何内核线程都可以操作文件系统和驱动
fn simple(id: usize) {
    println!("hello from thread id {}", id);
    // 新建一个目录
    fs::ROOT_INODE
        .create("tmp", rcore_fs::vfs::FileType::Dir, 0o666)
        .expect("failed to mkdir /tmp");
    // 输出根文件目录内容
    fs::ls("/");

    loop {}
}

```

make run：

```

.
..
notebook
hello_world
tmp
Thread {


```

## 小结

本章我们的工作有：

- 在 QEMU 上挂载了存储设备
- 通过读取设备树找到了挂载的设备
- 实现了 virtio 驱动，把物理设备抽象为了驱动
- 进一步把驱动抽象给上层文件系统使用
- 调用 rcore-fs 的文件系统实现对文件的管理


# lab6 学习报告

这一章的实验指导包含：


- 单独生成 ELF 格式的用户程序，并打包进文件系统中
- 创建并运行用户进程
- 使用系统调用为用户程序提供服务

## 构建用户程序框架

接下来，我们需要为用户程序提供一个类似的没有Rust std标准运行时依赖的极简运行时环境。这里我们会快速梳理一遍我们为用户程序进行的流程。

`与 os 的旁边建立一个 ` typo

首先，我们在 os 的旁边建立一个 user crate，移除默认的 main.rs，而是在 src 目录下建立 lib 和 bin 子目录， 在 lib 中存放的是极简运行时环境，在 bin 中存放的源文件会被编译成多个单独的执行文件。

### 基础框架搭建

和操作系统一样，我们需要为用户程序移除 std 依赖，并且补充一些必要的功能：

在 lib.rs 中添加：

- 声明
- 堆栈相关
- panic 处理
- 入口函数

```rs

#![no_std]
#![feature(llvm_asm)]
#![feature(lang_items)]
#![feature(panic_info_message)]
#![feature(linkage)]


/// 大小为 [`USER_HEAP_SIZE`] 的堆空间
static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

/// 使用 `buddy_system_allocator` 中的堆
#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

/// 打印 panic 信息并退出用户程序
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "\x1b[1;31m{}:{}: '{}'\x1b[0m",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("\x1b[1;31mpanic: '{}'\x1b[0m", info.message().unwrap());
    }
    sys_exit(-1);
}

/// 程序入口
#[no_mangle]
pub extern "C" fn _start(_args: isize, _argv: *const u8) -> ! {
    unsafe {
        HEAP.lock()
            .init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    }
    sys_exit(main())
}

/// 默认的 main 函数
///
/// 设置了弱的 linkage，会被 `bin` 中文件的 `main` 函数取代
#[linkage = "weak"]
#[no_mangle]
fn main() -> isize {
    panic!("no main() linked");
}

/// 终止程序
#[no_mangle]
pub extern "C" fn abort() {
    panic!("abort");
}

/// 内存不足时终止程序
#[lang = "oom"]
fn oom(_: Layout) -> ! {
    panic!("out of memory");
}


```

另外，在 .cargo/config 还需要设置编译目标为 RISC-V 64：

```toml
# 编译的目标平台
[build]
target = "riscv64imac-unknown-none-elf"

```

console.rs:

在 stdout stdin 基础上进行输入输出

```rs
//! 在系统调用基础上实现 `print!` `println!`
//!
//! 代码与 `os` crate 中的 `console.rs` 基本相同

use crate::syscall::*;
use alloc::string::String;
use core::fmt::{self, Write};

/// 实现 [`core::fmt::Write`] trait 来进行格式化输出
struct Stdout;

impl Write for Stdout {
    /// 打印一个字符串
    fn write_str(&mut self, s: &str) -> fmt::Result {
        sys_write(STDOUT, s.as_bytes());
        Ok(())
    }
}

/// 打印由 [`core::format_args!`] 格式化后的数据
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

/// 实现类似于标准库中的 `print!` 宏
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

/// 实现类似于标准库中的 `println!` 宏
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

/// 从控制台读取一个字符（阻塞）
pub fn getchar() -> u8 {
    let mut c = [0u8; 1];
    sys_read(STDIN, &mut c);
    c[0]
}

/// 从控制台读取一个或多个字符（阻塞）
pub fn getchars() -> String {
    let mut buffer = [0u8; 64];
    loop {
        let size = sys_read(STDIN, &mut buffer);
        if let Ok(string) = String::from_utf8(buffer.iter().copied().take(size as usize).collect())
        {
            return string;
        }
    }
}

```

## 打包为磁盘镜像

现在，我们只需要利用工具将编译后的用户程序打包为镜像，就可以使用了。

安装工具：

```cargo install rcore-fs-fuse --git https://github.com/rcore-os/rcore-fs```

打包:

这个工具可以将一个目录打包成 SimpleFileSystem 格式的磁盘镜像。

将elf文件单独放在一个导出目录中，即

`user/build/disk`:

user/Makefile

```makefile
.PHONY: build

TARGET      := riscv64imac-unknown-none-elf
MODE        := debug

# 用户程序目录
SRC_DIR		:= src/bin
# 编译后执行文件目录
TARGET_DIR	:= target/$(TARGET)/$(MODE)
# 用户程序源文件
SRC_FILES	:= $(wildcard $(SRC_DIR)/*.rs)
# 根据源文件取得编译后的执行文件
BIN_FILES	:= $(patsubst $(SRC_DIR)/%.rs, $(TARGET_DIR)/%, $(SRC_FILES))

OUT_DIR		:= build/disk
IMG_FILE	:= build/raw.img
QCOW_FILE	:= build/disk.img

# 安装 rcore-fs-fuse 工具
dependency:
ifeq ($(shell which rcore-fs-fuse),)
	@echo Installing rcore-fs-fuse
	@cargo install rcore-fs-fuse --git https://github.com/rcore-os/rcore-fs
endif

# 编译、打包、格式转换、预留空间
build: dependency
	@cargo build
	@echo Targets: $(patsubst $(SRC_DIR)/%.rs, %, $(SRC_FILES))
	@rm -rf $(OUT_DIR)
	@mkdir -p $(OUT_DIR)
	@cp $(BIN_FILES) $(OUT_DIR)
	@rcore-fs-fuse --fs sfs $(IMG_FILE) $(OUT_DIR) zip
	@qemu-img convert -f raw $(IMG_FILE) -O qcow2 $(QCOW_FILE)
	@qemu-img resize $(QCOW_FILE) +1G

clean:
	@cargo clean
	@rm -rf $(OUT_DIR) $(IMG_FILE) $(QCOW_FILE)
```

## 解析 ELF 文件并创建线程

现在，我们需要从 ELF 文件中加载用户程序的代码和数据信息，并且映射到内存中：

使用 xmas-elf 这个 crate 替我们实现了 ELF 的解析。

为 INode 添加一个将整个文件作为 [u8] 读取出来的方法：

os/src/fs/inode_ext.rs

```rs
impl INodeExt for dyn INode {
    fn ls(&self) {
        let mut id = 0;
        while let Ok(name) = self.get_entry(id) {
            println!("{}", name);
            id += 1;
        }
    }

    fn readall(&self) -> Result<Vec<u8>> {
        // 从文件头读取长度
        let size = self.metadata()?.size;
        // 构建 Vec 并读取
        let mut buffer = Vec::with_capacity(size);
        unsafe { buffer.set_len(size) };
        self.read_at(0, buffer.as_mut_slice())?;
        Ok(buffer)
    }
}

```

### 解析各个字段

对于不同的字段进行不同的处理：

os/src/memory/mapping/memory_set.rs

```rs
/// 通过 elf 文件创建内存映射（不包括栈）
pub fn from_elf(file: &ElfFile, is_user: bool) -> MemoryResult<MemorySet> {
    // 建立带有内核映射的 MemorySet
    let mut memory_set = MemorySet::new_kernel()?;

    // 遍历 elf 文件的所有部分
    for program_header in file.program_iter() {
        if program_header.get_type() != Ok(Type::Load) {
            continue;
        }
        // 从每个字段读取「起始地址」「大小」和「数据」
        let start = VirtualAddress(program_header.virtual_addr() as usize);
        let size = program_header.mem_size() as usize;
        let data: &[u8] =
            if let SegmentData::Undefined(data) = program_header.get_data(file).unwrap() {
                data
            } else {
                return Err("unsupported elf format");
            };

        // 将每一部分作为 Segment 进行映射
        let segment = Segment {
            map_type: MapType::Framed,
            range: Range::from(start..(start + size)),
            flags: Flags::user(is_user)
                | Flags::readable(program_header.flags().is_read())
                | Flags::writable(program_header.flags().is_write())
                | Flags::executable(program_header.flags().is_execute()),
        };

        // 建立映射并复制数据
        memory_set.add_segment(segment, Some(data))?;
    }

    Ok(memory_set)
}

```

同时在这里也需要更改一下线程相关数据结构的定义（就不详细写了）

### 加载数据到内存中

思考：我们在为用户程序建立映射时，虚拟地址是 ELF 文件中写明的，那物理地址是程序在磁盘中存储的地址吗？这样做有什么问题吗？

这部分是把文件信息一次性全部加载到内存里面了，因此就是像正常那样映射执行。

我们将修改 Mapping::map 函数，为其增加一个参数表示用于初始化的数据。

```rs

    /// 加入一段映射，可能会相应地分配物理页面
    ///
    /// 未被分配物理页面的虚拟页号暂时不会写入页表当中，它们会在发生 PageFault 后再建立页表项。
    pub fn map(
        &mut self,
        segment: &Segment,
        init_data: Option<&[u8]>,
    ) -> MemoryResult<Vec<(VirtualPageNumber, FrameTracker)>> {
        match segment.map_type {
            // 线性映射，直接对虚拟地址进行转换
            MapType::Linear => {
                for vpn in segment.page_range().iter() {
                    self.map_one(vpn, vpn.into(), segment.flags | Flags::VALID)?;
                }
                // 拷贝数据
                if let Some(data) = init_data {
                    unsafe {
                        (&mut *slice_from_raw_parts_mut(segment.range.start.deref(), data.len()))
                            .copy_from_slice(data);
                    }
                }
                Ok(Vec::new())
            }
            // 需要分配帧进行映射
            MapType::Framed => {
                // 记录所有成功分配的页面映射
                let mut allocated_pairs = Vec::new();
                for vpn in segment.page_range().iter() {
                    // 分配物理页面
                    let mut frame = FRAME_ALLOCATOR.lock().alloc()?;
                    // 映射，填充 0，记录
                    self.map_one(vpn, frame.page_number(), segment.flags | Flags::VALID)?;
                    frame.fill(0);
                    allocated_pairs.push((vpn, frame));
                }

                // 拷贝数据，注意页表尚未应用，无法直接从刚刚映射的虚拟地址访问，因此必须用物理地址 + 偏移来访问。
                if let Some(data) = init_data {
                    // 对于 bss，参数会传入 data，但其长度为 0。我们已经在前面用 0 填充过页面了，因此跳过
                    if !data.is_empty() {
                        for (vpn, frame) in allocated_pairs.iter_mut() {
                            // 拷贝时必须考虑区间与整页不对齐的情况
                            //    start（仅第一页时非零）
                            //      |        stop（仅最后一页时非零）
                            // 0    |---data---|          4096
                            // |------------page------------|
                            let page_address = VirtualAddress::from(*vpn);
                            let start = if segment.range.start > page_address {
                                segment.range.start - page_address
                            } else {
                                0
                            };
                            let stop = min(PAGE_SIZE, segment.range.end - page_address);
                            // 计算来源和目标区间并进行拷贝
                            let dst_slice = &mut frame[start..stop];
                            let src_slice = &data[(page_address + start - segment.range.start)
                                ..(page_address + stop - segment.range.start)];
                            dst_slice.copy_from_slice(src_slice);
                        }
                    }
                }

                Ok(allocated_pairs)
            }
        }
    }

```

### 运行 Hello World

os/src/main.rs

```rs

// 从文件系统中找到程序
let app = fs::ROOT_INODE.find("hello_world").unwrap();
// 读取数据
let data = app.readall().unwrap();
// 解析 ELF 文件
let elf = ElfFile::new(data.as_slice()).unwrap();
// 利用 ELF 文件创建线程，映射空间并加载数据
let process = Process::from_elf(&elf, true).unwrap();
// 再从 ELF 中读出程序入口地址
let thread = Thread::new(process, elf.header.pt2.entry_point() as usize, None).unwrap();
// 添加线程
PROCESSOR.lock().add_thread(thread);

```

## 实现系统调用

为练习做准备：
- sys_read
- sys_write 
- sys_exit 

### 用户程序中调用系统调用

实验指导提供了这样一种系统调用格式

```rs
llvm_asm!("ecall" :
    "={x10}" (/* 返回读取长度 */) :
    "{x10}" (/* 文件描述符 */),
    "{x11}" (/* 读取缓冲区 */),
    "{x12}" (/* 缓冲区长度 */),
    "{x17}" (/* sys_read 编号 63 */) ::
);

```
### 避免忙等待

这里的读写系统调用都是阻塞的。

### 实现系统调用的思路

把系统调用的处理结果分为三类：


- 返回一个数值，程序继续执行
- 程序进入等待
- 程序将被终止

系统调用的处理流程：

- 首先，从相应的寄存器中取出调用代号和参数
- 根据调用代号，进入不同的处理流程，得到处理结果：
  - 返回数值并继续执行：返回值存放在 x10 寄存器，sepc += 4，继续此 context 的执行
  - 程序进入等待：同样需要更新 x10 和 sepc，但是需要将当前线程标记为等待，切换其他线程来执行
  - 程序终止：不需要考虑系统调用的返回，直接终止线程

os/src/kernel/syscall.rs

系统调用的总入口：

```rs
//! 实现各种系统调用

use super::*;
use alloc::{format, string::String};

pub const SYS_READ: usize = 63;
pub const SYS_WRITE: usize = 64;
pub const SYS_EXIT: usize = 93;

/// 系统调用在内核之内的返回值
pub(super) enum SyscallResult {
    /// 继续执行，带返回值
    Proceed(isize),
    /// 记录返回值，但暂存当前线程
    Park(isize),
    /// 丢弃当前 context，调度下一个线程继续执行
    Kill,
}

/// 系统调用的总入口
pub fn syscall_handler(context: &mut Context) -> Result<*mut Context, String> {
    // 无论如何处理，一定会跳过当前的 ecall 指令
    context.sepc += 4;

    let syscall_id = context.x[17];
    let args = [context.x[10], context.x[11], context.x[12]];

    let result = match syscall_id {
        SYS_READ => sys_read(args[0], args[1] as *mut u8, args[2]),
        SYS_WRITE => sys_write(args[0], args[1] as *mut u8, args[2]),
        SYS_EXIT => sys_exit(args[0]),
        _ => return Err(format!("unimplemented syscall: {}", syscall_id)),
    };

    Ok(match result {
        SyscallResult::Proceed(ret) => {
            // 将返回值放入 context 中
            context.x[10] = ret as usize;
            context
        }
        SyscallResult::Park(ret) => {
            // 将返回值放入 context 中
            context.x[10] = ret as usize;
            // 保存 context，准备下一个线程
            PROCESSOR.get().park_current_thread(context);
            PROCESSOR.get().prepare_next_thread()
        }
        SyscallResult::Kill => {
            // 终止，跳转到 PROCESSOR 调度的下一个线程
            PROCESSOR.get().kill_current_thread();
            PROCESSOR.get().prepare_next_thread()
        }
    })
}


```

## 处理文件描述符

这里我们只为 stdin 和 stdout 实现最简单的读写接口。

- 首先，操作系统需要为进程维护一个进程打开的文件清单；
- stdin 和 stdout，它们的文件描述符数值分别为 0 和 1；
- 输出流最为简单：每当遇到系统调用时，直接将缓冲区中的字符通过 SBI 调用打印出去。
- 输入流较为复杂：每当遇到系统调用时，通过中断或轮询方式获取字符：如果有，就进一步获取；如果没有就等待。直到收到约定长度的字符串才返回。

因此，我们来看看外部中断的实现：

每一个键盘按键对于操作系统而言都是一次短暂的中断：

OpenSBI 默认会关闭各种外部中断，需要将其打开，来接受按键信息。

这里需要调整一下中断开启部分：

os/src/interrupt/handler.rs

```rs

/// 初始化中断处理
///
/// 把中断入口 `__interrupt` 写入 `stvec` 中，并且开启中断使能
pub fn init() {
    unsafe {
        extern "C" {
            /// `interrupt.asm` 中的中断入口
            fn __interrupt();
        }
        // 使用 Direct 模式，将中断入口设置为 `__interrupt`
        stvec::write(__interrupt as usize, stvec::TrapMode::Direct);

        // 开启外部中断使能
        sie::set_sext();

        // 在 OpenSBI 中开启外部中断
        *PhysicalAddress(0x0c00_2080).deref_kernel() = 1 << 10;
        // 在 OpenSBI 中开启串口
        *PhysicalAddress(0x1000_0004).deref_kernel() = 0x0bu8;
        *PhysicalAddress(0x1000_0001).deref_kernel() = 0x01u8;
    }
}

```

### 实现输入输出流

这里，缓冲区使用 alloc::collections::VecDeque 来实现。

在遇到键盘中断时，调用 sbi_call 来获取字符并加入到缓冲区中。当遇到系统调用 sys_read 时，再相应从缓冲区中取出一定数量的字符。

os/src/kernel/fs.rs

```rs

//! 文件相关的内核功能

use super::*;
use crate::fs::*;
use core::slice::from_raw_parts_mut;

/// 从指定的文件中读取字符
///
/// 如果缓冲区暂无数据，返回 0；出现错误返回 -1
// todo: inode 放到 process 中去
pub(super) fn sys_read(fd: usize, buffer: *mut u8, size: usize) -> SyscallResult {
    // 从线程中获取 inode，注意避免锁
    let inode: Arc<dyn INode> =
        if let Some(inode) = PROCESSOR.get().current_thread().inner().descriptors.get(fd) {
            inode.clone()
        } else {
            return SyscallResult::Proceed(-1);
        };
    let buffer = unsafe { from_raw_parts_mut(buffer, size) };
    if let Ok(ret) = inode.read_at(0, buffer) {
        let ret = ret as isize;
        if ret > 0 {
            return SyscallResult::Proceed(ret);
        }
        if ret == 0 {
            return SyscallResult::Park(ret);
        }
    }
    SyscallResult::Proceed(-1)
}

/// 将字符写入指定的文件
pub(super) fn sys_write(fd: usize, buffer: *mut u8, size: usize) -> SyscallResult {
    if let Some(inode) = PROCESSOR.get().current_thread().inner().descriptors.get(fd) {
        let buffer = unsafe { from_raw_parts_mut(buffer, size) };
        if let Ok(ret) = inode.write_at(0, buffer) {
            let ret = ret as isize;
            if ret >= 0 {
                return SyscallResult::Proceed(ret);
            }
        }
    }
    SyscallResult::Proceed(-1)
}

```

在 os/src/fs/stdout.rs 中，实现stdout：

```rs

//! 控制台输出 [`Stdout`]

use super::*;

lazy_static! {
    pub static ref STDOUT: Arc<Stdout> = Default::default();
}

/// 控制台输出
# [derive(Default)]
pub struct Stdout;

impl INode for Stdout {
    fn write_at(&self, offset: usize, buf: &[u8]) -> Result<usize> {
        if offset != 0 {
            Err(FsError::NotSupported)
        } else if let Ok(string) = core::str::from_utf8(buf) {
            print!("{}", string);
            Ok(buf.len())
        } else {
            Err(FsError::InvalidParam)
        }
    }

    /// Read bytes at `offset` into `buf`, return the number of bytes read.
    fn read_at(&self, _offset: usize, _buf: &mut [u8]) -> Result<usize> {
        Err(FsError::NotSupported)
    }

    fn poll(&self) -> Result<PollStatus> {
        Err(FsError::NotSupported)
    }

    /// This is used to implement dynamics cast.
    /// Simply return self in the implement of the function.
    fn as_any_ref(&self) -> &dyn Any {
        self
    }
}


```

## 条件变量

条件变量用来解决这样一个问题：如果遇到了 sys_read 系统调用，而缓冲区并没有数据可以读取，应该如何让线程进行等待，而又不浪费 CPU 资源呢？

条件变量（conditional variable）的常见接口是这样的：

- wait：当前线程开始等待这个条件变量
- notify_one：让某一个等待此条件变量的线程继续运行
- notify_all：让所有等待此变量的线程继续运行

条件变量和互斥锁的区别在于：
- 互斥锁解铃还须系铃人，但条件变量可以由任何来源发出 notify 信号。
- 互斥锁的一次 lock 一定对应一次 unlock，但条件变量多次 notify 只能保证 wait 的线程执行次数不超过 notify 次数。

为输入流加入条件变量后，就可以使得调用 sys_read 的线程在等待期间保持休眠，不被调度器选中，消耗 CPU 资源。

### 调整调度器

首先，我们需要为线程池单独设立一个「休眠区」，其中保存的线程与调度器互斥。：

os/src/process/processor.rs

```rs

pub struct Processor {
    /// 当前正在执行的线程
    current_thread: Option<Arc<Thread>>,
    /// 线程调度器，记录活跃线程
    scheduler: SchedulerImpl<Arc<Thread>>,
    /// 保存休眠线程
    sleeping_threads: HashSet<Arc<Thread>>,
}

```

### 实现条件变量

条件变量会被包含在输入流等涉及等待和唤起的结构中，而一个条件变量保存的就是所有等待它的线程。

os/src/kernel/condvar.rs

```rs

//! 条件变量

use super::*;
use alloc::collections::VecDeque;

# [derive(Default)]
pub struct Condvar {
    /// 所有等待此条件变量的线程
    watchers: Mutex<VecDeque<Arc<Thread>>>,
}

impl Condvar {
    /// 令当前线程休眠，等待此条件变量
    pub fn wait(&self) {
        self.watchers
            .lock()
            .push_back(PROCESSOR.get().current_thread());
        PROCESSOR.get().sleep_current_thread();
    }

    /// 唤起一个等待此条件变量的线程
    pub fn notify_one(&self) {
        if let Some(thread) = self.watchers.lock().pop_front() {
            PROCESSOR.get().wake_thread(thread);
        }
    }
}

```

当一个线程调用 sys_read 而缓冲区为空时，就会将其加入条件变量的 watcher 中，同时在 Processor 中移出活跃线程。而当键盘中断到来，读取到字符时，就会将线程重新放回调度器中，准备下一次调用。

os/src/fs/stdin.rs

```rs
//! 键盘输入 [`Stdin`]

use super::*;
use alloc::collections::VecDeque;

lazy_static! {
    pub static ref STDIN: Arc<Stdin> = Default::default();
}

/// 控制台键盘输入，实现 [`INode`] 接口
# [derive(Default)]
pub struct Stdin {
    /// 从后插入，前段弹出
    buffer: Mutex<VecDeque<u8>>,
    /// 条件变量用于使等待输入的线程休眠
    condvar: Condvar,
}

impl INode for Stdin {
    /// Read bytes at `offset` into `buf`, return the number of bytes read.
    fn read_at(&self, offset: usize, buf: &mut [u8]) -> Result<usize> {
        if offset != 0 {
            // 不支持 offset
            Err(FsError::NotSupported)
        } else if self.buffer.lock().len() == 0 {
            // 缓冲区没有数据，将当前线程休眠
            self.condvar.wait();
            Ok(0)
        } else {
            let mut stdin_buffer = self.buffer.lock();
            for (i, byte) in buf.iter_mut().enumerate() {
                if let Some(b) = stdin_buffer.pop_front() {
                    *byte = b;
                } else {
                    return Ok(i);
                }
            }
            Ok(buf.len())
        }
    }

    /// Write bytes at `offset` from `buf`, return the number of bytes written.
    fn write_at(&self, _offset: usize, _buf: &[u8]) -> Result<usize> {
        Err(FsError::NotSupported)
    }

    fn poll(&self) -> Result<PollStatus> {
        Err(FsError::NotSupported)
    }

    /// This is used to implement dynamics cast.
    /// Simply return self in the implement of the function.
    fn as_any_ref(&self) -> &dyn Any {
        self
    }
}

impl Stdin {
    /// 向缓冲区插入一个字符，然后唤起一个线程
    pub fn push(&self, c: u8) {
        self.buffer.lock().push_back(c);
        self.condvar.notify_one();
    }
}


```

开放思考：如果多个线程同时等待输入流会怎么样？有什么解决方案吗？

会导致只有一个线程获取输入，别的就一直被阻塞。

比对源代码，这边还发现一个问题：

```rs
    .global boot_page_table
boot_page_table:
    # .8byte表示长度为8个字节的整数
    .8byte 0
    .8byte 0
    # 第 2 项：0x8000_0000 -> 0x8000_0000，0xcf 表示 VRWXAD 均为 1
    .8byte (0x80000 << 10) | 0xcf
    .zero 505 * 8
    # 第 508 项：0xffff_ffff_0000_0000 -> 0x0000_0000，0xcf 表示 VRWXAD 均为 1
    .8byte (0x00000 << 10) | 0xcf
    .8byte 0
    # 第 510 项：0xffff_ffff_8000_0000 -> 0x8000_0000，0xcf 表示 VRWXAD 均为 1
    .8byte (0x80000 << 10) | 0xcf
    .8byte 0

```

和之前相比，这个需要改成 8byte（好像是前几天的修改

## 总结

其实这一章的内容比较多（但很多代码没有写出来...

- 我们成功单独生成 ELF 格式的用户程序，并打包进文件系统中
- 从文件中读取，创建并运行用户进程
- 而为了可以让用户程序享受到操作系统的功能，我们使用系统调用为用户程序提供服务。 


# 实验六：系统调用

这一部分的代码是在 master 分支上进行实现的：

1. 原理：使用条件变量之后，分别从线程和操作系统的角度而言读取字符的系统调用是阻塞的还是非阻塞的？

- 对于线程，阻塞；
- 对于操作系统，非阻塞；

2. 设计：如果要让用户线程能够使用 Vec 等，需要做哪些工作？如果要让用户线程能够使用大于其栈大小的动态分配空间，需要做哪些工作？

- 应当要在用户部分实现 #[global_allocator] ：包含 [`alloc::alloc::GlobalAlloc`] trait等
- 另外开辟一个空间作为用户堆；

3. 实验：实现 get_tid 系统调用，使得用户线程可以获取自身的线程 ID。

user/src/syscall.rs

```rs

const SYSCALL_GETTID: usize = 94;

pub fn sys_get_tid() -> isize {
    syscall(SYSCALL_GETTID, 0, 0, 0)
}

```

os/src/kernel/syscall.rs

```rs

pub const SYS_GETTID: usize = 94;

...
let result = match syscall_id {
        SYS_READ => sys_read(args[0], args[1] as *mut u8, args[2]),
        SYS_WRITE => sys_write(args[0], args[1] as *mut u8, args[2]),
        SYS_EXIT => sys_exit(args[0]),
        SYS_GETTID => sys_get_tid(),
        _ => {
            println!("unimplemented syscall: {}", syscall_id);
            SyscallResult::Kill
        }
    };
...


```

os/src/kernel/process.rs

```rs
pub(super) fn sys_get_tid() -> SyscallResult {
    SyscallResult::Proceed(PROCESSOR.lock().current_thread().id.clone())
}

```


测试程序：

user/src/bin/hello_world.rs

```rs
pub fn main() -> usize {
    println!("Hello world from user mode program!");
    println!("tid: {}",sys_get_tid());
    0
}
```

结果：

>Hello world from user mode program!
>tid: 1


4. 实验：将你在实验四（上）实现的 clone 改进成为 sys_clone 系统调用，使得该系统调用为父线程返回自身的线程 ID，而为子线程返回 0。

在实验四（上）实现的代码未作改动；（虽然实验指导里面的名字换了）

os/src/kernel/syscall.rs

```rs

pub const SYS_CLONE: usize = 95;

...

let result = match syscall_id {
        SYS_READ => sys_read(args[0], args[1] as *mut u8, args[2]),
        SYS_WRITE => sys_write(args[0], args[1] as *mut u8, args[2]),
        SYS_EXIT => sys_exit(args[0]),
        SYS_GETTID => sys_get_tid(),
        SYS_CLONE => sys_clone(context),
        _ => {
            println!("unimplemented syscall: {}", syscall_id);
            SyscallResult::Kill
        }
    };

```

os/src/kernel/process.rs

```rs

pub(super) fn sys_clone(context: &Context) -> SyscallResult {
    let id = PROCESSOR.lock().current_thread().id.clone();
    PROCESSOR.lock().fork_current_thread(context);
    if PROCESSOR.lock().current_thread().id.clone() == id {
        SyscallResult::Proceed(id)
    }else{
        SyscallResult::Proceed(0)
    }
}

```

user/src/syscall.rs

```rs

pub fn sys_clone() -> isize {
    syscall(SYSCALL_CLONE, 0, 0, 0)
}

```

测试程序：

user/src/bin/hello_world.rs

```rs
# [no_mangle]
pub fn main() -> usize {
    println!("Hello world from user mode program!");
    println!("tid: {}",sys_get_tid());
    println!("clone: {}",sys_clone());
    println!("tid: {}",sys_get_tid());
    0
}


```

结果：

```rs

Hello world from user mode program!
tid: 1
enter fork
clone: 1
tid: 1
thread 1 exit with code 0
clone: 0
tid: 2
thread 2 exit with code 0
src/process/processor.rs:88: 'all threads terminated, shutting down'

```


5. 实验：将一个文件打包进用户镜像，并让一个用户进程读取它并打印其内容。需要实现 sys_open，将文件描述符加入进程的 descriptors 中，然后通过 sys_read 来读取。


- 打包：

在 user 根目录下放置 hello_world.rs 文件：

user/Makefile

```makefile
ADD_FILES	:= hello_world.rs

# 编译、打包、格式转换、预留空间
build: dependency
	@cargo build
	@echo Targets: $(patsubst $(SRC_DIR)/%.rs, %, $(SRC_FILES))
	@rm -rf $(OUT_DIR)
	@mkdir -p $(OUT_DIR)
	@cp $(BIN_FILES) $(OUT_DIR)
	@cp $(ADD_FILES) $(OUT_DIR)
	@rcore-fs-fuse --fs sfs $(IMG_FILE) $(OUT_DIR) zip
	@qemu-img convert -f raw $(IMG_FILE) -O qcow2 $(QCOW_FILE)
	@qemu-img resize $(QCOW_FILE) +1G

```

os/src/kernel/fs.rs

```rs

pub(super) fn sys_open(buffer: *mut u8, size: usize) -> SyscallResult {
    let name = unsafe {
        let slice = slice::from_raw_parts(buffer, size);
        str::from_utf8(slice).unwrap()
    };
    // 从文件系统中找到程序
    let file = ROOT_INODE.find(name).unwrap();
    let process = PROCESSOR.lock().current_thread().process.clone();
    process.inner().descriptors.push(file);
    SyscallResult::Proceed(
        (PROCESSOR
            .lock()
            .current_thread()
            .process
            .clone()
            .inner()
            .descriptors
            .len()
            - 1) as isize,
    )
}

```

user/src/syscall.rs

```rs
pub fn sys_open(name: &str) -> isize {
    syscall(
        SYSCALL_OPEN,
        0,
        name.as_ptr() as *const u8 as usize,
        name.len(),
    )
}


```

测试程序：

```rs
# ![no_std]
# ![no_main]

# [macro_use]
extern crate user_lib;

use user_lib::alloc::string::String;
use user_lib::syscall::*;

# [no_mangle]
pub fn main() -> usize {
    println!("Hello world from user mode program!");
    println!("tid: {}", sys_get_tid());
    let fd = sys_open("hello_world.rs");
    println!("fd: {}", fd);
    let mut buffer = [0u8; 1024];
    let size = sys_read(fd as usize, &mut buffer);
    if let Ok(string) = String::from_utf8(buffer.iter().copied().take(size as usize).collect()) {
        print!("{}", string);
    }
    0
}


```

输出：


```
Hello world from user mode program!
tid: 1
fd: 2
# [no_mangle]
pub fn main() -> usize {
    println!("Hello world from user mode program!");
    println!("tid: {}",sys_get_tid());
    println!("clone: {}",sys_clone());
    println!("tid: {}",sys_get_tid());
    0
}
thread 1 exit with code 0

```

6. 挑战实验：实现 sys_pipe，返回两个文件描述符，分别为一个管道的读和写端。用户线程调用完 sys_pipe 后调用 sys_fork，父线程写入管道，子线程可以读取。读取时尽量避免忙等待。
