use std::{collections::HashMap, time::Duration};

use anyhow::{Ok, Result};
use reqwest::{header::HeaderMap, redirect::Policy, Client, ClientBuilder, Proxy};

pub fn create_client(
    timeout: u64,
    user_agent: &str,
    redirects: bool,
    insecure: bool,
    headers: &HashMap<String, String>,
    proxy: Option<&str>,
) -> Result<Client> {
    let policy = if redirects {
        Policy::limited(10)
    } else {
        Policy::none()
    };

    let header_map: HeaderMap = headers.try_into()?;

    let client = Client::builder()
        .timeout(Duration::new(timeout, 0))
        .user_agent(user_agent)
        .danger_accept_invalid_certs(insecure)
        .default_headers(header_map)
        .redirect(policy)
        .http1_title_case_headers();

    if let Some(proxy_scheme) = proxy {
        if !proxy_scheme.is_empty() {
            let proxy_obj = Proxy::all(proxy_scheme)?;
            return Ok(client.proxy(proxy_obj).build()?);
        }
    }

    Ok(client.build()?)
}
