use chrono::{DateTime, Utc};
use reqwest::Response;
use reqwest_middleware::ClientWithMiddleware as HttpClient;
use serde_derive::{Deserialize, Serialize};

use super::{ensure_status_success, Result, BASE_URL};
use crate::errors::TasksError::InvalidArgument;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tasklists {
    /// Type of the resource. This is always "tasks#taskLists".
    pub kind: String,

    /// ETag of the resource.
    pub etag: String,

    /// Token that can be used to request the next page of this result.
    pub next_page_token: Option<String>,

    /// Collection of task lists.
    pub items: Vec<Tasklist>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tasklist {
    /// Type of the resource. This is always "tasks#taskList".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Task list identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// ETag of the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// Title of the task list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// URL pointing to this task list. Used to retrieve, update, or delete this task list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<DateTime<Utc>>,

    /// Last modification time of the task list (as a RFC 3339 timestamp).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
}

#[derive(Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListOptions {
    /// Maximum number of task lists returned on one page.
    /// Optional. The default is 20 (max allowed: 100).
    pub max_results: Option<u64>,

    /// Token specifying the result page to return. Optional.
    pub page_token: Option<String>,
}

// Returns all the authenticated user's task lists.
pub(crate) async fn list(client: &HttpClient, opt: Option<ListOptions>) -> Result<Tasklists> {
    let url = format!("{base_url}/users/@me/lists", base_url = BASE_URL);
    let mut builder = client.get(url.as_str());

    if let Some(query_params) = opt {
        builder = builder.query(&query_params);
    }

    let resp = builder.send().await?;

    let resp = ensure_status_success(resp).await?;
    Ok(resp.json::<Tasklists>().await?)
}

// Returns the authenticated user's specified task list.
pub(crate) async fn get(client: &HttpClient, id: &str) -> Result<Tasklist> {
    let url = format!(
        "{base_url}/users/@me/lists/{tasklist_id}",
        base_url = BASE_URL,
        tasklist_id = id
    );
    let resp = client.get(url.as_str()).send().await?;
    handle_response_tasklist(resp).await
}

// Creates a new task list and adds it to the authenticated user's task lists.
pub(crate) async fn insert(client: &HttpClient, b: Tasklist) -> Result<Tasklist> {
    let url = format!("{base_url}/users/@me/lists", base_url = BASE_URL);
    let resp = client
        .post(url.as_str())
        .body(serde_json::to_vec(&b)?)
        .send()
        .await?;

    handle_response_tasklist(resp).await
}

// Updates the authenticated user's specified task list.
pub(crate) async fn update(client: &HttpClient, v: Tasklist) -> Result<Tasklist> {
    let tasklist_id = match v.id.as_ref() {
        Some(id) => id,
        None => return Err(InvalidArgument("tasklist id cannot be None".to_owned())),
    };

    let url = format!(
        "{base_url}/users/@me/lists/{tasklist_id}",
        base_url = BASE_URL,
        tasklist_id = tasklist_id
    );
    let resp = client
        .put(url.as_str())
        .body(serde_json::to_vec(&v)?)
        .send()
        .await?;

    handle_response_tasklist(resp).await
}

// Deletes the authenticated user's specified task list.
pub(crate) async fn delete(client: &HttpClient, id: &str) -> Result<()> {
    let url = format!(
        "{base_url}/users/@me/lists/{tasklist_id}",
        base_url = BASE_URL,
        tasklist_id = id
    );
    let resp = client.delete(url.as_str()).send().await?;

    ensure_status_success(resp).await?;
    Ok(())
}

// Updates the authenticated user's specified task list. This method supports patch semantics.
pub(crate) async fn patch(client: &HttpClient, tasklist_id: &str, v: Tasklist) -> Result<Tasklist> {
    let url = format!(
        "{base_url}/users/@me/lists/{tasklist_id}",
        base_url = BASE_URL,
        tasklist_id = tasklist_id
    );
    let resp = client
        .patch(url.as_str())
        .body(serde_json::to_vec(&v)?)
        .send()
        .await?;

    handle_response_tasklist(resp).await
}

async fn handle_response_tasklist(resp: Response) -> Result<Tasklist> {
    let resp = ensure_status_success(resp).await?;
    Ok(resp.json::<Tasklist>().await?)
}
