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

        let paths = env::var_os(key)
            .ok_or_else(|| eyre::format_err!("{} is not defined in the environment.", key))?;

        for path in env::split_paths(&paths) {
            let abs_path_str = path.as_os_str().to_string_lossy().replace(
                '~',
                &dirs::home_dir()
                    .ok_or_else(|| eyre::format_err!("can not get home dir"))?
                    .as_os_str()
                    .to_string_lossy(),
            );

            let abs_path = Path::new(&abs_path_str);

            if format!("{}", abs_path.display()) == format!("{}", self.bin_dir().display()) {
                return Ok(());
            }
        }

        let msg = format!(
            r#"REQUIREMENT:

            make sure '{}' has been add to your $PATH environment variable.

            manually add the directory to your $HOME/.bash_profile (or similar)

            then create a new session in terminal
            "#,
            self.bin_dir().display()
        )
        .lines()
        .map(|s| s.trim_start().to_owned())
        .collect::<Vec<String>>()
        .join("\n");

        Err(eyre::format_err!(msg))
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

    pub fn list_formula(&self) -> Result<Vec<formula::Formula>, Report> {
        let formula_dir = self.formula_dir();
        let mut list: Vec<formula::Formula> = vec![];

        let dir = fs::read_dir(formula_dir)?;

        for entry in dir.into_iter().filter(|f| f.is_ok()).map(|f| f.unwrap()) {
            let p = entry.path();

            if !p.is_dir() {
                continue;
            }

            let cask_file_path = p.join("Cask.toml");

            if !cask_file_path.exists() {
                continue;
            }

            let package_formula = formula::new(&cask_file_path, "")?;

            list.push(package_formula);
        }

        Ok(list)
    }
}
