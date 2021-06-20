use crate::PhishtankResult;
use reqwest::{
    blocking::{Client, Response},
    StatusCode,
};
use serde::de::DeserializeOwned;

#[inline]
fn process_resp_bz<T>(resp: Response) -> PhishtankResult<T>
where
    T: DeserializeOwned,
{
    let status = resp.status();

    match status {
        StatusCode::OK => {
            let read = bzip2::read::BzDecoder::new(resp);
            Ok(serde_json::from_reader(read)?) // 200
        }
        _ => Err(status.into()),
    }
}

/// GET from a URL with bzip response
pub(crate) fn http_get_bz<T>(user_agent: &str, url: &str) -> PhishtankResult<T>
where
    T: DeserializeOwned,
{
    let client = Client::builder().user_agent(user_agent).build()?;
    let resp = client.get(url).send()?;
    process_resp_bz(resp)
}
