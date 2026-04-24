use octocrab::params;
use serde::{Deserialize, Serialize};
use ucp_core::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredRepo {
    pub full_name: String,
    pub html_url: String,
    pub spdx_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct RepoItem {
    full_name: String,
    html_url: String,
    license: Option<LicenseInfo>,
}

#[derive(Debug, Clone, Deserialize)]
struct LicenseInfo {
    spdx_id: String,
}

pub async fn find_shadcn_repos(query: &str, base_url: &str) -> Result<Vec<DiscoveredRepo>> {
    let octo = octocrab::Octocrab::builder()
        .base_uri(base_url.trim_end_matches('/'))
        .build()
        .map_err(|e| ucp_core::UcpError::Parsing(e.to_string()))?;

    let page = octo.search(query)
        .per_page(10)
        .send()
        .await
        .map_err(|e| ucp_core::UcpError::Parsing(e.to_string()))?;

    let items: Vec<RepoItem> = page.items;
    Ok(items.into_iter().map(|i| DiscoveredRepo {
        full_name: i.full_name,
        html_url: i.html_url.to_string(),
        spdx_id: i.license.as_ref().map(|l| l.spdx_id.clone()),
    }).collect())
}
