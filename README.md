<div align="center">
   <img src="logo.svg" with="64" height="64"/>

   <h1>Cask</h1>

[![lint](https://github.com/axetroy/cask.rs/actions/workflows/lint.yml/badge.svg)](https://github.com/axetroy/cask.rs/actions/workflows/lint.yml)
[![test](https://github.com/axetroy/cask.rs/actions/workflows/test.yml/badge.svg)](https://github.com/axetroy/cask.rs/actions/workflows/test.yml)
[![build](https://github.com/axetroy/cask.rs/actions/workflows/build.yml/badge.svg)](https://github.com/axetroy/cask.rs/actions/workflows/build.yml)
![Latest Version](https://img.shields.io/github/v/release/axetroy/cask.rs.svg)
![License](https://img.shields.io/github/license/axetroy/cask.rs.svg)
![Repo Size](https://img.shields.io/github/repo-size/axetroy/cask.rs.svg)

A universal, distributed package manager.

[Installation](#Installation) |
[Usage](#Usage) |
[How to publish package?](DESIGN.md#how-do-i-publish-my-package) |
[Design](DESIGN.md) |
[Contributing](CONTRIBUTING.md) |
[Cask.toml](Cask.toml.md)

</div>

Refuse to write the installation script. Refuse to upload packages to the center server. Refuse to write a cumbersome configuration.

Then choose Cask

Features:

- [x] cross-platform support
- [x] none center server
- [x] easy to use

## Installation

1. Shell (Mac/Linux)

   ```bash
   curl -fsSL https://github.com/axetroy/cask.rs/raw/main/install.sh | bash
   ```

2. PowerShell (Windows):

   ```pwshell
   iwr https://github.com/axetroy/cask.rs/raw/main/install.ps1 -useb | iex
   ```

3. [Github release page](https://github.com/axetroy/cask.rs/releases)

   Download the executable, then put it in the `$PATH` directory.

4. Upgrade from existing version

   ```bash
   cask update
   ```

## Usage

```sh
# cask install <package>
cask install github.com/axetroy/gpm.rs

# cask uninstall <package>
cask uninstall github.com/axetroy/gpm.rs
```

## Command

| Command                            | Description                           |
| ---------------------------------- | ------------------------------------- |
| cask install \<PACKAGE\> [VERSION] | Install package                       |
| cask uninstall \<PACKAGE\>         | Uninstall package                     |
| cask info \<PACKAGE\>              | Show information of installed package |
| cask search \<PACKAGE\>            | Show information of remote package    |
| cask upgrade \<PACKAGE\>           | Upgrade package to latest             |
| cask list                          | List installed package                |
| cask clean                         | Clear residual data                   |
| cask update                        | Update Cask to the newest version     |

## Requirement

Cask depends on [Git](https://git-scm.com)

## LICENSE

The [MIT License](LICENSE)
