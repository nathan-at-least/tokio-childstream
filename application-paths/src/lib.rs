use std::path::PathBuf;

/// Provide initialized standardized paths based on an app name
///
/// When methods return a directory path, they guarantee that path already exists. This is not the case for file paths.
#[derive(Debug)]
pub struct ApplicationPaths {
    appname: String,
}

impl<T> From<T> for ApplicationPaths
where
    String: From<T>,
{
    fn from(appname: T) -> Self {
        ApplicationPaths {
            appname: String::from(appname),
        }
    }
}

/// Construct [ApplicationPaths] from the crate's `CARGO_PKG_NAME` environment variable
///
/// Note if this is used from a library crate, then all dependent applications on the same system will share these paths.
#[macro_export]
macro_rules! application_paths {
    () => {
        $crate::ApplicationPaths::from(env!("CARGO_PKG_NAME"))
    };
}

macro_rules! get_dir {
    ( $name:ident ) => {
        dirs::$name().ok_or_else(|| {
            use std::io::{Error, ErrorKind::Other};

            Error::new(Other, format!("`dirs::{}` undefined", stringify!($name)))
        })
    };
}

impl ApplicationPaths {
    /// The app name
    pub fn app_name(&self) -> &str {
        &self.appname
    }

    /// The app-specific config directory
    pub fn config_dir(&self) -> std::io::Result<PathBuf> {
        self.cratify(get_dir!(config_local_dir)?, None)
    }

    /// Get the main/default config file for the app with the given extension
    pub fn config_file(&self, extension: &str) -> std::io::Result<PathBuf> {
        let mut pb = self.config_dir()?;
        pb.push(format!("config.{extension}"));
        Ok(pb)
    }

    /// The app-specific log directory
    pub fn logs_dir(&self) -> std::io::Result<PathBuf> {
        self.cratify(get_dir!(data_local_dir)?, Some("logs"))
    }

    /// Get the main/default log file for the app
    pub fn log_file(&self) -> std::io::Result<PathBuf> {
        let mut pb = self.logs_dir()?;
        pb.push("main.log");
        Ok(pb)
    }

    /// The app-specific data directory
    pub fn data_dir(&self) -> std::io::Result<PathBuf> {
        self.cratify(get_dir!(data_local_dir)?, Some("data"))
    }

    fn cratify(&self, mut d: PathBuf, subdir: Option<&str>) -> std::io::Result<PathBuf> {
        d.push(self.app_name());
        if let Some(subdir) = subdir {
            d.push(subdir);
        }
        std::fs::create_dir_all(&d)?;
        Ok(d)
    }
}

#[cfg(test)]
mod tests;
