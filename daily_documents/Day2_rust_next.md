# rust入门笔记：继续深入学习rust相关语法

github地址：[https://github.com/yunwei37/os-summer-of-code-daily](https://github.com/yunwei37/os-summer-of-code-daily)

主要参考资料：[Rust 程序设计语言](https://kaisery.github.io/trpl-zh-cn/)

## 常见编程概念

- 关键字（keywords）

### 变量和可变性

变量默认是不可改变的（immutable）。这是推动你以充分利用 Rust 提供的安全性和简单并发性来编写代码的众多方式之一。

- 不能对不可变变量 x 二次赋值
- Rust 编译器保证，如果声明一个值不会变，它就真的不会变。
- 不允许对常量使用 mut。常量不光默认不能变，它总是不能变。
- 声明常量使用 const 关键字而不是 let，必须 注明值的类型
- 常量只能被设置为常量表达式，而不能是函数调用的结果，或任何其他只能在运行时计算出的值。
- 而新变量会 隐藏 之前的变量

### 数据类型

- 数据类型（data type）
- Rust 是 静态类型（statically typed）语言
- 标量（scalar）类型代表一个单独的值。Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。
- 整数 是一个没有小数部分的数字。
- Octal (八进制) 	0o77
- 在 debug 模式编译时，“整型溢出”（“integer overflow” ）；在 release 构建中，Rust 不检测溢出，相反会进行一种被称为二进制补码包装（two’s complement wrapping）的操作。
- Rust 的 char 类型的大小为四个字节(four bytes)，并代表了一个 Unicode 标量值
- 复合类型（Compound types）可以将多个值组合成一个类型。Rust 有两个原生的复合类型：元组（tuple）和数组（array）。
- 元组类型：元组是一个将多个其他类型的值组合进一个复合类型的主要方式。
- 模式匹配（pattern matching）来解构（destructure）
- 使用点号（.）后跟值的索引来直接访问它们。
- 与元组不同，数组中的每个元素的类型必须相同。

## 函数 

- 源码中 another_function 定义在 main 函数 之后；也可以定义在之前。Rust 不关心函数定义于何处，只要定义了就行。
- 语句（Statements）是执行一些操作但不返回值的指令。表达式（Expressions）计算并产生一个值。
- 我们并不对返回值命名，但要在箭头（->）后声明它的类型。

## 所有权（系统）

## 结构体