# Design

This document describes the design pattern of Cask and how it works.

## Motivation

For the developers of the tool, publishing is a distressed thing.

Mainly have a few more troublesome things:

1. Not cross-platform

   Every platform has its own version, eg `Brew` in MacOS, `scoop` in Windows

2. Package manager usually has a center

   eg. `Brew` need to add package information to [homebrew-core](https://github.com/Homebrew/homebrew-core) then it can install directly. Of course, this is not necessary.

3. Write too much installation script

   If you don't like these package managers, then you will see your own writing scripts for installation.

   eg `install.ps1` in Windows or `install.sh` in Unix.

Based on the above reasons, I need a package manager.

It needs to meet the following characteristics

- cross-platform support
- distributed publishing

## How it works?

Cask will get a configuration file(`Cask.toml`) from a remote repository.

Cask will download and install (or build) according to the information in the `Cask.toml`.

If you are not the publisher of the package, you don't need to care about this configuration file.

In the following document, the more information of `Cask.toml` will be described.

## How do I publish my/others package?

Cask is distributed, no servers, not to keep your package information.

So you have to add the package yourself.

eg. your repository address is `https://github.com/<username>/<repo>.git`.

You need to create a new repository name `https://github.com/<username>/<repo>-cask.git`.

And then create `Cask.toml` in the root of repository.

```toml
[package]
name = "github.com/<username>/<repo>"
bin = "gpm"
versions = ["0.1.0"]
authors = ["Username <email@email.com>"]
keywords = ["key", "word"]
repository = "https://github.com/<username>/<repo>"
description = """
description of package.
"""

[darwin]
x86_64 = { url = "https://github.com/<username>/<repo>/releases/download/v{version}/darwin_amd64.tar.gz" }

[windows]
x86_64 = { url = "https://github.com/<username>/<repo>/releases/download/v{version}/windows_amd64.tar.gz" }

[linux]
x86_64 = { url = "https://github.com/<username>/<repo>/releases/download/v{version}/linux_amd64.tar.gz" }
```

Please modify the information above, eg. username, version, etc.

for more information about [Cask.toml](Cask.toml.md)
