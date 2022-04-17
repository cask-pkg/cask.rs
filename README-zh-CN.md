[English](README.md) | 中文简体

<div align="center">
   <img src="logo.svg" with="64" height="64"/>

   <h1>Cask</h1>

[![installation](https://github.com/cask-pkg/cask.rs/actions/workflows/installation.yml/badge.svg)](https://github.com/cask-pkg/cask.rs/actions/workflows/installation.yml)
[![lint](https://github.com/cask-pkg/cask.rs/actions/workflows/lint.yml/badge.svg)](https://github.com/cask-pkg/cask.rs/actions/workflows/lint.yml)
[![test](https://github.com/cask-pkg/cask.rs/actions/workflows/test.yml/badge.svg)](https://github.com/cask-pkg/cask.rs/actions/workflows/test.yml)
[![build](https://github.com/cask-pkg/cask.rs/actions/workflows/build.yml/badge.svg)](https://github.com/cask-pkg/cask.rs/actions/workflows/build.yml)
![Latest Version](https://img.shields.io/github/v/release/cask-pkg/cask.rs.svg)
![License](https://img.shields.io/github/license/cask-pkg/cask.rs.svg)
![Repo Size](https://img.shields.io/github/repo-size/cask-pkg/cask.rs.svg)

一个通用的，二进制包管理器。

[安装](#安装) |
[使用](#使用) |
[如何发布包?](DESIGN.md#how-do-i-publish-package) |
[设计](DESIGN.md) |
[贡献](CONTRIBUTING.md) |
[Cask.toml](Cask.toml.md)

</div>

如果你已经疲于:

1. 在不同的平台安装不同的包管理器 (Homebrew/Chocolatey/Scoop).
2. 写很多次安装脚本(Bash/PowerShell).
3. 发布新版本是更新远端包信息

那么欢迎来到 Cask。

## 安装

1. Shell (Mac/Linux)

   ```bash
   curl -fsSL https://github.com/cask-pkg/cask.rs/raw/main/install.sh | bash
   ```

2. PowerShell (Windows):

   ```pwshell
   iwr https://github.com/cask-pkg/cask.rs/raw/main/install.ps1 -useb | iex
   ```

3. [Github release page](https://github.com/cask-pkg/cask.rs/releases)

   下载可执行文件，然后放到 `$PATH` 目录下

4. 从已有的版本更新

   ```bash
   cask self-update
   ```

## 使用

```sh
# cask install <package>
cask install github.com/axetroy/dvm
# cask install <package> <version>
cask install github.com/axetroy/dvm 1.x
# cask install <repository URL>
cask install https://github.com/axetroy/dvm.git
# cask install from stdin
curl https://raw.githubusercontent.com/axetroy/dvm/master/Cask.toml | cask install

# cask uninstall <package or the executable file name of the package>
cask uninstall github.com/axetroy/dvm
# or uninstall with shorter command
cask rm dvm
```

运行 `cask --help` 以查看更多信息.

## 命令

| Command                            | Description            |
| ---------------------------------- | ---------------------- |
| cask install \<PACKAGE\> [VERSION] | 安装包                 |
| cask uninstall \<PACKAGE\>         | 卸载包                 |
| cask info \<PACKAGE\>              | 显示包信息             |
| cask update \<PACKAGE\>            | 更新包到最新版本       |
| cask check-updates                 | 检查并更新包到最新版本 |
| cask list                          | 列出已安装的包         |
| cask clean                         | 清除缓存数据           |
| cask self-update                   | 升级 Cask              |
| cask remote sync                   | 同步远端的内置包信息   |
| cask remote list                   | 显示远端的内置包信息   |

## 使用条件

Cask 依赖于 [Git](https://git-scm.com)

## 贡献者

这个项目的存在离不开你们的贡献。 [如何贡献](CONTRIBUTING.md).

<a href="https://github.com/cask-pkg/cask.rs/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=cask-pkg/cask.rs" />
</a>

## 开源许可

The [MIT License](LICENSE)