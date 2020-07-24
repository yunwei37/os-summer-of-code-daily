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
#[no_mangle]
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
#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::alloc::string::String;
use user_lib::syscall::*;

#[no_mangle]
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
#[no_mangle]
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
