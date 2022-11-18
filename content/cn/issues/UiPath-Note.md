---
title: UiPath 学习笔记
date: 2020-01-28 20:13:01
tags: [UiPath]
---
完成了 Level 3 最后一个任务。

尝试把项目发布到 Ochestrator，可以正常运行。

以下是遇到的一些问题及解决方法：

**一、开始 Job 时报错**

解决方法：更新 Robot 说明中的密码

参考：[Job failed when trying to run manually - RPA Developers - UiPath Community Forum](https://forum.uipath.com/t/job-failed-when-trying-to-run-manually/152883)

**二、Type Into (Simulate Type) 触发输入法**

解决方法：勾选 TypeInto 的 sendwindowsMessage 选项

参考：[【UiPath 问题 20 】如何解决输入法问题？ - 简书](https://www.jianshu.com/p/800e58d9bf0f)

**三、另存文件时输入路径 + 文件名并勾选「Simulate Type」导致另存失败（以原文件名保存在当前路径）**

解决方法：取消勾选「Simulate Type」

**四、On element appear 超时报错**

解决方法：RepeatForever 设为 false

**五、提取字符串中介于两个指定 index 之间的字符**

```
stringObject.substring(start,stop)
```

**六、选中下拉菜单/下拉列表中文字的方法**

第一种：Click + Click （F2 开始 3 秒倒计时）

第二种：用「Select item」Activity 实现

[UiPath认证L3(Level3)详解Calculate Client Security Hash—2 - 简书](https://www.jianshu.com/p/b708029603d9)

**七、在 Selector 中使用变量的方法**

只能在属性面板中进行修改

[UiPath Level 1-Lesson 6. Selectors - 韭菜也是有尊严的 - 博客园](https://www.cnblogs.com/TeresaMu/p/9871718.html)
