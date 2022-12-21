---
title: WezTerm笔记
date: 2022-04-08 07:13:58
---
解决中日文乱码问题（Mac）

在 `~/.zshrc` 文件中添加
```shell
export LANG=en_US.UTF-8
```
然后执行 `source ~/.zshrc` 或重启一下 WezTerm 应该就可以了。

---

### ShortCuts

- [Copy Mode](https://wezfurlong.org/wezterm/copymode.html)
  - Enter copy mode: `CTRL-SHIFT-X`
  - Toggle selection mode: `v`
  - Copy: `CTRl-SHIFT-C`
  - Exit copy mode: `Esc`