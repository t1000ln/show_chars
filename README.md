# Show Chars
这是一个辅助`fltk`开发过程的手册应用。

该手册提供常用字符及颜色查看功能。
- `ASCII`码表。
- `Emoji`字符表。
- `Unicode`字符码值查询。
- `Fltk-color`颜色表。
- `ANSI-color`颜色表。
- `HTML-color`颜色表。

在应用界面上鼠标点击目标节点，即可复制对应的码值到系统剪贴板。

# 安装方法
使用`cargo`命令安装：
```bash
$ cargo install show_chars
```
在`linux`桌面环境，该应用依赖`Emoji`字体`NotoColorEmoji.ttf`才能正常显示`Emoji`符号，下载地址：`https://github.com/googlefonts/noto-emoji/tree/main/fonts`。
请自行安装字体。


# 已知问题
- 在`Windows`环境下，无法显示`Emoji`字符颜色，这是`FLTK`组件的缺陷，暂时无法解决。
