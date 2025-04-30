// This is for the progenitor generated gitlab.rs
#![allow(dead_code)]

mod api;
mod config;
mod gitlab;

use anyhow::Result;
use api::{create_gitlab_client, fetch_project_commits, fetch_project_ids, fetch_project_names};
use chrono::{prelude::*, Duration};
use jsonrpc_core::{
    futures::future::{self, FutureExt},
    Error as JsonRpcError, IoHandler, Params, Result as JsonRpcResult, Value as JsonValue,
};
use jsonrpc_stdio_server::ServerBuilder;
use log::error;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

// Global state to store configuration loaded during initialization
static GLOBAL_CONFIG: Lazy<Arc<RwLock<Option<config::AppConfig>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

// Mot plugin time entry format
#[derive(Debug, Serialize, Deserialize, Clone)]
struct TimeEntry {
    id: String,
    description: String,
    project_name: String,
    customer_name: String,
    started_at: String,
    ended_at: String,
    tags: Vec<String>,
    source: String,
    source_url: Option<String>,
    billable: bool,
}

// Structure to hold parameters for get_time_entries
#[derive(Deserialize, Debug)]
struct GetTimeEntriesParams {
    start_date: String,
    end_date: String,
}

// Structure to hold parameters for initializing
#[derive(Deserialize, Debug)]
struct InitializeParams {
    config_path: String,
}

// Enum to represent cache check result
enum CacheStatus {
    Hit(Vec<TimeEntry>),
    Miss(PathBuf), // Path to write to if fetched
    NotCacheable,
}

// --- Cache Definitions ---

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CacheKey {
    client_name: String,
    gitlab_host: String,
    author_email: String,
    start_date_rfc3339: String,
    end_date_rfc3339: String,
}

// Global in-memory cache for TimeEntry results
static IN_MEMORY_CACHE: Lazy<RwLock<HashMap<CacheKey, Arc<Vec<TimeEntry>>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

// --- End Cache Definitions ---

// --- RPC Method Implementations ---

/// Helper function to extract a string parameter from Params
fn extract_string_param(params: &Params, name: &str) -> JsonRpcResult<String> {
    params
        .clone()
        .parse::<serde_json::Map<String, serde_json::Value>>()
        .and_then(|map| {
            map.get(name)
                .ok_or_else(|| {
                    JsonRpcError::invalid_params(format!("Missing '{}' parameter", name))
                })
                .and_then(|v| {
                    v.as_str()
                        .ok_or_else(|| {
                            JsonRpcError::invalid_params(format!("'{}' must be a string", name))
                        })
                        .map(|s| s.to_string())
                })
        })
}

async fn initialize_impl(params: Params) -> JsonRpcResult<JsonValue> {
    let config_path = extract_string_param(&params, "config_path")?;
    log_to_file(&format!(
        "Received initialize request with config_path: {}",
        config_path
    ))
    .ok();

    let app_config = config::get_config();

    if app_config.clients.is_empty() {
        error!("No clients found in configuration");
        log_to_file("ERROR: No clients found in configuration").ok();
    } else {
        let client_names: Vec<String> = app_config.clients.iter().map(|c| c.name.clone()).collect();
        log_to_file(&format!(
            "Loaded configuration with clients: {}",
            client_names.join(", ")
        ))
        .ok();
    }

    if let Ok(mut config_lock) = GLOBAL_CONFIG.write() {
        *config_lock = Some(app_config);
    } else {
        error!("Failed to acquire write lock for global configuration");
        log_to_file("ERROR: Failed to acquire write lock for global configuration").ok();
    }

    Ok(serde_json::json!(true))
}

async fn get_time_entries_impl(params: Params) -> JsonRpcResult<JsonValue> {
    let start_date = extract_string_param(&params, "start_date")?;
    let end_date = extract_string_param(&params, "end_date")?;

    log_to_file(&format!(
        "Request: get_time_entries from {} to {}",
        start_date, end_date
    ))
    .ok();

    let parsed_start_date = DateTime::parse_from_rfc3339(&start_date)
        .map_err(|e| {
            let err_msg = format!("Invalid start_date format: {}. Expected RFC3339 format.", e);
            log_to_file(&err_msg).ok();
            JsonRpcError::invalid_params(err_msg)
        })?
        .with_timezone(&Local);

    let parsed_end_date = DateTime::parse_from_rfc3339(&end_date)
        .map_err(|e| {
            let err_msg = format!("Invalid end_date format: {}. Expected RFC3339 format.", e);
            log_to_file(&err_msg).ok();
            JsonRpcError::invalid_params(err_msg)
        })?
        .with_timezone(&Local);

    if parsed_end_date < parsed_start_date {
        let err_msg = format!(
            "Invalid date range: end_date {} is before start_date {}",
            end_date, start_date
        );
        error!("{}", err_msg);
        log_to_file(&err_msg).ok();
        return Err(JsonRpcError::invalid_params(err_msg));
    }

    match fetch_gitlab_activities_with_date_range(parsed_start_date, parsed_end_date).await {
        Ok(time_entries) => {
            let entry_count = time_entries.len();
            log_to_file(&format!(
                "Returning {} time entries for period {} to {}",
                entry_count, start_date, end_date
            ))
            .ok();

            // Sort the entries by started_at date, most recent first
            let mut sorted_time_entries = time_entries.clone();
            sorted_time_entries.sort_unstable_by(|a, b| {
                // Parse started_at strings into DateTime<Utc> for comparison
                // Ignore entries with invalid format during sorting (treat them as older)
                let date_a = DateTime::parse_from_rfc3339(&a.started_at).ok();
                let date_b = DateTime::parse_from_rfc3339(&b.started_at).ok();
                // Compare b to a for descending order
                date_b.cmp(&date_a)
            });

            match serde_json::to_value(sorted_time_entries) {
                Ok(json_value) => Ok(json_value),
                Err(e) => {
                    let err_msg = format!("Error serializing time entries: {}", e);
                    error!("{}", err_msg);
                    log_to_file(&err_msg).ok();
                    Err(JsonRpcError::internal_error())
                }
            }
        }
        Err(e) => {
            let err_msg = format!("Error fetching activities: {}", e);
            error!("{}", err_msg);
            log_to_file(&err_msg).ok();
            Err(JsonRpcError::internal_error())
        }
    }
}

async fn shutdown_impl(_params: Params) -> JsonRpcResult<JsonValue> {
    log_to_file("Shutting down GitLab plugin").ok();

    tokio::spawn(async {
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        std::process::exit(0);
    });

    Ok(serde_json::json!(true))
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    log_to_file("GitLab plugin started in service mode").ok();

    // Set up panic handling to prevent process termination
    std::panic::set_hook(Box::new(|panic_info| {
        error!("Plugin panic occurred: {}", panic_info);
        log_to_file(&format!("PANIC: {}", panic_info)).ok();
    }));

    let mut io = IoHandler::new();

    io.add_method("initialize", |params| initialize_impl(params).boxed());
    io.add_method("get_time_entries", |params| {
        get_time_entries_impl(params).boxed()
    });
    io.add_method("shutdown", |params| shutdown_impl(params).boxed());

    let server = ServerBuilder::new(io).build();
    server.await;

    Ok(())
}

/// Get configuration from a global state or load it if not available
fn get_global_config() -> Result<config::AppConfig> {
    if let Ok(config_lock) = GLOBAL_CONFIG.read() {
        if let Some(app_config) = config_lock.as_ref() {
            return Ok(app_config.clone());
        }
    }

    let app_config = config::get_config();

    if let Ok(mut config_lock) = GLOBAL_CONFIG.write() {
        *config_lock = Some(app_config.clone());
    }

    Ok(app_config)
}

/// Run GitLab activity fetcher with a specific date range, fetching from clients in parallel
async fn fetch_gitlab_activities_with_date_range(
    start_date: DateTime<Local>,
    end_date: DateTime<Local>,
) -> Result<Vec<TimeEntry>> {
    let app_config = get_global_config()?;

    if app_config.clients.is_empty() {
        let err_msg = "No clients found in configuration";
        error!("{}", err_msg);
        log_to_file(err_msg).ok();
        return Err(anyhow::anyhow!(err_msg));
    }

    let mut tasks = Vec::new();

    for client_config in app_config.clients {
        log_to_file(&format!(
            "Spawning fetch task for client: {}",
            client_config.name
        ))
        .ok();

        // Clone data needed for the async task
        let client_config_clone = client_config.clone();
        let start_date_clone = start_date;
        let end_date_clone = end_date;

        tasks.push(tokio::spawn(async move {
            fetch_time_entries_for_client(start_date_clone, end_date_clone, client_config_clone)
                .await
        }));
    }

    let mut all_time_entries: Vec<TimeEntry> = Vec::new();
    let results = future::join_all(tasks).await;

    for result in results {
        match result {
            Ok(Ok(client_entries)) => {
                // Task completed successfully and returned Ok(Vec<TimeEntry>)
                all_time_entries.extend(client_entries);
            }
            Ok(Err(e)) => {
                // Task completed successfully but fetch_time_entries_for_client returned an Err
                let err_msg = format!("Error in fetch task: {}", e);
                error!("{}", err_msg);
                log_to_file(&err_msg).ok();
                // Optionally, you might want to return an error here or just log and continue
            }
            Err(e) => {
                // Task failed to complete (e.g., panicked)
                let err_msg = format!("Tokio task failed: {}", e);
                error!("{}", err_msg);
                log_to_file(&err_msg).ok();
                // Optionally, return an error here
            }
        }
    }

    // Sort the entries by started_at date, most recent first
    all_time_entries.sort_unstable_by(|a, b| {
        // Parse started_at strings into DateTime<Utc> for comparison
        // Ignore entries with invalid format during sorting (treat them as older)
        let date_a = DateTime::parse_from_rfc3339(&a.started_at).ok();
        let date_b = DateTime::parse_from_rfc3339(&b.started_at).ok();
        // Compare b to a for descending order
        date_b.cmp(&date_a)
    });

    Ok(all_time_entries)
}

/// Processes commits for a single project, groups them by date, and creates TimeEntry objects.
fn process_project_commits_into_time_entries(
    project_name: &str,
    project_commits: Vec<gitlab::types::ApiEntitiesCommit>,
    client_config: &config::ClientConfig,
    start_date: DateTime<Local>,
    end_date: DateTime<Local>,
) -> Vec<TimeEntry> {
    let mut project_time_entries = Vec::new();
    // Temporary map to group activities and times by date for the current project
    let mut project_daily_data: HashMap<String, (Vec<String>, Vec<DateTime<Utc>>)> = HashMap::new();

    for commit in project_commits {
        // Filter by author email
        if commit.author_email.as_deref() == Some(&client_config.author_email) {
            // Skip merge commits
            if let Some(title) = &commit.title {
                if title.starts_with("Merge branch") {
                    continue;
                }
            }

            // Check if the authored date is within the requested range and get date string
            if let Some(authored_at) = &commit.authored_date {
                let commit_date = authored_at.with_timezone(&Local);
                let commit_date_utc = *authored_at; // Keep the original Utc DateTime
                if commit_date.date_naive() >= start_date.date_naive()
                    && commit_date.date_naive() <= end_date.date_naive()
                {
                    let date_str = commit_date.format("%Y-%m-%d").to_string();
                    // Store the full UTC DateTime
                    let (activities, times) = project_daily_data.entry(date_str).or_default();

                    if let Some(title) = &commit.title {
                        activities.push(title.clone());
                    }
                    times.push(commit_date_utc);
                }
            }
        }
    }

    // Now, create TimeEntry objects from the aggregated data for this project
    for (date_str, (mut activities, mut times)) in project_daily_data {
        if activities.is_empty() {
            continue;
        }

        activities.sort();
        activities.dedup();
        times.sort();
        times.dedup();

        // Format the min/max times into RFC3339 UTC strings
        let started_at_rfc3339 = times
            .first()
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_else(|| "".to_string()); // Handle empty times (shouldn't happen if activities exist)
        let ended_at_rfc3339 = times
            .last()
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_else(|| "".to_string()); // Handle empty times

        let time_entry = TimeEntry {
            id: format!("gitlab-{}-{}", project_name, date_str.replace("-", "")),
            description: format!("{}\n- {}", project_name, activities.join("\n- ")),
            project_name: project_name.to_string(),
            customer_name: client_config.name.clone(),
            started_at: started_at_rfc3339,
            ended_at: ended_at_rfc3339,
            tags: vec!["gitlab".to_string(), "commit".to_string()],
            source: "GitLab".to_string(),
            source_url: Some(format!("https://{}/", client_config.gitlab_host,)),
            billable: true,
        };

        project_time_entries.push(time_entry);
    }
    project_time_entries
}

/// Run GitLab activity fetcher for a specific client and date range
async fn fetch_time_entries_for_client(
    start_date: DateTime<Local>,
    end_date: DateTime<Local>,
    client_config: config::ClientConfig,
) -> Result<Vec<TimeEntry>> {
    log_to_file(&format!(
        "Using client: name='{}', gitlab_host='{}', author_email='{}'",
        client_config.name, client_config.gitlab_host, client_config.author_email
    ))
    .ok();

    let today = Local::now().date_naive();
    let is_cacheable_range = end_date.date_naive() < today;

    // Determine cache key upfront if the range is cacheable
    let cache_key = if is_cacheable_range {
        Some(CacheKey {
            client_name: client_config.name.clone(),
            gitlab_host: client_config.gitlab_host.clone(),
            author_email: client_config.author_email.clone(),
            start_date_rfc3339: start_date.to_rfc3339(),
            end_date_rfc3339: end_date.to_rfc3339(),
        })
    } else {
        None
    };

    // --- Try Cache Read --- 
    if let Some(ref key) = cache_key { // Only attempt read if cacheable
        log_to_file(&format!(
            "{}: Checking in-memory cache for past range {} - {}",
            client_config.name,
            key.start_date_rfc3339,
            key.end_date_rfc3339,
        ))
        .ok();

        let cache_read_guard = IN_MEMORY_CACHE
            .read()
            .map_err(|e| anyhow::anyhow!("Failed to acquire cache read lock: {}", e))?;

        if let Some(cached_entries_arc) = cache_read_guard.get(key) {
            log_to_file(&format!(
                "{}: In-memory cache hit. Returning {} entries.", // Simplified log
                client_config.name,
                cached_entries_arc.len(),
            ))
            .ok();
            return Ok(cached_entries_arc.to_vec()); // Return early on hit
        } else {
            log_to_file(&format!(
                "{}: In-memory cache miss for range {} - {}.",
                client_config.name,
                key.start_date_rfc3339,
                key.end_date_rfc3339
            ))
            .ok();
            // Proceed to fetch data if cache missed
        }
    } else { // Not cacheable range
        log_to_file(&format!(
            "{}: Date range {} - {} not cacheable. Skipping cache read.",
            client_config.name,
            start_date.to_rfc3339(),
            end_date.to_rfc3339()
        ))
        .ok();
    }

    // --- Fetch Data (if cache miss or not cacheable) ---
    let fetched_entries = fetch_and_process_api_data(&client_config, start_date, end_date).await?;

    // --- Try Cache Write (only if it was a cacheable range) ---
    if let Some(key) = cache_key { // Use the key created earlier if it exists
        log_to_file(&format!(
            "{}: Attempting to write {} entries to in-memory cache for range {} - {}.",
            client_config.name,
            fetched_entries.len(),
            key.start_date_rfc3339,
            key.end_date_rfc3339,
        ))
        .ok();

        match IN_MEMORY_CACHE.write() {
            Ok(mut cache_write_guard) => {
                cache_write_guard.insert(key, Arc::new(fetched_entries.clone())); // Insert Arc of cloned data
                log_to_file(&format!(
                    "{}: Successfully wrote {} entries to in-memory cache.",
                    client_config.name,
                    fetched_entries.len(),
                ))
                .ok();
            }
            Err(e) => {
                // Log error but don't fail the overall operation
                log_to_file(&format!(
                    "{}: Failed to acquire cache write lock: {}. Skipping cache write.",
                    client_config.name, e
                ))
                .ok();
            }
        }
    }

    Ok(fetched_entries) // Return the fetched data
}

/// Fetches and processes data from the GitLab API for a given client and date range.
async fn fetch_and_process_api_data(
    client_config: &config::ClientConfig,
    start_date: DateTime<Local>,
    end_date: DateTime<Local>,
) -> Result<Vec<TimeEntry>> {
    log_to_file(&format!(
        "{}: Fetching from API for range {} - {}",
        client_config.name,
        start_date.date_naive(),
        end_date.date_naive()
    ))
    .ok();

    let gitlab_client = create_gitlab_client(&client_config);
    log_to_file(&format!(
        "{}: Using GitLab endpoint: {}",
        client_config.name, client_config.gitlab_host
    ))
    .ok();

    log_to_file(&format!(
        "{}: Fetching events with adjusted date range: before {} and after {}",
        client_config.name,
        (start_date.date_naive() - Duration::days(1)),
        (end_date.date_naive() + Duration::days(1))
    ))
    .ok();

    // Take care of api edge case weirdness
    let end_date_naive = end_date.date_naive() + Duration::days(1);
    let start_date_naive = start_date.date_naive() - Duration::days(1);
    let project_ids = fetch_project_ids(&gitlab_client, start_date_naive, end_date_naive).await?;

    log_to_file(&format!(
        "{}: Fetched {} project ids",
        client_config.name,
        project_ids.len()
    ))
    .ok();

    let projects_map = fetch_project_names(&gitlab_client, &project_ids).await?;
    let mut sorted_projects: Vec<_> = projects_map.iter().collect();
    sorted_projects.sort_by(|a, b| a.1.cmp(b.1));

    let mut client_time_entries: Vec<TimeEntry> = Vec::new();

    for (project_id, project_name) in sorted_projects {
        log_to_file(&format!(
            "{}: Fetching commits for project: {}",
            client_config.name, project_name
        ))
        .ok();

        let project_commits = fetch_project_commits(
            &gitlab_client,
            project_id,
            start_date,
            end_date, // Use original dates for commits
        )
        .await?;

        log_to_file(&format!(
            "{}: Processing {} commits for project: {}",
            client_config.name,
            project_commits.len(),
            project_name
        ))
        .ok();

        let project_entries = process_project_commits_into_time_entries(
            project_name,
            project_commits,
            &client_config,
            start_date,
            end_date,
        );
        client_time_entries.extend(project_entries);
    }

    log_to_file(&format!(
        "{}: API fetch yielded {} time entries",
        client_config.name,
        client_time_entries.len()
    ))
    .ok();

    Ok(client_time_entries)
}

/// Get the path to the log file in the same directory as the executable
pub(crate) fn get_log_file_path() -> PathBuf {
    let exe_path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    let exe_dir = exe_path
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."));
    exe_dir.join("mot-plugin-gitlab.log")
}

/// Helper function to log important events to a file, ONLY if log level is Debug or higher
pub(crate) fn log_to_file(message: &str) -> Result<()> {
    // Check if the maximum enabled log level is Debug or higher.
    // This depends on env_logger::init() having been called already.
    if log::max_level() >= log::LevelFilter::Debug {
        let log_file_path = get_log_file_path();
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // Attempt to open/create and append to the log file
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file_path)
            .map_err(|e| {
                // Use the standard error log if we can't write to the file
                error!(
                    "Failed to open or create log file at {}: {}",
                    log_file_path.display(),
                    e
                );
                anyhow::Error::from(e) // Convert the error for the function signature
            })?;

        // Write the message
        if let Err(e) = writeln!(file, "[{}] {}", timestamp, message) {
            error!(
                "Failed to write to log file at {}: {}",
                log_file_path.display(),
                e
            );
            return Err(anyhow::Error::from(e));
        }
    }
    // If log level is not Debug or higher, do nothing.
    Ok(())
}
