//  STATE.rs
//    by Lut99
//
//  Created:
//    17 Jul 2024, 19:03:23
//  Last edited:
//    17 Jul 2024, 22:29:57
//  Auto updated?
//    Yes
//
//  Description:
//!   Represents runtime state shared by paths.
//

use std::fmt::{Display, Formatter, Result as FResult};
use std::fs::File;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::{error, fs};

use log::{debug, info, warn};
use serde::{Deserialize, Serialize};


/***** CONSTANTS *****/
/// The default contents of the not found file.
const DEFAULT_NOT_FOUND_FILE: &'static str = r#"
<!DOCTYPE html>
<html>
    <head>
        <title>Not found</title>
    </head>
    <body>
        Oops! That page wasn't found on this server.
    </body>
</html>
"#;





/***** ERRORS *****/
/// Defines errors thrown by the [`Context`].
#[derive(Debug)]
pub enum Error {
    /// Failed to open the target config file.
    ConfigOpen { path: PathBuf, err: std::io::Error },
    /// Failed to read & parse the target config file.
    ConfigParse { path: PathBuf, err: serde_yml::Error },

    /// Failed to create a default config file.
    ConfigCreate { path: PathBuf, err: std::io::Error },
    /// Failed to write to the default config file.
    ConfigWrite { path: PathBuf, err: serde_yml::Error },
    /// Failed to create a default not found file.
    NotFoundFileCreate { path: PathBuf, err: std::io::Error },
    /// Failed to canonicalize the site directory path.
    SiteDirCanonicalize { path: PathBuf, err: std::io::Error },
    /// Failed to create the site directory.
    SiteDirCreate { path: PathBuf, err: std::io::Error },
}
impl Display for Error {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use Error::*;
        match self {
            ConfigOpen { path, .. } => write!(f, "Failed to open config file '{}'", path.display()),
            ConfigParse { path, .. } => write!(f, "Failed to read & parse config file '{}'", path.display()),

            ConfigCreate { path, .. } => write!(f, "Failed to create default config file '{}'", path.display()),
            ConfigWrite { path, .. } => write!(f, "Failed to write to default config file '{}'", path.display()),
            NotFoundFileCreate { path, .. } => write!(f, "Failed to create default not found file '{}'", path.display()),
            SiteDirCanonicalize { path, .. } => write!(f, "Failed to canonicalize site directory path '{}'", path.display()),
            SiteDirCreate { path, .. } => write!(f, "Failed to create site directory '{}'", path.display()),
        }
    }
}
impl error::Error for Error {
    #[inline]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            ConfigOpen { err, .. } => Some(err),
            ConfigParse { err, .. } => Some(err),

            ConfigCreate { err, .. } => Some(err),
            ConfigWrite { err, .. } => Some(err),
            NotFoundFileCreate { err, .. } => Some(err),
            SiteDirCanonicalize { err, .. } => Some(err),
            SiteDirCreate { err, .. } => Some(err),
        }
    }
}





/***** LIBRARY *****/
/// Defines the context in which paths are executed.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Context {
    /// Some name for the file.
    #[serde(skip)]
    pub name:    &'static str,
    /// Some version for the file.
    #[serde(skip)]
    pub version: &'static str,

    /// The path to where the site files are located.
    pub site: PathBuf,
    /// The file sent back when a file isn't found.
    pub not_found_file: PathBuf,
}
impl Context {
    /// Constructor for the Context that loads it from a given file.
    ///
    /// # Arguments
    /// - `name`: A name that is sent back by the server in responses.
    /// - `version`: A version number that is sent back by the server in responses.
    /// - `path`: The path to the config file to load.
    ///
    /// # Returns
    /// A new Context loaded from the given `path`.
    ///
    /// # Errors
    /// This function can fail if we failed to open, read or parse the given file as YAML.
    #[inline]
    pub fn new(name: &'static str, version: &'static str, path: impl AsRef<Path>) -> Result<Self, Error> {
        let path: &Path = path.as_ref();

        // Open the file
        debug!("Reading config file at '{}'...", path.display());
        let handle: File = match File::open(path) {
            Ok(handle) => handle,
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    // Generate a default one instead
                    info!("No config file found at '{}'; generating default...", path.display());
                    let def: Self = Self { name, version, site: "./www".into(), not_found_file: "./www/not_found.html".into() };
                    match File::create(path) {
                        Ok(handle) => {
                            if let Err(err) = serde_yml::to_writer(handle, &def) {
                                return Err(Error::ConfigWrite { path: path.into(), err });
                            }
                        },
                        Err(err) => return Err(Error::ConfigCreate { path: path.into(), err }),
                    }
                    return Ok(def);
                } else {
                    return Err(Error::ConfigOpen { path: path.into(), err });
                }
            },
        };

        // Read it with serde
        let mut config: Self = match serde_yml::from_reader(handle) {
            Ok(config) => config,
            Err(err) => return Err(Error::ConfigParse { path: path.into(), err }),
        };

        // Create the www directory if it doesn't exist
        if !config.site.exists() {
            warn!("Site directory '{}' does not exist; creating it...", config.site.display());
            if let Err(err) = fs::create_dir_all(&config.site) {
                return Err(Error::SiteDirCreate { path: config.site, err });
            }
        }
        // Use the canonical version of the site
        config.site = match fs::canonicalize(&config.site) {
            Ok(path) => path,
            Err(err) => return Err(Error::SiteDirCanonicalize { path: config.site, err }),
        };

        // Create the not found file if it doesn't exist
        if !config.not_found_file.exists() {
            warn!("Not found file '{}' does not exist; creating it...", config.not_found_file.display());
            if let Err(err) = fs::write(&config.not_found_file, DEFAULT_NOT_FOUND_FILE) {
                return Err(Error::NotFoundFileCreate { path: config.not_found_file, err });
            }
        }

        // Inject the server info and return
        config.name = name;
        config.version = version;
        Ok(config)
    }
}
