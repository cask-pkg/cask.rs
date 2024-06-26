# Design

This document describes the design pattern of Cask and how it works.

## Motivation

For the developers who writing the cli tool, publishing is a distressed thing.

Mainly have a few more troublesome things:

1. Not cross-platform

   Every platform has its own packager manager, eg `Brew` in MacOS, `scoop` in Windows

2. Package manager usually has a center server

   eg. `homebrew` need to add package information to [homebrew-core](https://github.com/Homebrew/homebrew-core) then it can install directly. Of course, this is not necessary. User can create a custom Formula.

3. Write too much installation script

   If you don't like these package managers, then you can write scripts for installation.

   eg `install.ps1` in Windows or `install.sh` in Unix.

Based on the above reasons, I need a package manager.

It needs the following feature:

- Cross-platform support
- Distributed publishing
- Reduce use cost

## How it works?

Cask will get a configuration file(`Cask.toml`) from a remote repository.

Cask will download and install (or build) according to the information in the `Cask.toml`.

If you are not the publisher of the package, you don't need to care about this configuration file.

In the following document, the more information of `Cask.toml` will be described.

## How do I publish package?

You can release your package with following ways(pick one):

1. Create a [Cask.toml](Cask.toml.md) file in the root of your repository.

2. Create a [Cask.toml](Cask.toml.md) file in [cask-core](https://github.com/cask-pkg/cask-core)

   And then run the following command in your local machine

   ```bash
   cask remote sync
   ```

Finally, try to install the package

```bash
cask install github.com/<username>/<repo>
```

## How do I publish a new version to an exist package?

if your `Cask.toml` file not provide the `package.versions` field

you need to create a new tag and for your package repository. eg.

```bash
git tag v0.2.0
git push --tags
```

or add a new version in the `package.versions` of `Cask.Toml`.

```diff
[package]
name = "github.com/<username>/<repo>"
bin = "gpm"
- versions = ["0.1.0"]
+ versions = ["0.2.0", "0.1.0"]
authors = ["Username <email@email.com>"]
keywords = ["key", "word"]
repository = "https://github.com/<username>/<repo>"
description = """
description of package.
"""

[darwin]
x86_64 = "https://github.com/<username>/<repo>/releases/download/v{version}/darwin_amd64.tar.gz"

[windows]
x86_64 = "https://github.com/<username>/<repo>/releases/download/v{version}/windows_amd64.tar.gz"

[linux]
x86_64 = "https://github.com/<username>/<repo>/releases/download/v{version}/linux_amd64.tar.gz"
```
