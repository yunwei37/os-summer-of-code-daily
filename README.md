# Daily Schedule for OS Tutorial Summer of Code 2020

- daily_documents：每日笔记
- part1-exercises-for-rust：rust的一些小练习
- practice：rust写的一些小程序
- labs：rCore的lab: 根据lab1~6的各个小节和代码，自己一步一步手写代码重现整个实现过程，并提交各个阶段的code成果

可供检查的具体实现目录：

- [rCore labs](labs/README.md)
- [rust practices](part1-exercises-for-rust/README.md)

## TOC

*七月*

| Mon               | Tues              | Wed                          | Thur                         | Fri                          | Sat               | Sun               |
| ----------------- | ----------------- | ---------------------------- | ---------------------------- | ---------------------------- | ----------------- | ----------------- |
|                   |                   | 1 <br> ([D1](#day-1-202071)) | 2 <br> ([D2](#day-2-202072)) | 3 <br> ([D3](#day-3-202073)) | 4 <br> ([D4](#day-4-202074)) | 5 <br> ([D5](#day-5-202075)) |
| 6 <br> ([D6](#day-6-202076)) | 7 <br> ([D7](#day-7-202077)) | 8 <br> ([D8](#day-8-202078))            | 9 <br> ([D9](#day-9-202079))            | 10 <br> ([D10](#day-10-2020710))         | 11  <br>  ([D11](#day-11-2020711))             | 12      <br>    ([D12](#day-12-2020712))       |
| 13    <br>    ([D13](#day-13-2020713))             | 14         <br>    ([D14](#day-14-2020711))        | 15        <br>    ([D15](#day-15-2020715))                    | 16    <br>     ([D16](#day-16-2020716))                       | 17    <br>      ([D17](#day-17-2020717))                       | 18    <br>    ([D18](#day-18-2020718))            | 19   <br>     ([D19](#day-19-2020719))            |
| 20   <br>    ([D20](#day-20-2020720))            | 21                | 22                           | 23                           | 24                           | 25                | 26                |
| 27                | 28                | 29                           | 30                           |                              |                   |                   |

------

## Day 1 2020/7/1

### 事件1：OS Tutorial Summer of Code 2020

前天就在github上面看到了活动信息，然后立马就投了简历和报名表；正好一直以来就对系统相关的信息很感兴趣，也很早就看到了rcore这样一个项目，暑假也没啥别的事情，这样的好机会当然不能错过啦。（就是今天傍晚才收到回复...所以今天算第一天吧）

相关的计算机原理知识大致都了解过一遍，但没有什么很好的实践；学校教的是MIPS体系结构，rust大名如雷贯耳，但也未曾上手。大概还是有挺多东西要学的。

对于课程的计划时间表，也许有点希望尝试一下把这几个部分进行一定的交叉（？），从了解rust语法和RISC-V的原理开始，结合rcore labs相关的源代码阅读和一定的rust实践工作交叉进行...

### 事件2：rust beginner

今天大部分时间还是花在了mit的6.828上面（继续之前的学习），晚上才算是真正开始（入坑）rust。

主要参考资料：[Rust 程序设计语言](https://kaisery.github.io/trpl-zh-cn/)

- 安装并配置环境；换到国内源
- Hello, World!
- 简单的猜猜看游戏

参考今日文档记录：[Day1_rust_beginner.md](daily_documents/Day1_rust_beginner.md)

顺带瞻仰了一下rcore的设计论文，晚上去了一家还不错的日料店（吃到了河豚锅、海胆刺身还有很不错的土瓶烧！）

>rust看起来真好玩

### 问题

- `waiting for file lock on package cache lock` 折腾了一会，虽然解决了问题但并不很清楚其发生的原因；
- 大概了解了一下rust的设计理念，但语法还需要更深入地学习；

### 预期计划

- 睡个好觉，明天继续看rust基本知识；
- 简单了解一下 RISC-V 体系结构；
- 翻翻rCore的论文和tutorial，大概了解一下lab是要做什么怎么做的；

## Day 2 2020/7/2

### 事件1：rust PL again

主要参考资料：[Rust 程序设计语言](https://kaisery.github.io/trpl-zh-cn/)

继续阅读 Rust程序设计语言，梳理相关语法；

- 常见编程概念
- 所有权
- 结构体
- 枚举、方法匹配
- 使用包和crate
- 集合
- 错误处理
- 泛型、trait 和生命周期

笔记：[Day2_rust_next.md](daily_documents/Day2_rust_next.md)

编程：小练习题：

[Small exercises to get you used to reading and writing Rust code!](part1-exercises-for-rust/rustlings/readme.md)


- quiz 1-4
- variables
- if
- function
- primitive_types
- structs
- strings
- enums
- tests
- modules
- macros
- move_semantics

>晚上去吃了甲鱼生蚝火锅x

### 事件2：阅读rCore的一些参考资料

参考资料：

[Rust语言操作系统的设计与实现,王润基本科毕设论文,2019](https://github.com/rcore-os/zCore/wiki/files/wrj-thesis.pdf)

[PPT: 尝试用RUST写教学操作系统, 2018](https://s4plus.ustc.edu.cn/_upload/article/files/57/c6/a2ce9bd84b2ab411967842a1334d/27730908-ef69-4827-98a7-8e387875b39b.pdf)

- 大致了解了一下rCore的基本架构和设计实现思路；
- 大致了解了一下rust设计操作系统的相关语言特性和能力；

笔记：[Day2_rCore.md](daily_documents/Day2_rCore.md)

### 遗留的一些问题

- 不知道现在rCore相比于2019年的这篇文章，具体做了哪些演进呢
- 对于rust的一些语法特性并没有特别清晰熟练，还需要更多的阅读和练习
- 部分笔记还没有整理

### 预期计划

- 好好睡觉，明天继续深入rust（应该可以完成 Small exercises 的部分，并看完 PL，再考虑下一步计划
- 简单了解一下 RISC-V 体系结构；
- 尝试编译一下rCore，简单翻翻源代码长啥子样

## Day 3 2020/7/3

### 事件1：rust PL next

Rust程序设计语言：

- 函数式与闭包
- 测试
- 智能指针
- 并发
- 模式

练习：做完了rustlings的部分；

>[Small exercises to get you used to reading and writing Rust code!](part1-exercises-for-rust/rustlings/readme.md)

>★ All exercises completed! ★

- （深刻感受到了如何和编译器编译器作斗争）

感觉还是比预想的要久一点的，主要是后面的一些练习（边看语法书边瞎蒙编译器结果）...不过感觉还是收获满满的

### 遗留的一些问题：

- 感觉大概了解了一下语法，接下来需要更多的练习和多部分特性更深入的理解和运用；
- 需要解决一下开发环境的问题（图形化连接qemu调试等等）
- 整理笔记和README

### 明日计划

- 开始看lab和RISC-V；
- 继续练习和解决一些自己还不是特别清晰的语法特性；
- 补学校作业。

## Day 4 2020/7/4

>算是正式开始啦（可是还有作业没补完

今天由于有别的事情在忙，所以实际完成的部分并不多。

### 事件1：rust相关：

整理了先前的一些笔记；

阅读Rust程序设计语言：

- 部分高级特性；
- unsafe rust

### 事件2：RISC-V

参考资料：

[RISC-V手册：一本开源指令集的指南 ](http://crva.io/documents/RISC-V-Reader-Chinese-v2p1.pdf)

- 第一到三章：
- 第十章：

### 事件3：搭建实验环境（未完成）

### 明日计划：

- 尝试完成lab0；
- 完成部分rust的练习相关；
- 继续学习RISC-V

## Day 5 2020/7/5

今天在写别的课的最后一点论文，没怎么整活qwq

### 事件1：把环境配好了，正在看lab0

## Day 6 2020/7/6

### 事件1：lab0完成：

（没有实际操作的话并没有挑战性啦...不过这是lab0）

尽量一个命令都不漏的详细记录，包含lab0和安装环境：

[daily_documents/Day6_lab0.md](daily_documents/Day6_lab0.md)

lab0的代码可以参考：

[lab0/README.md](lab0/README.md)

### 事件2：RISC-V：

尝试整理了 RISC-V 的笔记；

[daily_documents/Day5_riscv.md](daily_documents/Day5_riscv.md)

顺手翻了翻这本书其他部分（真简短）

### 遗留的一些问题：

- 一些参考资料还没很仔细阅读，我希望先做lab1，边做边看；
- rust还不能说很熟练（虽然看懂lab0的部分没啥问题），需要继续做题；
- 需要对rust的配置了解多一点；

### 明日计划

- 试试lab1
- 做题家x
- 把调试工具能不能弄得更顺手一点

## Day 7 2020/7/7

### 事件1：lab1

这部分参考的是 27-labdoc 分支的内容，可能并不稳定；（补充：第二天被合并到了主分支了

实验报告笔记，以及添加的练习内容：

[daily_documents/Day7_lab1.md](daily_documents/Day7_lab1.md)

另外也阅读了一下lab2的实验教程，准备明天继续看看；

lab1 :

[lab1/README.md](lab1/README.md)

大致了解了中断的实现方式，也自己尝试实现了一小部分的中断实验；

### 事件2：rust again

把`rust程序设计语言`复习了一下（以及之前跳过的部分），顺带翻了翻 `rust by example`，以及 hard way，考虑了一下要写什么:

明天开始写小习题；

### 问题

- 感觉之前看书可能有些略过并不是很仔细的地方，还需要再巩固一下；
- lab的教程相对还是有一点简略（没有更新过的版本，似乎文档的改进工作正在进行中，可能还是结合实际代码阅读的效果会好一些；还有一些练习题的部分也不是很完善，不知道后续会不会被放进考核内容里面去；
- （希望有空可以继续 xv6 的学习，能更深入地理解操作系统原理，也可相互对照；
- 感觉自己表达的能力可能并不是很好...
- lab我还是希望先阅读代码，在此基础上尝试做一些改进（可以自己尝试写一点模块，比如线段树，和最终代码相互对照？不过实验指导似乎还是教程性质的，并不是什么题目）；

### 计划；

- 整整 lab2；
- 整整 rust的中长练习实践；

## Day 8 2020/7/8

### 事件1：lab2

继续根据 42 - labdoc 分支（还未合并）的代码学习lab2：

lab2实验指导笔记：

[daily_documents\Day8_lab2.md](daily_documents/Day8_lab2.md)

大致了解了一下，打算等代码合并以后开始写线段树和伙伴系统相关；

### 事件2：rust 练习

尝试开始写了一小部分 rust 的 hardway 练习，完成了双向链表的建造；

### 问题

- 感觉对rust的智能指针等内容还是有一些不熟悉的地方；
- 大致浏览了一下基本的代码组成，但需要进行lab2的实验才会有比较深刻的理解：
- 这里是从数据结构（双向链表、树等等开始写的，感觉对 rust 的智能指针、内部可变性的一些主题之前理解还是不够深刻，看书也不够仔细，还需要再补一补；

### 计划

- 继续今天的两个任务；

## Day 9 2020/7/9

### 事件1：lab2 next

今日 lab2 的分支已经合并进去了，把代码拉下来阅读一下；

- 查阅了一些线段树和伙伴算法相关的资料，阅读了一下代码，准备明天开始写；

### 事件2：rust 练习

目录：

[part1-exercises-for-rust\hardways\README.md](part1-exercises-for-rust/hardways/README.md)

用 rust 完成 `笨办法学c语言` 中的排序算法，二叉树、堆栈等；

## Day 10 2020/7/10

### 事件1：rust 练习

这两天在忙学校短学期的一点点事情...进展有点慢qwq

感觉 `笨办法学c语言` 的简单题目似乎还不如去写leetcode；

完成了数道 rust 的 leetcode 习题，附加代码使用c语言编写；应该明天就可以结束rust的小练习部分

## Day 11 2020/7/11

### 事件1：rust练习

完成了15道 rust 练习题的部分：

具体记录可参考：

[part1-exercises-for-rust\hardways\README.md](part1-exercises-for-rust/hardways/README.md)

如果之后有做其他的小练习，比如leetcode这样的，也会记录过来的啦

### 在尝试完成lab2的过程中

### 问题

- 练习题做完15道之后确实对 rust 的理解更上一层楼了，也能尝试用rust实现一些算法，但是感觉还是挺浅薄的...可能还是需要一些工程上的练习

## Day 12 2020/7/12

今天短学期有不少事，所以没怎么整

## 事件1 第一次分享会

- 感觉真是大佬云集...不胜惶恐...；
- 感觉要加紧做了；
- os lab的部分暂时感觉还并没有很深入地探讨过，可能打算先跟着教程实现一遍代码；

## Day 13 2020/7/13

- 今天短学期ddl，所以做的也并没有很多
- 就这两天比较忙...接下来应该可以专心做os的事情了...

## Day 14 2020/7/14

- 今天也好忙qwq
- 看完了tutorial剩下的几章，大致对整个实验有个初步的了解

## Day 15 2020/7/15

- 今天也好忙qwq

### 事件1：lab2

- 仔细研读了一下lab2的代码，和实验教程对应起来；
- 自己尝试参考代码和教程手写实现了部分 lab2 的内容；

## Day 16 2020/7/16

- 今天也好颓qwq

### 事件1：lab2

- 使用线段树进行内存分配可以参考 [https://github.com/mufeng964497595/seg_tree_memory_management](https://github.com/mufeng964497595/seg_tree_memory_management)
- 写了 lab2 线段树的代码（单个页面分配）,但实际上在这种情况下并没有多大意义，时间和空间复杂度都更大（除了最坏情况下的空间复杂度）；

## Day 17 2020/7/17

- 笙笙啊笙笙 - 你不能再这样下去了qwq
- 今天好颓，不想干活（前几天在写短学期的MC项目）

### 事件1：lab2

- 更新了一部分lab2的内容；
- 完成了页面分配算法的测试工作；
- 基本完成 lab2（参照代码示例

## Day 18 2020/7/18

- 我好惨，今天生病了

## Day 19 2020/7/19

### 事件1 - lab学习笔记 & 代码整理

整理完成 lab3 学习笔记和代码

[lab3 学习笔记](daily_documents/Day17_lab3.md)

代码部分可参考 lab3 文件夹

开始整理部分 lab4 

### 问题

- lab3 的实验框架还没有整理完成，因此打算延迟一小段时间再开始完成实验题；

## Day 20 2020/7/20

### 事件1 - lab tutorial 学习笔记 & 代码整理

修改了一些 lab3 的内容

基本整理完成 lab4 学习笔记和代码；

[lab4 学习笔记](daily_documents/Day19_lab4.md)

还留有一些 bug 还没有解决

### 事件2 - lab 实验1

回过头看了一下伙伴系统的实现方式；如果采用链表实现的话，可能也会涉及动态分配内存的方式（或者采用数组分配的链表）；因此可能采用数组方式进行实现。

### 问题

- tutorial中存在一些定义不兼容；
- 代码版本似乎比较混乱；现有根据 toturial 实现的部分并不能编译运行；
- 尝试根据lab4分支中进行观察和调整，但lab4分支中的代码实际上包含完成 lab6 用户进程之后的部分；根据教程一步步手写完成的版本则不应当包含 lab5 和 lab6 的内容（但实际上后期耦合和修改的部分很多
- （ lab4好像是尝试根绝 toturial 复现代码过程中第一个遇到困难的

### 明日计划

- 解决今天lab4遗留的一些问题；
- 做lab5
- lab4实验题

## Day 21 2020/7/21

### 事件1 - lab4 根据tutorial进行的代码整理

- 更新了一下 lab4 ，修复了一下昨天没法编译的问题（代码有很多不兼容的地方)
- 现在应该基本没问题了（尝试跑起来了多线程），虽然线程终止的时候似乎还是有一点问题没有解决；

### 事件2 - lab4 实验题

<del>打算解决掉 lab5 再回过头来看实验题；</del>

这里先做完了lab4的实验part1：

### 事件3 - lab5 tutorial 学习笔记 & 代码整理



### 遗留的一些问题

- 对于简单的main函数进行fork不会出现问题，对于notebook程序可能会发生错误；（有可能是因为此时该线程被阻塞，在等待中断的过程中fork：

```rs
Thread {
    thread_id: 0x3,
    stack: Range {
        start: VirtualAddress(
            0x1080000,
        ),
        end: VirtualAddress(
            0x1100000,
        ),
    },
    context: None,
} terminated: unimplemented interrupt type: Exception(InstructionPageFault)
cause: Exception(InstructionPageFault), stval: 0
```

- 调试工具掌握还不熟练（不过打log倒是挺熟的