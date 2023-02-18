---
title: VIM/NeoVIM配置
date: 2022-04-17
---
等了很久的 DevTerm 终于到了。主要是想用它来当写作工具。说实话，用它打字并不令人愉快。至少需要一个适应阶段。它让我想起很多年前在电视里看过一个时尚人物访谈，虽然人名早已不记得了，只记得一点：那个时尚人物会自己给自己剪头发。他说：不便也是一种时尚。我的价值观，大概就是这样一点点拼凑起来的。

东闯西撞地装好了输入法，装了 NeoVIM。装了Firge 字体、文泉驿正黑字体，看上去总算舒服了些。接下来又想折腾 VIM/NeoVIM 的配置。以下记录一下自己折腾的过程。

## 输入法
想试一下用 VIM 插件实现的输入法。为什么呢？图个安静吧。ibus 的输入框还是太跳了。本来想尝试 zhcon 在纯命令行的模式下写作，可惜不知道为什么，进入 zhcon 之后屏幕就被旋转了 90 度。暂时还没找到解决方法。如果找到，我肯定第一时间放弃图形界面。在那之前，还需忍耐一下。

### 安装 [ZFVimIM](https://github.com/ZSaberLv0/ZFVimIM)
安装 vim-plug

Vim: macOS/Linux(UNIX)：
```
curl -fLo ~/.vim/autoload/plug.vim --create-dirs \
    https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim
```

NVim:
```
sh -c 'curl -fLo "${XDG_DATA_HOME:-$HOME/.local/share}"/nvim/site/autoload/plug.vim --create-dirs \
       https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim'
```

vimrc 中添加 ZFVimIM 插件：

```
call plug#begin('~/.vim/plugged')
Plug 'ZSaberLv0/ZFVimIM'
Plug 'ZSaberLv0/ZFVimJob'
call plug#end()
```
### TODO
- [ ] 
