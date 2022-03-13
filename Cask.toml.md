# Cask.Toml Description Document

This is the basic configuration of `Cask.Toml`.

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

As you can see, it only contains a few top-level fields:

- [package](#Package)

- [darwin](#Platform-specify-configuration)

- [linux](#Platform-specify-configuration)

- [windows](#Platform-specify-configuration)

## Package

| Field        | Description                                                  | type            | required | example                                   |
| ------------ | ------------------------------------------------------------ | --------------- | -------- | ----------------------------------------- |
| **name**     | The name of package                                          | string          | true     | `"github.com/axetroy/gpm.rs"`             |
| **bin**      | The binary name of package                                   | string          | true     | `"gpm"`                                   |
| **versions** | The versions of package.<br/>The latest version at the head. | Array\<string\> | true     | `["0.1.12", "0.1.11"]`                    |
| authors      | Show information of remote package                           | Array\<string\> |          | `["Axetroy <axetroy.dev@gmail.com>"]`     |
| keywords     | The keywords of package                                      | Array\<string\> |          | `["foo", "bar"]`                          |
| repository   | The repository url of package                                | string          |          | `"https://github.com/axetroy/gpm.rs.git"` |
| description  | The description information of package                       | string          |          | `"The description"`                       |

## Platform-specify-configuration

Current Support the arch:

- x86
- x86_64
- aarch64
- mips
- mips64
- mips64el

Every arch got a [Resource Target Object](#Resource-Target)

### Resource-Target

| Field    | Description                                                   | type   | required | example |
| -------- | ------------------------------------------------------------- | ------ | -------- | ------- |
| **url**  | The url of resource that will be download                     | string | true     |         |
| checksum | The checksum(SHA256) of resource. Check checksum if provided. | string |          |         |
