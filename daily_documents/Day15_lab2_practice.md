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