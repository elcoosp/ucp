use serde::{Deserialize, Serialize};
use ucp_core::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredRepo {
    pub full_name: String,
    pub html_url: String,
    pub spdx_id: Option<String>,
}

pub async fn find_shadcn_repos(query: &str, base_url: &str) -> Result<Vec<DiscoveredRepo>> {
    let octo = octocrab::Octocrab::builder()
        .base_uri(base_url.trim_end_matches('/'))
        .map_err(|e| ucp_core::UcpError::Http(format!("Failed to set base URI: {}", e)))?
        .build()
        .map_err(|e| ucp_core::UcpError::Http(format!("Failed to build octocrab client: {}", e)))?;

    let page = octo.search()
        .repositories(query)
        .per_page(10)
        .send()
        .await
        .map_err(|e| ucp_core::UcpError::Http(format!("GitHub API request failed: {}", e)))?;

    Ok(page.items.into_iter().map(|repo: octocrab::models::Repository| {
        let spdx_id = repo.license.as_ref().and_then(|l| {
            let id = l.spdx_id.clone();
            if id.is_empty() { None } else { Some(id) }
        });
        DiscoveredRepo {
            full_name: repo.full_name.unwrap_or_default(),
            html_url: repo.html_url.map(|u| u.to_string()).unwrap_or_default(),
            spdx_id,
        }
    }).collect())
}
