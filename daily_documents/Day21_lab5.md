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


```rs
```


```rs
```


```rs
```


```rs
```


```rs
```