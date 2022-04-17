English | [中文简体](README-zh-CN.md)

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

A universal, distributed package manager.

[Installation](#Installation) |
[Usage](#Usage) |
[How to publish package?](DESIGN.md#how-do-i-publish-package) |
[Design](DESIGN.md) |
[Contributing](CONTRIBUTING.md) |
[Cask.toml](Cask.toml.md)

</div>

If you are tired of:

1. Install different package manager in different platform (Homebrew/Chocolatey/Scoop).
2. Writing installation scripts many times(Bash/PowerShell).
3. Update remote package information when release a new version

Then welcome to Cask.

## Installation

1. Shell (Mac/Linux)

   ```bash
   curl -fsSL https://github.com/cask-pkg/cask.rs/raw/main/install.sh | bash
   ```

2. PowerShell (Windows):

   ```pwshell
   iwr https://github.com/cask-pkg/cask.rs/raw/main/install.ps1 -useb | iex
   ```

3. [Github release page](https://github.com/cask-pkg/cask.rs/releases)

   Download the executable, then put it in the `$PATH` directory.

4. Upgrade from existing version

   ```bash
   cask self-update
   ```

## Usage

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

Show more information with `cask --help` command.

## Command

| Command                            | Description                                |
| ---------------------------------- | ------------------------------------------ |
| cask install \<PACKAGE\> [VERSION] | Install package                            |
| cask uninstall \<PACKAGE\>         | Uninstall package                          |
| cask info \<PACKAGE\>              | Show information of package                |
| cask update \<PACKAGE\>            | Update package to latest                   |
| cask check-updates                 | Check and update packages to latest        |
| cask list                          | List installed package                     |
| cask clean                         | Clear residual data                        |
| cask self-update                   | Update Cask to the newest version          |
| cask remote sync                   | Sync build-in formula from remote to local |
| cask remote list                   | List build-in formula on remote            |

## Requirement

Cask depends on [Git](https://git-scm.com)

## Contributors

This project exists thanks to all the people who contribute. [How to contribute](CONTRIBUTING.md).

<a href="https://github.com/cask-pkg/cask.rs/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=cask-pkg/cask.rs" />
</a>

## LICENSE

The [MIT License](LICENSE)
