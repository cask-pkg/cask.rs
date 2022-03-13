// #![deny(warnings)]

use crate::git;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, fs};

use eyre::Report;
use serde::Serialize;
use serde_derive::Deserialize;
use tinytemplate::TinyTemplate;
use toml::from_str;
use url::Url;

// #[derive(Serialize)]
#[derive(Deserialize, Serialize)]
pub struct Formula {
    #[serde(skip)]
    pub file_content: String, // the file content

    pub cask: Option<Cask>, // The cask information that generated by cask. This field is only available after the package is installed.
    pub package: Package,   // The package information
    pub windows: Option<Platform>, // The windows target information
    pub darwin: Option<Platform>, // The macOS target information
    pub linux: Option<Platform>, // The linux target information
    pub dependencies: Option<Vec<String>>, // TODO: The dependencies will be installed before install package
    pub postinstall: Option<String>,       // TODO: The script will run after install package
}

#[derive(Deserialize, Serialize)]
pub struct Cask {
    pub name: String,       // The package name. eg github.com/axetroy/gpm.rsÏ
    pub created_at: String, // The package installed date
    pub version: String,    // The version is using for package
    pub repository: String, // The package installed from the repository url
}

#[derive(Deserialize, Serialize)]
pub struct Package {
    pub name: String,                  // The package name
    pub bin: String,                   // The binary name of package
    pub version: Option<String>,       // Specify the download package version.
    pub versions: Vec<String>,         // The version of package
    pub authors: Vec<String>,          // The author of package
    pub keywords: Option<Vec<String>>, // The keywords of package
    pub repository: String,            // The repository url
    pub description: String,           // The description of package
}

#[derive(Deserialize, Serialize)]
pub struct Platform {
    pub x86: Option<Arch>,
    pub x86_64: Option<Arch>,
    pub arm: Option<Arch>,
    pub aarch64: Option<Arch>,
    pub mips: Option<Arch>,
    pub mips64: Option<Arch>,
    pub mips64el: Option<Arch>,
}

#[derive(Deserialize, Serialize)]
pub struct Arch {
    pub url: String,              // The url will be download when install the package
    pub checksum: Option<String>, // The hash256 of download resource
    pub ext: Option<String>, // The extension name of download resource. optional value: ".tar.gz" ".tar" ".zip"
}

pub fn new(formula_file: &Path) -> Result<Formula, Report> {
    let mut file = match File::open(formula_file) {
        Ok(f) => f,
        Err(e) => {
            return Err(eyre::Report::from(e));
        }
    };

    let mut file_content = String::new();

    match file.read_to_string(&mut file_content) {
        Ok(_) => {}
        Err(e) => {
            return Err(eyre::Report::from(e));
        }
    };

    drop(file);

    let mut f: Formula = match from_str(&file_content) {
        Ok(r) => r,
        Err(e) => return Err(eyre::Report::from(e)),
    };

    f.file_content = file_content;

    Ok(f)
}

pub struct DownloadTarget {
    pub url: String,
    pub checksum: Option<String>,
    pub ext: String,
}

pub fn get_formula_git_url(package_name: &str) -> String {
    format!("https://{}-cask.git", package_name)
}

// fetch remote formula
pub fn fetch(package_name: &str) -> Result<Formula, Report> {
    let cask_git_url = get_formula_git_url(package_name);

    let unix_time = {
        let start = SystemTime::now();

        let t = start.duration_since(UNIX_EPOCH)?;

        t.as_secs()
    };

    let formula_cloned_dir = env::temp_dir().join(format!("cask_formula_{}", unix_time));
    let cask_file_path = formula_cloned_dir.join("Cask.toml");

    match git::clone(
        &cask_git_url,
        &formula_cloned_dir,
        vec!["--depth", "1", "--quiet"],
    ) {
        Ok(()) => {
            if !cask_file_path.exists() {
                fs::remove_dir_all(formula_cloned_dir)?;
                return Err(eyre::format_err!(
                    "{} is not a valid formula!",
                    package_name
                ));
            }

            match new(&cask_file_path) {
                Ok(r) => {
                    fs::remove_dir_all(formula_cloned_dir)?;
                    Ok(r)
                }
                Err(e) => {
                    fs::remove_dir_all(formula_cloned_dir)?;

                    Err(e)
                }
            }
        }
        Err(e) => Err(e),
    }
}

impl Formula {
    fn get_current_os(&self) -> Option<&Platform> {
        if cfg!(target_os = "macos") {
            self.darwin.as_ref()
        } else if cfg!(target_os = "windows") {
            self.windows.as_ref()
        } else if cfg!(target_os = "linux") {
            self.linux.as_ref()
        } else {
            None
        }
    }
    fn get_current_arch(&self) -> Option<&Arch> {
        if let Some(os) = self.get_current_os() {
            if cfg!(target_arch = "x86") {
                os.x86.as_ref()
            } else if cfg!(target_arch = "x86_64") {
                os.x86_64.as_ref()
            } else if cfg!(target_arch = "arm") {
                os.arm.as_ref()
            } else if cfg!(target_arch = "aarch64") {
                os.aarch64.as_ref()
            } else if cfg!(target_arch = "mips") {
                os.mips.as_ref()
            } else if cfg!(target_arch = "mips64") {
                os.mips64.as_ref()
            } else if cfg!(target_arch = "mips64el") {
                os.mips64el.as_ref()
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_file_content(&self) -> String {
        self.file_content.clone()
    }

    pub fn get_current_download_url(&self, version: &str) -> Result<DownloadTarget, Report> {
        #[derive(Serialize)]
        struct URLTemplateContext {
            name: String,
            bin: String,
            version: String,
        }

        if let Some(arch) = self.get_current_arch() {
            let render_context = URLTemplateContext {
                name: self.package.name.clone(),
                bin: self.package.bin.clone(),
                version: version.to_string(),
            };

            let mut tt = TinyTemplate::new();
            tt.add_template("url_template", &arch.url)?;

            let renderer_url = tt.render("url_template", &render_context)?;

            let ext_name = match &arch.ext {
                Some(s) => s.clone(),
                None => {
                    let u = Url::parse(&renderer_url)?;

                    let default_ext = ".tar.gz".to_string();

                    if let Some(sep) = u.path_segments() {
                        if let Some(filename) = sep.last() {
                            if filename.ends_with(".tar.gz") {
                                ".tar.gz".to_string()
                            } else if filename.ends_with(".tar") {
                                ".tar".to_string()
                            } else if filename.ends_with(".zip") {
                                ".zip".to_string()
                            } else {
                                default_ext
                            }
                        } else {
                            default_ext
                        }
                    } else {
                        default_ext
                    }
                }
            };

            Ok(DownloadTarget {
                url: renderer_url,
                checksum: arch.checksum.clone(),
                ext: ext_name,
            })
        } else {
            Err(eyre::format_err!(
                "the package '{}' not support your system",
                self.package.name
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::formula;

    #[test]
    fn test_read_config() {
        let config_path = env::current_dir()
            .unwrap()
            .join("fixtures")
            .join("config")
            .join("default_Cask.toml");

        let rc = formula::new(&config_path).unwrap();

        assert_eq!(rc.package.name, "gpm");
        assert_eq!(rc.package.bin, "gpm");
        assert!(rc.package.version.is_none());
        assert_eq!(rc.package.versions, vec!["0.1.12", "0.1.11"]);
        assert_eq!(rc.package.authors, vec!["Axetroy <axetroy.dev@gmail.com>"]);
        assert_eq!(
            rc.package.keywords.unwrap(),
            vec!["gpm", "git", "project", "manager"]
        );
        assert_eq!(rc.package.repository, "https://github.com/axetroy/gpm.rs");
        assert_eq!(
            rc.package.description,
            "A command line tool, manage your hundreds of repository, written with Rust.\n"
        );

        let windows = &rc.windows.unwrap();
        let darwin = &rc.darwin.unwrap();
        let linux = &rc.linux.unwrap();

        // windows
        assert_eq!(
            windows.x86.as_ref().unwrap().url,
            "https://github.com/axetroy/gpm.rs/releases/download/v{version}/gpm_windows_386.tar.gz"
        );
        assert_eq!(
            windows.x86_64.as_ref().unwrap().url,
            "https://github.com/axetroy/gpm.rs/releases/download/v{version}/gpm_windows_amd64.tar.gz"
        );
        assert_eq!(
            windows.aarch64.as_ref().unwrap().url,
            "https://github.com/axetroy/gpm.rs/releases/download/v{version}/gpm_windows_arm64.tar.gz"
        );

        // darwin
        assert_eq!(
            darwin.x86_64.as_ref().unwrap().url,
            "https://github.com/axetroy/gpm.rs/releases/download/v{version}/gpm_darwin_amd64.tar.gz"
        );
        assert_eq!(
            darwin.aarch64.as_ref().unwrap().url,
            "https://github.com/axetroy/gpm.rs/releases/download/v{version}/gpm_darwin_arm64.tar.gz"
        );

        // linux
        assert_eq!(
            linux.x86.as_ref().unwrap().url,
            "https://github.com/axetroy/gpm.rs/releases/download/v{version}/gpm_linux_386.tar.gz"
        );
        assert_eq!(
            linux.x86_64.as_ref().unwrap().url,
            "https://github.com/axetroy/gpm.rs/releases/download/v{version}/gpm_linux_amd64.tar.gz"
        );
        assert_eq!(
            linux.aarch64.as_ref().unwrap().url,
            "https://github.com/axetroy/gpm.rs/releases/download/v{version}/gpm_linux_arm64.tar.gz"
        );
        assert_eq!(
            linux.mips.as_ref().unwrap().url,
            "https://github.com/axetroy/gpm.rs/releases/download/v{version}/gpm_linux_mips.tar.gz"
        );
        assert_eq!(
            linux.mips64.as_ref().unwrap().url,
            "https://github.com/axetroy/gpm.rs/releases/download/v{version}/gpm_linux_mips64.tar.gz"
        );
        assert_eq!(
            linux.mips64el.as_ref().unwrap().url,
            "https://github.com/axetroy/gpm.rs/releases/download/v{version}/gpm_linux_mips64el.tar.gz"
        );
    }
}
