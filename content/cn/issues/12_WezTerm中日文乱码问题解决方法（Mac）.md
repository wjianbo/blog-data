---
title: WezTerm中日文乱码问题解决方法（Mac）
date: 2022-04-08 07:13:58
---
在 `~/.zshrc` 文件中添加一行：
```shell
export LANG=en_US.UTF-8
```
然后执行 `source ~/.zshrc` 或重启一下 WezTerm 应该就可以了。