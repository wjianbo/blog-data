---
title: Rust学习笔记
date: 2022-08-09 09:43:56
---
### TODO
- [ ] 《Rust编程之道》
- [ ] [Learn Rust Programming Course – Interactive Rust Language Tutorial on Replit](https://www.freecodecamp.org/news/rust-in-replit/)
- [ ] [The Rust Programming Language](https://www.udemy.com/course/rust-lang/)

---

### 《Rust编程之道》
- Rust中没有传统面向对象语言中的继承概念。Rust通过trait将类型和行为明确区分，贯彻了**组合优于继承和面向接口编程**的编程思想。
- 返回值为类型为“!”的发散函数（diverging function）永远不会有任何返回值。
- 高阶函数是指以函数作为参数或返回值的函数。

### 問題点

- 访问者模式

---

## Web framework:  Actix
[hello-world](https://actix.rs/docs/getting-started)

---

- **Rc** stands for reference counting, and it is used in Rust to makes possibles a single values has multiples owners. 
- **Arc** stands for Atomic Reference Counting, it is like the Rc, but with the possibility to work with threads.[^1]

[^1]: [Box, Rc, Arc, and the relation with threads.](https://henrybarreto.dev/box-rc-arc-and-the-relation-with-threads)
