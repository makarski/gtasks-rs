use anyhow::{bail, ensure};
use chrono::{DateTime, Utc};
use reqwest::{Client, Response};
use serde_derive::{Deserialize, Serialize};

use super::{Result, BASE_URL};

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
    max_results: Option<u64>,

    /// Token specifying the result page to return. Optional.
    page_token: Option<String>,
}

// Returns all the authenticated user's task lists.
pub fn list(client: &reqwest::Client, opt: Option<ListOptions>) -> Result<Tasklists> {
    let url = format!("{base_url}/users/@me/lists", base_url = BASE_URL);
    let mut builder = client.get(url.as_str());

    if let Some(query_params) = opt {
        builder = builder.query(&query_params);
    }

    let mut resp = builder.send()?;

    ensure!(resp.status().is_success(), resp.text()?);
    Ok(resp.json::<Tasklists>()?)
}

// Returns the authenticated user's specified task list.
pub fn get(client: &reqwest::Client, id: &str) -> Result<Tasklist> {
    let url = format!(
        "{base_url}/users/@me/lists/{tasklist_id}",
        base_url = BASE_URL,
        tasklist_id = id
    );
    let resp = client.get(url.as_str()).send()?;
    handle_response_tasklist(resp)
}

// Creates a new task list and adds it to the authenticated user's task lists.
pub fn insert(client: &reqwest::Client, b: Tasklist) -> Result<Tasklist> {
    let url = format!("{base_url}/users/@me/lists", base_url = BASE_URL);
    let resp = client
        .post(url.as_str())
        .body(serde_json::to_vec(&b)?)
        .send()?;

    handle_response_tasklist(resp)
}

// Updates the authenticated user's specified task list.
pub fn update(client: &reqwest::Client, v: Tasklist) -> Result<Tasklist> {
    let tasklist_id = match v.id.as_ref() {
        Some(id) => id,
        None => bail!("tasklist id cannot be None"),
    };

    let url = format!(
        "{base_url}/users/@me/lists/{tasklist_id}",
        base_url = BASE_URL,
        tasklist_id = tasklist_id
    );
    let resp = client
        .put(url.as_str())
        .body(serde_json::to_vec(&v)?)
        .send()?;

    handle_response_tasklist(resp)
}

// Deletes the authenticated user's specified task list.
pub fn delete(client: &reqwest::Client, id: &str) -> Result<()> {
    let url = format!(
        "{base_url}/users/@me/lists/{tasklist_id}",
        base_url = BASE_URL,
        tasklist_id = id
    );
    let mut resp = client.delete(url.as_str()).send()?;

    ensure!(resp.status().is_success(), resp.text()?);
    Ok(())
}

// Updates the authenticated user's specified task list. This method supports patch semantics.
pub fn patch(client: &Client, tasklist_id: &str, v: Tasklist) -> Result<Tasklist> {
    let url = format!(
        "{base_url}/users/@me/lists/{tasklist_id}",
        base_url = BASE_URL,
        tasklist_id = tasklist_id
    );
    let resp = client
        .patch(url.as_str())
        .body(serde_json::to_vec(&v)?)
        .send()?;

    handle_response_tasklist(resp)
}

fn handle_response_tasklist(mut resp: Response) -> Result<Tasklist> {
    ensure!(resp.status().is_success(), resp.text()?);
    Ok(resp.json::<Tasklist>()?)
}
