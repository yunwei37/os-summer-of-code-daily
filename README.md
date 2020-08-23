# Daily Schedule for OS Tutorial Summer of Code 2020

- daily_documents：每日笔记
- part1-exercises-for-rust：rust的一些小练习
- practice：rust写的一些小程序
- labs：rCore的lab

可供检查的具体实现目录：

- [rCore labs](labs)
- [rust practices](part1-exercises-for-rust)

## TOC

*七月*

| Mon               | Tues              | Wed                          | Thur                         | Fri                          | Sat               | Sun               |
| ----------------- | ----------------- | ---------------------------- | ---------------------------- | ---------------------------- | ----------------- | ----------------- |
|                   |                   | 1 <br> ([D1](#day-1-202071)) | 2 <br> ([D2](#day-2-202072)) | 3 <br> ([D3](#day-3-202073)) | 4 <br> ([D4](#day-4-202074)) | 5 <br> ([D5](#day-5-202075)) |
| 6 <br> ([D6](#day-6-202076)) | 7 <br> ([D7](#day-7-202077)) | 8 <br> ([D8](#day-8-202078))            | 9 <br> ([D9](#day-9-202079))            | 10 <br> ([D10](#day-10-2020710))         | 11  <br>  ([D11](#day-11-2020711))             | 12      <br>    ([D12](#day-12-2020712))       |
| 13    <br>    ([D13](#day-13-2020713))             | 14         <br>    ([D14](#day-14-2020711))        | 15        <br>    ([D15](#day-15-2020715))                    | 16    <br>     ([D16](#day-16-2020716))                       | 17    <br>      ([D17](#day-17-2020717))                       | 18    <br>    ([D18](#day-18-2020718))            | 19   <br>     ([D19](#day-19-2020719))            |
| 20   <br>    ([D20](#day-20-2020720))            | 21       <br>    ([D21](#day-21-2020721))         | 22     <br>    ([D22](#day-22-2020722))                         | 23     <br>    ([D23](#day-23-2020723))                         | 24    <br>    ([D24](#day-24-2020724))                        | 25      <br>    ([D25](#day-25-2020725))             | 26         <br>    ([D26](#day-26-2020726))           |
| 27         <br>    ([D27](#day-27-2020727))           | 28       <br>    ([D28](#day-28-2020728))           | 29         <br>    ([D29](#day-29-2020729))                    | 30        <br>    ([D30](#day-30-2020730))                     |      <br>    ([D31](#day-31-2020731))                           |                   |                   |

*八月*

| Mon               | Tues              | Wed                          | Thur                         | Fri                          | Sat               | Sun               |
| ----------------- | ----------------- | ---------------------------- | ---------------------------- | ---------------------------- | ----------------- | ----------------- |
|                   |                   | 1 <br> ([D32](#day-1-202081)) | 2 <br> ([D33](#day-2-202082)) | 3 <br> ([D34](#day-3-202083)) | 4 <br> ([D35](#day-35-202084)) | 5 <br> ([D36](#day-36-202085)) |
| 6 <br> ([D37](#day-37-202086)) | 7 <br> ([D38](#day-38-202087)) | 8 <br> ([D8](#day-8-202078))            | 9 <br> ([D9](#day-9-202079))            | 10 <br> ([D10](#day-10-2020710))         | 11  <br>  ([D11](#day-11-2020711))             | 12      <br>    ([D12](#day-12-2020712))       |
| 13    <br>    ([D13](#day-13-2020713))             | 14         <br>    ([D14](#day-14-2020711))        | 15        <br>    ([D15](#day-15-2020715))                    | 16    <br>     ([D16](#day-16-2020716))                       | 17    <br>      ([D17](#day-17-2020717))                       | 18    <br>    ([D18](#day-18-2020718))            | 19   <br>     ([D19](#day-19-2020719))            |
| 20   <br>    ([D20](#day-20-2020720))            | 21       <br>    ([D21](#day-21-2020721))         | 22     <br>    ([D22](#day-22-2020722))                         | 23     <br>    ([D23](#day-23-2020723))                         | 24    <br>    ([D24](#day-24-2020724))                        | 25      <br>    ([D25](#day-25-2020725))             | 26         <br>    ([D26](#day-26-2020726))           |
| 27         <br>    ([D27](#day-27-2020727))           | 28                | 29                           | 30                           |                              |                   |                   |


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

完成了一部分；

### 遗留的一些问题

- 对于简单的main函数进行fork不会出现问题，对于notebook程序可能会发生错误；（有可能是因为此时该线程被阻塞，在等待中断的过程中fork，这里复制的是内核线程：

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

## Day 22 2020/7/22

### 事件1 - lab tutorial 学习笔记 & 代码整理

完成了 lab5 的笔记和代码整理;

[daily_documents\Day21_lab5.md](daily_documents/Day21_lab5.md)

基本完成了 lab6 的笔记和代码整理;

[daily_documents\Day22_lab6.md](daily_documents/Day22_lab6.md)

### 总结

- 感觉 lab4-6 和前期相比省略的内容还是比较多，可能还是有不少不完善的地方吧...
- 对于整个 lab 实验的代码和实现基本过了一遍，接下来打算做完实验题看看有没有什么自己想改进的地方；
- 感觉了解的操作系统具体实现还是比较少，可以先看看其他的书和rcore的实现；
- 还有一些细节问题和思考不全面的地方需要回头再整理清楚一下；

## Day 23 2020/7/23

### 事件1 - lab4 实验题

完成了 lab4 实验题 part2 部分

[daily_documents\Day21_lab4_practice.md](daily_documents/Day21_lab4_practice.md)

## Day 24 2020/7/24

### 事件1 - lab6 实验题

完成了 lab6 的实验题：

[daily_documents\Day23_lab6_practice.md](daily_documents/Day23_lab6_practice.md)

顺带关于 lab5-6 的小细节发了一点pr

### 事件2 - 总结、提交部分

整理了一下最终的报告文档；目前基本上 lab 的部分也已经接近尾声了，从头到尾自己根据文档实现了一遍，也完成了大部分实验题的内容（除了一点挑战实验和框架还没打好的页面置换算法）

## Day 25 2020/7/25

### 事件1：blog总结

[daily_documents\云微的-rCore-阶段性总结报告.md](daily_documents/云微的-rCore-阶段性总结报告.md)

### 事件2：更进一步的探索

尝试阅读了一下 rCore 和 zCore 的源代码；

学堂在线《Linux 内核分析与应用》课程：概述、中断、进程

感觉自己具体操作系统工程实践方面还是了解的不够多，与其去测试一些新增加的算法，不如先学一点别的东西，可以互相对比照应；

### 问题

- [https://github.com/rcore-os/zCore/issues/75](https://github.com/rcore-os/zCore/issues/75) 遇到了和这个issue一样的bug

## Day 26 2020/7/26

### 事件1：lab3 实现时钟页面置换算法

看起来实验框架终于整理好了，把这个剩下的也完成一下；

参考这里：

[daily_documents\Day18_lab3_practice.md](daily_documents/Day18_lab3_practice.md)

另外也大致了解了一下 rCore 和 zCore 的整个架构；

## Day 27 2020/7/27

### 事件1：lab3 的一点小修复

好像有一点问题...（被公开处刑了）...要去修复一下

之前实现的好像并不是标准的时钟页面置换算法，有一点偏差

### 事件2：继续学习

- 今天忙了一点小学期的收尾工作，所以没做太多
- 继续看《Linux 内核分析与应用》；

## Day 28 2020/7/28

是摸鱼的一天呢

中啦

## Day 29 2020/7/29

[深入分析Linux内核源代码阅读笔记](daily_documents/深入分析Linux内核源代码阅读笔记.md) 第一章、第二章

## Day 30 2020/7/30

[深入分析Linux内核源代码阅读笔记](daily_documents/深入分析Linux内核源代码阅读笔记.md) 第三章、第四章、第五章

《Linux 内核分析与应用》系统调用

## Day 31 2020/7/31

- 继续看了部分《rust编程之道的章节》
- 《Linux 内核分析与应用》
- 尝试运行 rCore 和 zCore，浏览了一下相关文档；

## Day 32 2020/8/1

## Day 33 2020/8/2

- 准备出发
- 在火车上浏览了部分 zCore-Tutorial 的教程；
- 看了看 zCore 的系统调用部分；

## Day 34 2020/8/3

### 参考相关资料

今天是 Hackthon 的第一天

- [2020操作系统课程设计：zCore的增强报告](http://os.cs.tsinghua.edu.cn/oscourse/OS2020spring/projects/g08?action=AttachFile&do=view&target=report.pdf)
- [A development of zCore sycall](https://github.com/GCYYfun/DailySchedule/blob/master/doc/blog/A_development_of_zCore_sycall.md)
- [zircon-notes](https://github.com/PanQL/zircon-notes)
- [fuchsia-docs-zh-CN](https://github.com/zhangpf/fuchsia-docs-zh-CN/issues)

大致浏览了一遍相关资料（虽然很大一部分没看懂），目前对整个 zCore 的大致框架已经有了一个概念

> 晚上去吃了好吃的

## Day 35 2020/8/4

- 继续浏览代码，准备开始干活；
- 不知道为什么头很晕...可能强度有点大


## Day 36 2020/8/5

### 事件1：linux-syscall 的注释文档完善

相关 pull-request: [https://github.com/rcore-os/zCore/pull/125](https://github.com/rcore-os/zCore/pull/125)

这一部分主要参考linux的文档：system reference manuals section 2: System calls 

另外也尝试简单地修复了一下 `uname` 的系统调用，似乎要把 `rCore` 换成 `Linux` 才可以；

### 一些疑问 & TODO

- 还有很大一部分 `syscall` 并没有完成移植，这部分可能是接下来的主要目标；希望能跑起来 `gcc` 作为这一个月的目标；
- 需要进一步完善相关单元测试，可以以单元测试驱动开发；
- 尝试补全 Linux-loader 和 Linux-object 的代码；
- 目前只是实现了 `linux syscall` 的一个小子集，很多部分的代码也并不完善；接下来打算先整理一个 `TODO` 的列表出来：
  -  GCC 需要使用哪些系统调用；
  -  哪些系统调用可以用空调用函数蒙混过关；
  -  哪些系统调用需要继续完善；
  -  哪些系统调用可以很方便地移植；
  -  哪些系统调用很复杂但是一定需要实现（实现一个子集）；
  -  哪些系统调用有 bug
- 在跑起来 GCC 和 make 之后，可以尝试 libc-test

## Day 37 2020/8/6

- 之前的 pull-request 被合并啦

### 完善了一点 linux-loader 的注释和测试；

相关 pull-request: [https://github.com/rcore-os/zCore/pull/128](https://github.com/rcore-os/zCore/pull/128)

- 系统调用的错误处理似乎并不是特别完善；感觉可以再想想会不会有更好的错误处理方案；
- 感觉 pipe 的 syscall 似乎并不是很复杂，也许可以尝试一下；

## Day 38 2020/8/7

### 事件1：代码改进

- 搭建了相关的单元测试框架，但还没有得到合并；
  
  大概是这样的：

  Currently you can add c language unit test like this:

  1. add c files in `linux-syscall/test/`;
  2. add a cargo unit test in `linux-loader` like ( the program name is the same as source file )
    ```
    #[async_std::test]
    async fn test_pipe() {
        test("/bin/testpipe1").await;
    }
    ```
  - if you want to run the unit test locally right away, you can run `make rootfs` again.
  - if you want to inspect the output of program, maybe output the content to a file and read it in rust can be helpful.  

- 完善了 `pipe` 的 syscall，并增加了相关测试；
- 具体内容可以参考 pull-request；

### 接下来的大概计划

- 对于 rCore 异步相关的代码有更多的了解，向勇老师提及的将操作系统内部的 syscall 等机制全部替换为 async 的异步协程确实是个非常有趣的话题，值得进一步探究
- 对于 zCore 来说，我们需要用这种机制完成信号量等内容；
- 测试覆盖和文档内容还需要继续增加，同时通过测试来发现 bug ；

> 这几天的内容结束啦
> 
> 这是我第一次参加这样的活动，感觉认识了非常多厉害的朋友，收获也非常丰富呢

## Day 39 2020/8/8

- 尝试跑起来了 libc-test
- 加了点 time 的系统调用

## Day 40 2020/8/9

- libc-test 似乎还是有一点编译上面的问题？某些测试可以运行但是并不能正常测试，可能是并没有正确调用系统调用函数；math部分可以正常运行；也可能是编译环境的问题
- 另外补上了 sys_utimensat，现在可以正常使用 touch 创建文件；

## Day 42 2020/8/11

- 修改了一点之前犯蠢的测试代码，现在可以使用c语言进行正常的单元测试；
- pull-request: [#135](https://github.com/rcore-os/zCore/pull/135) ,包含time和touch相关的被合并啦

## Day 43 2020/8/12

- 一些文档和杂七杂八的系统调用接口以及测试的打杂的活,具体参考这里：[#142](https://github.com/rcore-os/zCore/pull/142) 
- 已经被合并

## Day 44 2020/8/13

- 目前的状态是 rustc 可以运行输出帮助信息，但还暂时没有办法进行正常编译；通过查日志可以发现可能是 pipe 之后跟着的 poll 的问题，显示有事件阻塞；
- ly 大佬已经把 eventbus 修好了，之后去加一下 pipe 的 eventbus 相关内容；
- 准备去修复一下 rcore-fs 里面 symbol link 相关， 可能要实现 create2 函数；
- ly 大佬编译好了 gcc 并成功在上面跑起来啦！
- libc-test 目前的想法是写个脚本先试着运行一下并查看输出；
- 这两天也有些时候在解决测量学实验和计算机组成课程的内容，以及其他杂七杂八的考试，所以时间有可能会稍微不那么多一点...

## Day 45 2020/8/14

- 着手修复 pipe 的相关问题，加上了 async-poll 但还是不能正常使用；
- 可以考虑把libc-test在zcore上面编译运行

## Day 46 2020/8/15

- 例会；
- wrj学长解决了 libc-test 的 github ci 部署的事情；
- 提交了一个相关的pr，解决了一下 shell 里面命令执行 path 的问题：[#149](https://github.com/rcore-os/zCore/pull/149) ，顺带加了一下pipe

## Day 47 2020/8/16

- 今天去试着跑了一下 libc-test：
  - [#152](https://github.com/rcore-os/zCore/pull/152) 解决了一个小测试，顺带补了一点文档；
  - 似乎很多没办法通过的测试还是和 symlink 的绝对路径相关；
  - 浮点数运算差异应该可以不用管
  - 部分是系统调用还没有实现或者实现有部分差异；
- 感觉接下来可以做一些 ipc 共享内存相关的部分？

## Day 48 2020/8/17

- 共享内存部分需要虚拟内存的支持，实现差异可能有点大；
- 先从信号量部分入手，相关 PR：
- [#156](https://github.com/rcore-os/zCore/pull/156)

## Day 49 2020/8/18

- 完善了一下 #156 的文档，并进行了一定的重构修改和相关测试
- #156 已合并到主分支

## Day 50 2020/8/19

今天是摸鱼的一天呢...

- 主要是学校计算机组成的实验要验收了，细节需要补一补
- 还有一门短学期的GIS课也要验收了
- 接下来九月初还有五门考试要考，所以可能时间不那么多了...
- 再回过头来翻了翻 rust 编程之道，主要是之前忽略掉的一些小细节...

## Day 51 2020/8/20

- 今天是看看代码的一天
- 大概看了看相关的内容，似乎大块的可以迁移部分已经基本完成，或者就是需要硬件层的支持（比如网络部分）；
- 接下来大概可以尝试的有三个小部分：
  - 共享内存：需要虚拟内存支持，rcore中间是使用类似 toturial 的做法，zcore 中间是面向对象的包装方式；
  - epoll等 io多路复用，尝试了一下，也涉及到一些文件相关的概念转换；
  - 内存锁机制的完善，目前我们只有信号量相关和eventbus，同时mutex也并不是原子性的，可能需要完善一下关中断的自旋锁；

- 另外好像 libc-test 的测试又有点奇怪的问题了？
- 之前说的 symlink 写了一点也还没继续...

- 以及我觉得接下来也可以把这段时间的一些工作，比如文档，新增加和改善的功能，整合到rcore里面去？

## Day 53 2020/8/22

- pull request: [#160](https://github.com/rcore-os/zCore/pull/160)
- 大概写了一下关于共享内存的支持，参考mmap采用 vmObject 完成对虚拟内存段的映射（虽然理解还不是很深刻，也需要再看看具体有什么差异；
- shmctl还没有，看看能不能整个（类似sem

## Day 54 2020/8/23

- 继续完善之前的 PR，完成了shmctl系统调用，并添加了对shmget的flag处理；
- 基本可以通过 libc-test 的相关测试；

## Day 55 2020/8/24

- 去看了一下 io多路复用 的内容；
- 目前不清楚 条件变量等的使用方式是否可以被 async 代替；
- select 和 poll 的差异并不是很大；
- （这两天有点晕晕的...而且下周就要考试了也要抓紧时间复习一下了...