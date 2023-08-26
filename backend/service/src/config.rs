use std::{
    env,
    error::Error,
    fmt,
    ops::{Deref, DerefMut},
    str::FromStr,
};

// Helpers //

pub fn var<T: FromStr>(key: &str) -> Result<T, ConfigError> {
    env::var(key)
        .map_err(|_| ConfigError::new(key, ErrorKind::Missing))?
        .parse()
        .map_err(|_| ConfigError::new(key, ErrorKind::Invalid))
}

pub fn var_or<T: FromStr>(key: &str, default: T) -> Result<T, ConfigError> {
    var_or_else(key, || default.into())
}

pub fn var_or_else<T: FromStr, F: FnOnce() -> T>(key: &str, f: F) -> Result<T, ConfigError> {
    if let Ok(s) = env::var(key) {
        s.parse()
            .map_err(|_| ConfigError::new(key, ErrorKind::Invalid))
    } else {
        Ok(f())
    }
}

// Newtypes //

pub struct CommaVec<T: FromStr>(Vec<T>);
impl<T: FromStr> CommaVec<T> {
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }
}
impl<T: FromStr> Deref for CommaVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: FromStr> DerefMut for CommaVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T: FromStr> FromStr for CommaVec<T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s.split(',').map(|s| s.parse()).collect::<Result<_, _>>()?;
        Ok(CommaVec(inner))
    }
}

// Error //

#[derive(Clone, Debug)]
pub struct ConfigError {
    pub key: String,
    pub kind: ErrorKind,
}

#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    Missing,
    Invalid,
}

impl ConfigError {
    fn new(key: &str, kind: ErrorKind) -> ConfigError {
        ConfigError {
            key: key.to_string(),
            kind,
        }
    }
}

impl Error for ConfigError {}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::Missing => write!(f, "{} missing", self.key),
            ErrorKind::Invalid => write!(f, "{} is invalid", self.key),
        }
    }
}
