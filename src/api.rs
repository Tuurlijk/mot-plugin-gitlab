use crate::config::ClientConfig;
use crate::gitlab::{self, types::ApiEntitiesCommit};
use crate::log_to_file;
use anyhow::Result;
use chrono::{DateTime, Local, NaiveDate};
use std::collections::{HashMap, HashSet};

/// Creates a GitLab client with proper authentication
pub fn create_gitlab_client(client_config: &ClientConfig) -> gitlab::Client {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {}", &client_config.gitlab_token)
            .parse()
            .unwrap(),
    );

    gitlab::Client::new_with_client(
        format!("https://{}", &client_config.gitlab_host).as_str(),
        reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap(),
    )
}

/// Fetches project IDs from GitLab events within the specified date range
pub async fn fetch_project_ids(
    gitlab_client: &gitlab::Client,
    start_date_naive: NaiveDate,
    end_date_naive: NaiveDate,
) -> Result<HashSet<i64>> {
    let mut project_ids: HashSet<i64> = HashSet::new();
    const PER_PAGE: i32 = 100;
    let mut current_page: i32;
    let mut next_page: Option<i32> = Some(1);

    while let Some(page) = next_page {
        current_page = page;
        let response_value = gitlab_client
            .get_api_v4_events()
            .before(end_date_naive)
            .after(start_date_naive)
            .per_page(PER_PAGE)
            .page(current_page)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed project ids request: {}", e))?;

        let headers = response_value.headers().clone();
        let events = response_value.into_inner();

        for event in events.iter() {
            if event.push_data.is_some() {
                if let Some(project_id) = event.project_id {
                    project_ids.insert(project_id);
                }
            }
        }
        log_to_file(&format!("Headers: {:?}", headers)).ok();

        next_page = headers
            .get("X-Next-Page")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse::<i32>().ok());
    }

    Ok(project_ids)
}

/// Fetches project names for the provided project IDs
pub async fn fetch_project_names(
    gitlab_client: &gitlab::Client,
    project_ids: &HashSet<i64>,
) -> Result<HashMap<i64, String>> {
    let mut projects_map: HashMap<i64, String> = HashMap::new();

    for project_id in project_ids {
        match gitlab_client
            .get_api_v4_projects_id()
            .id(project_id.to_string())
            .send()
            .await
        {
            Ok(project) => {
                if let Some(id) = project.id {
                    projects_map.insert(id, project.name.clone().unwrap_or_default());
                }
            }
            Err(e) => {
                println!("Error fetching project {}: {:?}", project_id, e);
            }
        }
    }

    Ok(projects_map)
}

/// Fetches all commits for a project within the specified date range
pub async fn fetch_project_commits(
    gitlab_client: &gitlab::Client,
    project_id: &i64,
    since: DateTime<Local>,
    until: DateTime<Local>,
) -> Result<Vec<ApiEntitiesCommit>> {
    let mut project_commits: Vec<ApiEntitiesCommit> = Vec::new();
    const PER_PAGE: i32 = 100;
    let mut current_page: i32;
    let mut next_page: Option<i32> = Some(1);

    while let Some(page) = next_page {
        current_page = page;
        let response_value = gitlab_client
            .get_api_v4_projects_id_repository_commits()
            .id(project_id.to_string())
            .since(since)
            .until(until)
            .all(true)
            .first_parent(true)
            .per_page(PER_PAGE)
            .page(current_page)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed project ids request: {}", e))?;

        let headers = response_value.headers().clone();
        let commits = response_value.into_inner();

        for commit in commits.iter() {
            project_commits.push(commit.clone());
        }
        log_to_file(&format!("{}: Headers: {:?}", project_id, headers)).ok();

        next_page = headers
            .get("X-Next-Page")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse::<i32>().ok());
    }

    Ok(project_commits)
}
