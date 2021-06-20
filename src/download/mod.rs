use crate::{de::PhishingDB, utils::http_get_bz, PhishtankClient, PhishtankResult};

impl PhishtankClient {
    pub fn download_db(&self) -> PhishtankResult<Vec<PhishingDB>> {
        //! Get the database for saving & local lookup's
        //!
        //! ## Example Usage
        //! ```rust
        //! use phishtank::PhishtankClient;
        //!
        //! let client = PhishtankClient::new("Your API Key");
        //! let res = client.download_db();
        //! ```
        let endpoint_url = format!(
            "{}/{}/online-valid.json.bz2",
            &self.data_endpoint, self.api_key
        );
        http_get_bz(&self.user_agent, &endpoint_url)
    }
}
