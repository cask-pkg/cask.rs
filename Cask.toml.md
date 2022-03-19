# Cask.Toml Description Document

This is the basic configuration of `Cask.Toml`.

```toml
[package]
name = "github.com/<username>/<repo>"
bin = "gpm"
# Cask will get versions from repository tags if versions field not provide.
versions = ["0.1.0"]
authors = ["Username <email@email.com>"]
keywords = ["key", "word"]
repository = "https://github.com/<username>/<repo>"
description = """
description of package.
"""

[hook]
preinstall = """
echo "running preinstall hook"
"""
postinstall = """
echo "running postinstall hook"
"""

[darwin]
x86_64 = { url = "https://github.com/<username>/<repo>/releases/download/v{version}/darwin_amd64.tar.gz" }

[windows]
x86_64 = { url = "https://github.com/<username>/<repo>/releases/download/v{version}/windows_amd64.tar.gz" }

[linux]
x86_64 = { url = "https://github.com/<username>/<repo>/releases/download/v{version}/linux_amd64.tar.gz" }
```

As you can see, it only contains a few top-level fields:

| Field                                      | Description                         | required |
| ------------------------------------------ | ----------------------------------- | -------- |
| [package](#Package)                        | Defined the information of package  | true     |
| [hook](#Hook)                              | The hook should run in some moment  |          |
| [darwin](#Platform-specify-configuration)  | The information of macOS platform   |          |
| [linux](#Platform-specify-configuration)   | The information of Linux platform   |          |
| [windows](#Platform-specify-configuration) | The information of Windows platform |          |

## Package

| Field           | Description                                                                                                                                      | type            | required | example                                   |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ | --------------- | -------- | ----------------------------------------- |
| **name**        | The name of package                                                                                                                              | string          | true     | `"github.com/axetroy/gpm.rs"`             |
| **description** | The description information of package                                                                                                           | string          | true     | `"The description"`                       |
| **bin**         | The non-extension binary name of package                                                                                                         | string          | true     | `"gpm"`                                   |
| **repository**  | The repository url of package                                                                                                                    | string          | true     | `"https://github.com/axetroy/gpm.rs.git"` |
| **versions**    | The versions without 'v' prefix of package.<br/>The latest version at the head.<br/> Cask will get versions from repository tags if not provide. | Array\<string\> |          | `["0.1.12", "0.1.11"]`                    |
| authors         | Show information of remote package                                                                                                               | Array\<string\> |          | `["Axetroy <axetroy.dev@gmail.com>"]`     |
| keywords        | The keywords of package                                                                                                                          | Array\<string\> |          | `["foo", "bar"]`                          |
| license         | The license of package                                                                                                                           | string          |          | `"MIT"`                                   |

## Platform-specify-configuration

Current Support the arch:

- x86_64
- aarch64
- mips64
- mips64el

Every arch got a [Resource Target Object](#Resource-Target)

### Resource-Target

| Field     | Description                                                   | type   | required | example   |
| --------- | ------------------------------------------------------------- | ------ | -------- | --------- |
| **url**   | The url of resource that will be download                     | string | true     |           |
| checksum  | The checksum(SHA256) of resource. Check checksum if provided. | string |          |           |
| extension | The resource extension. Specify the extension of resource     | string |          | ".tar.gz" |

### Hook

| Hook        | Description                                | type   | required | example |
| ----------- | ------------------------------------------ | ------ | -------- | ------- |
| preinstall  | The script will run before install package | string |          |         |
| postinstall | The script will run after install package  | string |          |         |
