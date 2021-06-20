use crate::{PhishtankResult, PhishtankError};
use reqwest::{
    blocking::{Client, Response},
    StatusCode,
};
use serde::de::DeserializeOwned;

#[inline]
fn process_resp<T>(resp: Response) -> PhishtankResult<T>
where
    T: DeserializeOwned,
{
    let status = resp.status();

    match status {
        StatusCode::OK => {
            let resp_text = resp.text().unwrap();
            // let resp_json = resp.json();
            if resp_text.contains("You must supply a URL to use this function") {
                Err(PhishtankError::InvalidUrl)
            } else {
                Ok(serde_json::from_str(&resp_text)?)
            }
        }, // 200
        _ => Err(status.into()),
    }
}

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

// POST to a URL
pub(crate) fn http_post<T>(
    user_agent: &str,
    url: &str,
    form_data: &[(&str, &str)],
) -> PhishtankResult<T>
where
    T: DeserializeOwned,
{
    let client = Client::builder().user_agent(user_agent).build()?;
    let resp = client
        .post(url)
        .header("Accept", "application/json")
        .form(form_data)
        .send()?;
    process_resp(resp)
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
