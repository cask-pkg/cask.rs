# Design

This document describes the design pattern of Cask and how it works.

## Motivation

For the developers who writing the cli tool, publishing is a distressed thing.

Mainly have a few more troublesome things:

1. Not cross-platform

   Every platform has its oackage, eg `Brew` in MacOS, `scoop` in Windows

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

Cask is distributed, no servers, not to keep your package information.

So you have to add the package yourself.

eg. your repository address is `https://github.com/<username>/<repo>.git`, you need to create a `Cask.toml` config file in the root of the repository.

Or you can create a new repository named `https://github.com/<username>/<repo>-cask.git` and then create `Cask.toml` in the root of the repository.

```toml
[package]
name = "github.com/<username>/<repo>"
bin = "gpm"
# Cask will get versions from repository tags if versions field not provide.
# versions = ["0.1.0"]
authors = ["Username <email@email.com>"]
keywords = ["key", "word"]
repository = "https://github.com/<username>/<repo>"
description = """
description of package.
"""

[darwin]
x86_64 =  "https://github.com/<username>/<repo>/releases/download/v{version}/darwin_amd64.tar.gz"

[windows]
x86_64 = "https://github.com/<username>/<repo>/releases/download/v{version}/windows_amd64.tar.gz"

[linux]
x86_64 = "https://github.com/<username>/<repo>/releases/download/v{version}/linux_amd64.tar.gz"
```

modify the information above, eg. username, version, etc. and then push to remote.

try to install the package

```bash
cask install github.com/<username>/<repo>
```

for more information about [Cask.toml](Cask.toml.md)

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
