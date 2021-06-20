use crate::{de::CheckUrlResult, utils::http_post, PhishtankClient, PhishtankResult};

impl PhishtankClient {
    pub fn lookup(&self, url: &str) -> PhishtankResult<CheckUrlResult> {
        //! Lookup the given url to see if it exists in phishtank db
        //!
        //! ## Example Usage
        //! ```rust
        //! use phishtank::PhishtankClient;
        //!
        //! let client = PhishtankClient::new("Your API Key");
        //! let res = client.lookup("https://example.com/");
        //! ```
        let encoded_url = base64::encode(url);
        let form_data = &[
            ("format", "json"),
            ("url", &encoded_url),
            ("app_key", &self.api_key),
        ];
        http_post(&self.user_agent, &self.check_endpoint, form_data)
    }
}
