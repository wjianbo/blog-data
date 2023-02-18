---
title: Linux笔记
date: 2022-12-20 23:57:00
---
安装中日文输入法（Steam Deck 桌面模式）

1. 通过 Discover - Software Center 安装 ：

    - Fcitx 5
    - Mozc for Fcitx 5
    - Rime for Fcitx 5

2. 通过 Fcitx Configuration 添加 Rime 和 Mozc。

3. 安装配置管理工具[東風破](https://github.com/rime/plum) /plum/ 

```
curl -fsSL https://raw.githubusercontent.com/rime/plum/master/rime-install | bash
```

4. 用東風破安裝口令安装所需的输入方案（需要在 plum 目录下执行）

```
bash rime-install wubi
```

5. 修改配置文件`~/.local/share/fcitx5/rime/`

```
schema_list:
  - schema: wubi86
  - schema: luna_pinyin_simp
```

配置文件的位置可能不同，参考[这里](https://github.com/rime/home/wiki/UserData)。

6. 右键点击输入法图标，选择 Deploy 使配置生效。


---

Get rid of read only filesystem folders (like /usr) on Steam Deck

```bash
sudo steamos-readonly disable
```

enable it again after you're done with your changes:

```shell
sudo steamos-readonly enable
```

---

- [setup L2TP IPsec VPN in archlinux](https://gist.github.com/wjianbo/e073893d3fd701332a425bce97c8830f)