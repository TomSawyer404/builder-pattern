# Builder模式

生成器模式（Builder Pattern）又叫做构建者模式，它意图在于将一个复杂的构建与其表示想分离，使得同样的构造过程可以创建不同的表示。它可以将复杂对象的建造过程抽象出来（抽象类别），使这个抽象过程的不同实现方法可以构造出不同表现（属性）的对象。

注意：创建不同的表示 = 多态（在 Rust 中通过 `trait`实现）。

构建者模式在 Rust 中很常见，比如标准库的`std::process::Command`就是一个例子。Builder有两个重要的角色：

- builder（相当于底层的建筑工人）：利用`trait`完成多态
- director（相当于设计师）：利用`struct`完成主流程

builder 负责提供构建对象各个部分的功能以及最后组装对象的功能，而 director 负责调用 builder 的功能来构建对象。

# 实战

我们定义了一个结构体`Circle`，它有三个成员：坐标`x`、坐标`y`、半径`radius`。我们还为其定义了一个构造者`CircleBuilder`结构体。我们实现了构造者的方法，它帮助我们组装了`Circle`的各个成员。

最后我们可以在`main()`函数中以一个优雅的链式调用`Circle::new().x(1.0).y(2.0).radius(2.0).build()`来创建`Circle`实例。

# 参考资料

- [建造者模式-菜鸟教程](https://www.runoob.com/design-pattern/builder-pattern.html)
- [原子之音的视频](https://www.bilibili.com/video/BV1bQ4y1R7wx?spm_id_from=333.999.0.0)
- [Rust编程之道7.2.1节](http://product.dangdang.com/26475568.html)

---
