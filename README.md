<div align="center">
   <img src="logo.svg" with="64" height="64"/>

   <h1>Cask</h1>

[![lint](https://github.com/axetroy/cask.rs/actions/workflows/lint.yml/badge.svg)](https://github.com/axetroy/cask.rs/actions/workflows/lint.yml)
[![test](https://github.com/axetroy/cask.rs/actions/workflows/test.yml/badge.svg)](https://github.com/axetroy/cask.rs/actions/workflows/test.yml)
[![build](https://github.com/axetroy/cask.rs/actions/workflows/build.yml/badge.svg)](https://github.com/axetroy/cask.rs/actions/workflows/build.yml)
![Latest Version](https://img.shields.io/github/v/release/axetroy/cask.rs.svg)
![License](https://img.shields.io/github/license/axetroy/cask.rs.svg)
![Repo Size](https://img.shields.io/github/repo-size/axetroy/cask.rs.svg)

A universal distributed binary distribution manage tool.

[Installation](#Installation) |
[Docs](docs.md) |
[Changelog](CHANGELOG.md) |
[Design](DESIGN.md) |
[Contributing](CONTRIBUTING.md)

</div>

## Installation

1. Shell (Mac/Linux)

   ```bash
   curl -fsSL https://github.com/release-lab/install/raw/v1/install.sh | bash -s -- -r=axetroy/cask.rs -e=cask
   ```

2. PowerShell (Windows):

   ```powershell
   $r="axetroy/cask.rs";$e="cask";iwr https://github.com/release-lab/install/raw/v1/install.ps1 -useb | iex
   ```

3. [Github release page](https://github.com/axetroy/cask.rs/releases)

   Download the executable, then put it in the `$PATH` directory.

## Usage

```sh
# cask install <package>
cask install github.com/axetroy/gpm.rs

# cask uninstall <package>
cask uninstall github.com/axetroy/gpm.rs
```

## Command

| Command                            | Description                 |
| ---------------------------------- | --------------------------- |
| cask install \<PACKAGE\> [VERSION] | Install package             |
| cask uninstall \<PACKAGE\>         | Uninstall package           |
| cask info \<PACKAGE\>              | Show information of package |
| cask list                          | List installed package      |

## Requirement

Cask depends on [Git](https://git-scm.com)

## LICENSE

The [MIT License](LICENSE)
