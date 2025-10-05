use serde::{Deserialize, Serialize};

/// Default Facebook Graph API version
///
/// According to Facebook's versioning policy, each API version is supported for at least 2 years.
/// v23.0 was released in 2024 and will be supported until at least 2026.
///
/// See: https://developers.facebook.com/docs/graph-api/guides/versioning
pub const DEFAULT_API_VERSION: &str = "v23.0";

/// Validates if an API version string is in the correct format
///
/// # Arguments
/// * `version` - A version string like "v23.0", "v22.0", etc.
///
/// # Returns
/// * `true` if the version format is valid (vXX.X or vX.X)
/// * `false` if the format is invalid
///
/// # Examples
/// ```
/// use facebook_api_rs::prelude::is_valid_api_version;
///
/// assert!(is_valid_api_version("v23.0"));
/// assert!(is_valid_api_version("v22.0"));
/// assert!(!is_valid_api_version("23.0"));  // Missing 'v' prefix
/// assert!(!is_valid_api_version("v23"));    // Missing minor version
/// ```
pub fn is_valid_api_version(version: &str) -> bool {
    // Check format: v{major}.{minor} where major and minor are numbers
    if !version.starts_with('v') {
        return false;
    }

    let version_number = &version[1..];
    let parts: Vec<&str> = version_number.split('.').collect();

    if parts.len() != 2 {
        return false;
    }

    // Check if both parts are valid numbers
    parts[0].parse::<u32>().is_ok() && parts[1].parse::<u32>().is_ok()
}

/// A struct which describes the parameters used to construction of Facebook
/// login
#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    /// The Facebook url preamble for the oath dialog.
    ///
    /// This parameters is set to default value to
    /// https://www.facebook.com/v23.0/dialog/oauth?
    pub facebook_oath_url: String,

    /// The ID of your app, found in your app's dashboard.
    pub client_id: String,

    /// The URL that you want to redirect the person logging in back to.
    pub redirect_uri: String,
}

impl Config {
    pub fn new(client_id: String, redirect_uri: String) -> Self {
        Config {
            facebook_oath_url: "https://www.facebook.com/v23.0/dialog/oauth?".to_owned(),
            client_id,
            redirect_uri,
        }
    }

    /// Create a new Config with a custom API version
    ///
    /// # Arguments
    /// * `client_id` - Your Facebook App ID
    /// * `redirect_uri` - The redirect URI for OAuth flow
    /// * `api_version` - The Facebook Graph API version (e.g., "v23.0", "v22.0")
    ///
    /// # Notes
    /// Facebook's versioning policy guarantees each version for at least 2 years.
    /// Using older versions (e.g., v11.0, v13.0) may result in deprecated endpoints.
    ///
    /// # Panics
    /// Panics if the API version format is invalid. Use `is_valid_api_version()` to check first.
    ///
    /// # Example
    /// ```
    /// use facebook_api_rs::prelude::Config;
    ///
    /// let config = Config::new_with_version(
    ///     "your_app_id".to_string(),
    ///     "https://yourapp.com/callback".to_string(),
    ///     "v23.0".to_string()
    /// );
    /// ```
    pub fn new_with_version(client_id: String, redirect_uri: String, api_version: String) -> Self {
        if !is_valid_api_version(&api_version) {
            panic!(
                "Invalid API version format: '{}'. Expected format: 'vXX.X' (e.g., 'v23.0')",
                api_version
            );
        }

        let facebook_oath_url = format!("https://www.facebook.com/{}/dialog/oauth?", api_version);
        Config {
            facebook_oath_url,
            client_id,
            redirect_uri,
        }
    }

    pub fn facebook_oath_url(&self) -> &str {
        &self.facebook_oath_url
    }

    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    pub fn redirect_uri(&self) -> &str {
        &self.redirect_uri
    }

    /// Extract the API version from the OAuth URL
    ///
    /// # Returns
    /// The API version string (e.g., "v23.0") or None if not found
    ///
    /// # Example
    /// ```
    /// use facebook_api_rs::prelude::Config;
    ///
    /// let config = Config::new("app_id".to_string(), "redirect".to_string());
    /// assert_eq!(config.api_version(), Some("v23.0"));
    /// ```
    pub fn api_version(&self) -> Option<&str> {
        // Extract version from URL: https://www.facebook.com/v23.0/dialog/oauth?
        self.facebook_oath_url
            .split('/')
            .find(|s| s.starts_with('v') && s.contains('.'))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_api_version() {
        assert!(is_valid_api_version("v23.0"));
        assert!(is_valid_api_version("v22.0"));
        assert!(is_valid_api_version("v1.0"));
        assert!(is_valid_api_version("v100.99"));

        assert!(!is_valid_api_version("23.0")); // Missing v
        assert!(!is_valid_api_version("v23")); // Missing minor
        assert!(!is_valid_api_version("v23.")); // Incomplete
        assert!(!is_valid_api_version("v23.0.1")); // Too many parts
        assert!(!is_valid_api_version("vAB.C")); // Not numbers
    }

    #[test]
    fn test_config_default_version() {
        let config = Config::new("test_id".to_string(), "http://test".to_string());
        assert_eq!(config.api_version(), Some("v23.0"));
    }

    #[test]
    fn test_config_custom_version() {
        let config = Config::new_with_version(
            "test_id".to_string(),
            "http://test".to_string(),
            "v22.0".to_string(),
        );
        assert_eq!(config.api_version(), Some("v22.0"));
        assert!(config.facebook_oath_url().contains("v22.0"));
    }

    #[test]
    #[should_panic(expected = "Invalid API version format")]
    fn test_config_invalid_version() {
        Config::new_with_version(
            "test_id".to_string(),
            "http://test".to_string(),
            "23.0".to_string(), // Missing 'v' prefix
        );
    }
}
