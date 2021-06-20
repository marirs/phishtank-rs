/// Phishtank API
/// Simple API to acces the Phishtank API to download the database or lookup for url
/// ## Example Usage
///
/// ```rust
/// use phishtank::PhishtankClient;
///
/// let client = PhishtankClient::new("YOUR API KEY");
/// ```
mod de;
mod download;
mod utils;

pub mod error;
use error::PhishtankError;
pub type PhishtankResult<T> = Result<T, PhishtankError>;

static DEFAULT_USER_AGENT: &str = "rust-client/phishtank+https://github.com/marirs/phishtank-rs";

#[derive(Clone)]
pub struct PhishtankClient {
    api_key: String,
    data_endpoint: String,
    check_endpoint: String,
    user_agent: String,
}

impl PhishtankClient {
    pub fn new(api_key: &str) -> Self {
        //! Creates a new Phishtank API Client to access the Phishtank API endpoint
        //!
        //! ## Example usage
        //! ```rust
        //! use phishtank::PhishtankClient;
        //!
        //! let client = PhishtankClient::new("YOUR API KEY");
        //! ```
        PhishtankClient {
            api_key: api_key.into(),
            data_endpoint: "https://data.phishtank.com/data".into(),
            check_endpoint: "https://checkurl.phishtank.com/checkurl".into(),
            user_agent: DEFAULT_USER_AGENT.into(),
        }
    }

    /// Sets a new user-agent that from the default
    pub fn user_agent(mut self, user_agent: &str) -> PhishtankClient {
        self.user_agent = user_agent.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::PhishtankClient;

    #[test]
    fn test_vtclient() {
        let client = PhishtankClient::new("someapikey");
        assert_eq!(client.api_key, "someapikey");
        assert_eq!(client.data_endpoint, "https://data.phishtank.com/data");
        assert_eq!(
            client.check_endpoint,
            "https://checkurl.phishtank.com/checkurl"
        );
    }
}
