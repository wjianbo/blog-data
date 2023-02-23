---
title: 用 Obsidian 写博客
date: 2023-02-23
---

去年一直在[[用 GitHub Issues 写博客]]，今年迷上了 Obsidian，经过新一轮的折腾，终于可以用它管理博客内容了。

和用 Issues 相比，ob 的好处是更方便在文章之间建立链接，来回跳转。另外，通过 Graph view 可以很直观地看到文章之间的联系，就像下面这张截图。每一个点都代表一篇文章。现在它就像一盘散沙。希望我可以慢慢把这些点连起来。就像乔布斯说的，**Connect the dots**。


![Graph view](/Extras/Images/IMG_0217.png)

另外，我给博客添加了 [hugo-wikilinks](https://github.com/milafrerichs/hugo-wikilinks) 支持。可以使用跟 Ob 内部链接同样的语法建立博客文章之间的链接。按照 markdown 的语法，链接要写成 `[text](file path)`，而 Ob 的内部链接语法则可以直接写成 `[[file name]]`。不需要指定具体路径，简洁明快。

用 Ob 管理博客的方法很简单。因为我的博客用 GitHub Actions 实现了自动部署，所以只要想办法把我 Ob vault 里的 Blog 文件夹同步到  blog-data 仓库的 content 目录即可。

我在 GitHub 上新建了一个用来备份和同步 Ob 笔记的仓库，然后在这个仓库上添加了一个 Action：

```yml
name: Push Blog folder

on:
  push:
    paths:
      - 'Blog/**'

jobs:
  copy-file:
    runs-on: ubuntu-latest
    steps:
    - name: Checkout
      uses: actions/checkout@v2

    - name: Pushes blog folder
      uses: datalbry/copy_folder_to_another_repo_action@1.0.0
      env:
        API_TOKEN_GITHUB: ${{ secrets.API_TOKEN_GITHUB }}
      with:
        source_folder: 'Blog'
        destination_repo: 'wjianbo/blog-data'
        destination_branch: 'main'
        destination_folder: 'content'
        user_email: 'jnb.wang@gmail.com'
        user_name: 'wjianbo'
        commit_msg: '[GHA] Update blog contents.'
```

这样只要我把写好的文章放到 Blog 文件夹下，然后通过 Obsidian Git 插件推送到 GitHub，稍等一会儿，就可以在在博客页面上看到新发布的内容了。

