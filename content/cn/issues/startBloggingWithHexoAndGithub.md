---
title: Hexo+Github 搭建博客（Mac）
date: 2020-01-14 07:54:22
tags: [blogging]
---
## 安装

1.  安装  [Hexo](https://hexo.io/zh-cn/) 前需要先安装下列应用程序：
*  [Node.js](http://nodejs.org/)
*  [Git](http://git-scm.com/) 
2. 安装 Hexo
```
$ npm install -g hexo-cli
```

## 建站
```
$ hexo init <folder>
$ cd <folder>
```
## 配置
在 _config.yml 中修改配置。

## 更换主题
换成我比较喜欢的 [next](https://github.com/iissnan/hexo-theme-next) 主题。
1. 安装
```
$ git clone https://github.com/iissnan/hexo-theme-next themes/next
```
2. 修改配置文件（_config.yml） 
```
theme: next
```
3. 链接失效问题
问题描述：「首页」「归档」等链接失效，URL 地址最后面出现「%20」。
解决方法：去掉主题配置文件（theme/next/ _config.yml）里menu部分多余的空格，例如：
```
home: / || home
```
改为：
```
home: /|| home
```

## 预览
```
$ hexo server
```
启动服务器。默认情况下，访问网址为：http://localhost:4000/
## 部署
1. 安装  [hexo-deployer-git](https://github.com/hexojs/hexo-deployer-git) 
```
$ npm install hexo-deployer-git —save
```
2. 修改配置（_config.yml） 
```
deploy:
  type: git
  repository: https://github.com/<你的 GitHub 用户名>/<你的 GitHub 用户名>.github.io.git
  branch: master
```
3. 生成站点文件并推送至远程库。
```
$ hexo clean && hexo deploy
```

## 写作
```
$ hexo new [layout] <title>
$ hexo clean && hexo deploy
```

---
参考：
[hexo+github 搭建个人博客(超详细教程)_AinUser 的博客-CSDN 博客](https://blog.csdn.net/ainuser/article/details/77609180)
[404 问题 · Issue #3816 · hexojs/hexo · GitHub](https://github.com/hexojs/hexo/issues/3816)
