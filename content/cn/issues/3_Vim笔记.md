---
title: Vim笔记
date: 2022-03-07 02:42:17
---
这里记录学习和使用 Vim/NeoVim 的笔记。

---

### 从剪贴板粘贴到 neovim 时中文乱码（Mac）

```shell
$ locale
LANG="en_US"
LC_COLLATE="en_US.UTF-8"
LC_CTYPE="en_US.UTF-8"
LC_MESSAGES="en_US.UTF-8"
LC_MONETARY="en_US.UTF-8"
LC_NUMERIC="en_US.UTF-8"
LC_TIME="en_US.UTF-8"
LC_ALL="en_US.UTF-8"
```
新建`~/.profile`文件，内容如下：
```shell
export LANG=en_US.UTF-8
```

```diff
$ source ~/.profile
$ locale
-LANG="en_US"
+LANG="en_US.UTF-8"
 LC_COLLATE="en_US.UTF-8"
 LC_CTYPE="en_US.UTF-8"
 LC_MESSAGES="en_US.UTF-8"
 LC_MONETARY="en_US.UTF-8"
 LC_NUMERIC="en_US.UTF-8"
 LC_TIME="en_US.UTF-8"
 LC_ALL="en_US.UTF-8"
```

---

Use VIM mode within leetcode editor

- Exit insert mode: `CTRL + C`