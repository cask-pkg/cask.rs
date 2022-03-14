#![deny(warnings)]

use crate::formula;

use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use eyre::Report;
use sha2::{Digest, Sha256};

pub struct Cask {
    root: PathBuf, // the root of the cask
}

pub fn new(root: &Path) -> Cask {
    Cask {
        root: root.to_path_buf(),
    }
}

impl Cask {
    // init the cask folder
    pub fn init(&self) -> Result<(), Report> {
        if !self.root_dir().exists() {
            fs::create_dir_all(&self.root_dir())?;
        }

        if !self.bin_dir().exists() {
            fs::create_dir_all(&self.bin_dir())?;
        }

        if !self.formula_dir().exists() {
            fs::create_dir_all(&self.formula_dir())?;
        }

        Ok(())
    }

    // check bin path of Cask
    pub fn check_bin_path(&self) -> Result<(), Report> {
        let key = "PATH";
        match env::var_os(key) {
            Some(paths) => {
                for path in env::split_paths(&paths) {
                    let abs_path_str = path
                        .as_os_str()
                        .to_str()
                        .unwrap()
                        .replace('~', dirs::home_dir().unwrap().as_os_str().to_str().unwrap());

                    let abs_path = Path::new(&abs_path_str);

                    if format!("{}", abs_path.display()) == format!("{}", self.bin_dir().display())
                    {
                        return Ok(());
                    }
                }

                Err(eyre::format_err!(
                    "make sure '{}' has been add to your $PATH environment variable.",
                    self.bin_dir().display()
                ))
            }
            None => Err(eyre::format_err!(
                "{} is not defined in the environment.",
                key
            )),
        }
    }

    pub fn root_dir(&self) -> PathBuf {
        self.root.clone()
    }

    pub fn bin_dir(&self) -> PathBuf {
        self.root_dir().join("bin")
    }

    pub fn formula_dir(&self) -> PathBuf {
        self.root_dir().join("formula")
    }

    // the package folder
    pub fn package_dir(&self, package_name: &str) -> PathBuf {
        let hash_of_package = {
            let mut hasher = Sha256::new();

            hasher.update(package_name);
            format!("{:x}", hasher.finalize())
        };

        self.formula_dir().join(hash_of_package)
    }

    pub fn package_bin_dir(&self, package_name: &str) -> PathBuf {
        self.package_dir(package_name).join("bin")
    }

    pub fn package_version_dir(&self, package_name: &str) -> PathBuf {
        self.package_dir(package_name).join("version")
    }

    pub fn init_package(&self, package_name: &str) -> Result<(), Report> {
        let package_dir = self.package_dir(package_name);
        let package_bin_dir = self.package_bin_dir(package_name);
        let package_version_dir = self.package_version_dir(package_name);

        if !package_dir.exists() {
            fs::create_dir_all(package_dir)?;
        }

        if !package_bin_dir.exists() {
            fs::create_dir_all(package_bin_dir)?;
        }

        if !package_version_dir.exists() {
            fs::create_dir_all(package_version_dir)?;
        }

        Ok(())
    }

    pub fn package_formula(&self, package_name: &str) -> Result<formula::Formula, Report> {
        let package_dir = self.package_dir(package_name);
        let formula_file_path = package_dir.join("Cask.toml");

        let package_formula = formula::new(&formula_file_path)?;

        Ok(package_formula)
    }
}
