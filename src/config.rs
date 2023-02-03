use serde::Deserialize;
use serde::Serialize;

/// Aggregate of all configurable options for a running VM.
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub log_file: Option<String>,
    pub syslog_tag: Option<String>,

}
pub fn from_key_values<'a, T: Deserialize<'a>>(value: &'a str) -> Result<T, String> {
    serde_keyvalue::from_key_values(value).map_err(|e| e.to_string())
}
pub fn invalid_value_err<T: AsRef<str>, S: ToString>(value: T, expected: S) -> String {
    format!("invalid value {}: {}", value.as_ref(), expected.to_string())
}
