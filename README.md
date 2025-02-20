<!-- markdownlint-disable MD033 MD041 -->
<p align="center">
  <h3 align="center">Lazyv2ex</h3>
</p>

<p align="center">
  v2ex 命令行客户端
</p>
<!-- markdownlint-enable MD033 -->

![showcase](./assets/showcase.gif)

## 安装

### 源码构建

从源码构建需要 [Rust](https://www.rust-lang.org/)
编译器, 和 [Cargo 包管理器](https://doc.rust-lang.org/cargo/)。如果你的系统包管理器无法使用它们，则使用 [rustup](https://rustup.rs/)。

从源码构建 lazyv2ex 二进制文件，然后安装到 `$HOME/.cargo/bin/` 运行：

```sh
cargo install --locked --git https://github.com/mingeme/lazyv2ex
```

除此之外，你也可以下载源码并构建 lazyv2ex 二进制文件：

```sh
git clone https://github.com/mingeme/lazyv2ex
cd lazyv2ex
cargo install --path .
```

## 使用

在终端调用 `lazyv2ex` 命令。

```sh
$ lazyv2ex
```

如果需要，你可以使用 `echo "alias lv='lazyv2ex'" >> ~/.zshrc`（或你正在使用的任何 rc 文件）为其添加别名。

## 贡献

有任何问题和想法，欢迎提 issue 和 pr。

## 替代品

感谢以下项目作者提供的创意，本项目基于他的思路开发，旨在拓展更多的功能，另一方面也是学习之作。

[v2ex-tui](https://github.com/kaolengmian7/v2ex-tui)
