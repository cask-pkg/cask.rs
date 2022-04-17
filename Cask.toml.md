# Cask.Toml Description Document

This is the basic configuration of `Cask.Toml`.

```toml
[package]
name = "github.com/<username>/<repo>"
bin = "gpm"
repository = "https://github.com/<username>/<repo>"
description = """
description of package.
"""

[context]
foo = "bar"
hello = "world"

[darwin]
x86_64 = "{package.repository}/releases/download/v{version}/{package.bin}_darwin_amd64.tar.gz"

[windows]
x86_64 = "{package.repository}/releases/download/v{version}/{package.bin}_windows_amd64.tar.gz"

[linux]
x86_64 = "{package.repository}/releases/download/v{version}/{package.bin}_linux_amd64.tar.gz"

[hook.windows.cmd]
postinstall = """
echo "hello postinstall from cmd"
"""

[hook.unix.sh]
postinstall = """
echo "hello postinstall from sh"
"""
```

As you can see, it only contains a few top-level fields:

| Field                                      | Description                         | required |
| ------------------------------------------ | ----------------------------------- | -------- |
| [package](#Package)                        | Defined the information of package  | true     |
| [darwin](#Platform-specify-configuration)  | The information of macOS platform   |          |
| [linux](#Platform-specify-configuration)   | The information of Linux platform   |          |
| [windows](#Platform-specify-configuration) | The information of Windows platform |          |
| [freebsd](#Platform-specify-configuration) | The information of Windows platform |          |
| [hook.windows](#Terminal)                  | The hook for windows                |          |
| [hook.unix](#Terminal)                     | The hook for unix                   |          |
| [hook.linux](#Terminal)                    | The hook for linux                  |          |
| [hook.macos](#Terminal)                    | The hook for macos                  |          |
| [hook.freebsd](#Terminal)                  | The hook for freebsd                |          |

## Package

| Field           | Description                                                                                                                                      | type            | required | example                                   |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ | --------------- | -------- | ----------------------------------------- |
| **name**        | The name of package                                                                                                                              | string          | true     | `"github.com/axetroy/gpm.rs"`             |
| **description** | The description information of package                                                                                                           | string          | true     | `"The description"`                       |
| **bin**         | The non-extension binary name of package                                                                                                         | string          | true     | `"gpm"`                                   |
| **repository**  | The repository url of package                                                                                                                    | string          | true     | `"https://github.com/axetroy/gpm.rs.git"` |
| versions        | The versions without 'v' prefix of package.<br/>The latest version at the head.<br/> Cask will get versions from repository tags if not provide. | Array\<string\> |          | `["0.1.12", "0.1.11"]`                    |
| authors         | The authors of package package                                                                                                                   | Array\<string\> |          | `["Axetroy <axetroy.dev@gmail.com>"]`     |
| keywords        | The keywords of package                                                                                                                          | Array\<string\> |          | `["foo", "bar"]`                          |
| license         | The license of package                                                                                                                           | string          |          | `"MIT"`                                   |

## Platform-specify-configuration

Current Support the arch:

- x86
- x86_64
- arm
- armv7
- aarch64
- mips
- mips64
- mips64el
- riscv64

Every arch got a [Resource Target Object](#Resource-Target)

### Resource-Target

The target resource can be one of following types:

1. String: tarball URL

The resource URL that will be download and extract. The tarball format support `.tar`/`.tgz`/`.tar.gz`/`.tar.bz2`/`.zip`

```toml
[darwin]
x86_64 = "https://github.com/<username>/<repo>/releases/download/v{version}/darwin_amd64.tar.gz"
```

2. Object: tarball URL detail with more information

| Field     | Description                                                   | type   | required | example       |
| --------- | ------------------------------------------------------------- | ------ | -------- | ------------- |
| **url**   | The url of resource that will be download                     | string | true     |               |
| checksum  | The checksum(SHA256) of resource. Check checksum if provided. | string |          |               |
| extension | The resource extension. Specify the extension of resource     | string |          | ".tar.gz"     |
| path      | The folder that binary file locate in the tarball             | string |          | "/sub-folder" |

The extension support `.tar`/`.tgz`/`.tar.gz`/`.tar.bz2`/`.zip`

```toml
[darwin]
x86_64 = { url = "https://github.com/<username>/<repo>/releases/download/v{version}/darwin_amd64.tar.gz", checksum = "15f841b9b8f60033528dfdce5883e622145911ede1f59d1f302042ded4c565a4", extension = ".tar.gz" }
```

3. Object: executable file URL

| Field          | Description                                                   | type   | required | example |
| -------------- | ------------------------------------------------------------- | ------ | -------- | ------- |
| **executable** | The url of resource of executable that will be download       | string | true     |         |
| checksum       | The checksum(SHA256) of resource. Check checksum if provided. | string |          |         |

```toml
[darwin]
x86_64 = { executable = "https://github.com/<username>/<repo>/releases/download/v{version}/executable" }
```

### Terminal

| Terminal   | Description | type          | required | example |
| ---------- | ----------- | ------------- | -------- | ------- |
| cmd        | cmd.exe     | [Hook](#Hook) |          |         |
| powershell | PowerShell  | [Hook](#Hook) |          |         |
| sh         | sh          | [Hook](#Hook) |          |         |
| bash       | bash        | [Hook](#Hook) |          |         |

### Hook

| Hook        | Description                                | type   | required | example |
| ----------- | ------------------------------------------ | ------ | -------- | ------- |
| preinstall  | The script will run before install package | string |          |         |
| postinstall | The script will run after install package  | string |          |         |

```toml
[hook.windows.cmd]
preinstall = """
echo "running preinstall hook"
"""
postinstall = """
echo "running postinstall hook"
"""
```
