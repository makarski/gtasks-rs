use chrono::{DateTime, Utc};
use reqwest::Client;

use super::{io_invalid_input_err, io_other_err, Result, BASE_URL};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tasks {
    /// Type of the resource. This is always "tasks#tasks".
    pub kind: String,

    /// ETag of the resource.
    pub etag: String,

    /// Token used to access the next page of this result.
    pub next_page_token: Option<String>,

    /// Collection of tasks.
    pub items: Vec<Task>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    /// Type of the resource. This is always "tasks#task".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Task identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// ETag of the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    /// Title of the task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Last modification time of the task (as a RFC 3339 timestamp).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<DateTime<Utc>>,

    /// URL pointing to this task. Used to retrieve, update, or delete this task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,

    // Parent task identifier.
    // This field is omitted if it is a top-level task.
    // This field is read-only. Use the "move" method to move the task under a different parent or to the top level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    /// String indicating the position of the task among its sibling tasks under the same parent task or at the top level.
    /// If this string is greater than another task's corresponding position string according to lexicographical ordering,
    /// the task is positioned after the other task under the same parent task (or at the top level).
    ///
    /// This field is read-only.
    /// Use the "move" method to move the task to another position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,

    /// Notes describing the task. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    /// Status of the task. This is either "needsAction" or "completed".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskStatus>,

    /// Due date of the task (as a RFC 3339 timestamp). Optional.
    /// The due date only records date information; the time portion of the timestamp is discarded when setting the due date.
    /// It isn't possible to read or write the time that a task is due via the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<DateTime<Utc>>,

    /// Completion date of the task (as a RFC 3339 timestamp).
    /// This field is omitted if the task has not been completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<DateTime<Utc>>,

    /// Flag indicating whether the task has been deleted. The default if False.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,

    /// Flag indicating whether the task is hidden.
    /// This is the case if the task had been marked completed when the task list was last cleared.
    /// The default is False. This field is read-only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,

    /// Collection of links. This collection is read-only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<TaskLink>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TaskLink {
    #[serde(rename = "type")]
    /// Type of the link, e.g. "email".
    pub task_type: String,

    /// The description. In HTML speak: Everything between <a> and </a>.
    pub description: String,

    /// The URL.
    pub link: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TaskStatus {
    NeedsAction,
    Completed,
}

#[derive(Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListOptions {
    /// Upper bound for a task's completion date (as a RFC 3339 timestamp) to filter by.
    /// Optional. The default is not to filter by completion date.
    pub completed_max: Option<String>,

    /// Lower bound for a task's completion date (as a RFC 3339 timestamp) to filter by.
    /// Optional. The default is not to filter by completion date.
    pub completed_min: Option<String>,

    /// Upper bound for a task's due date (as a RFC 3339 timestamp) to filter by.
    /// Optional. The default is not to filter by due date.
    pub due_max: Option<DateTime<Utc>>,

    /// Lower bound for a task's due date (as a RFC 3339 timestamp) to filter by.
    /// Optional. The default is not to filter by due date.
    pub due_min: Option<DateTime<Utc>>,

    /// Maximum number of task lists returned on one page.
    /// Optional. The default is 20 (max allowed: 100).
    pub max_results: Option<u64>,

    /// Token specifying the result page to return. Optional.
    pub page_token: Option<String>,

    /// Flag indicating whether completed tasks are returned in the result.
    /// Optional. The default is True.
    pub show_completed: Option<bool>,

    /// Flag indicating whether deleted tasks are returned in the result.
    /// Optional. The default is False.
    pub show_deleted: Option<bool>,

    /// Flag indicating whether hidden tasks are returned in the result.
    /// Optional. The default is False.
    pub show_hidden: Option<bool>,

    /// Lower bound for a task's last modification time (as a RFC 3339 timestamp) to filter by.
    /// Optional. The default is not to filter by last modification time.
    pub updated_min: Option<DateTime<Utc>>,
}

// Returns all tasks in the specified task list.
pub fn list(client: &Client, tasklist_id: &str, opt: Option<ListOptions>) -> Result<Tasks> {
    let url = format!(
        "{base_url}/lists/{tasklist_id}/tasks",
        base_url = BASE_URL,
        tasklist_id = tasklist_id
    );
    let mut builder = client.get(url.as_str());

    if let Some(q_opt) = opt {
        builder = builder.query(&q_opt);
    }

    let mut resp = builder.send()?;

    match resp.status().is_success() {
        true => Ok(resp.json::<Tasks>()?),
        _ => Err(io_other_err(resp.text()?)),
    }
}

// Returns the specified task.
pub fn get(client: &Client, tasklist_id: &str, task_id: &str) -> Result<Task> {
    let url = format!(
        "{base_url}/lists/{tasklist_id}/tasks/{task_id}",
        base_url = BASE_URL,
        tasklist_id = tasklist_id,
        task_id = task_id
    );

    let mut resp = client.get(url.as_str()).send()?;

    match resp.status().is_success() {
        true => Ok(resp.json::<Task>()?),
        _ => Err(io_other_err(resp.text()?)),
    }
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InsertOptions {
    /// Parent task identifier.
    /// If the task is created at the top level, this parameter is omitted. Optional.
    pub parent: Option<String>,

    /// Previous sibling task identifier.
    /// If the task is created at the first position among its siblings, this parameter is omitted. Optional.
    pub previous: Option<String>,
}

// Creates a new task on the specified task list.
pub fn insert(
    client: &Client,
    tasklist_id: &str,
    v: Task,
    opts: Option<InsertOptions>,
) -> Result<Task> {
    let url = format!(
        "{base_url}/lists/{tasklist_id}/tasks",
        base_url = BASE_URL,
        tasklist_id = tasklist_id,
    );

    let mut builder = client.post(url.as_str()).body(serde_json::to_vec(&v)?);

    if let Some(query_params) = opts {
        builder = builder.query(&query_params);
    }

    let mut resp = builder.send()?;

    match resp.status().is_success() {
        true => Ok(resp.json::<Task>()?),
        _ => Err(io_other_err(resp.text()?)),
    }
}

// Updates the specified task.
pub fn update(client: &Client, tasklist_id: &str, mut v: Task) -> Result<Task> {
    if v.id.is_none() {
        return Err(io_invalid_input_err("id can not be none"));
    }

    let url = format!(
        "{base_url}/lists/{tasklist_id}/tasks/{task_id}",
        base_url = BASE_URL,
        tasklist_id = tasklist_id,
        task_id = v.id.clone().unwrap().as_str()
    );

    v.updated = None;

    let mut resp = client
        .put(url.as_str())
        .body(serde_json::to_vec(&v)?)
        .send()?;

    match resp.status().is_success() {
        true => Ok(resp.json::<Task>()?),
        _ => Err(io_other_err(resp.text()?)),
    }
}

// Deletes the specified task from the task list.
pub fn delete(client: &Client, tasklist_id: &str, task_id: &str) -> Result<()> {
    let url = format!(
        "{base_url}/lists/{tasklist_id}/tasks/{task_id}",
        base_url = BASE_URL,
        tasklist_id = tasklist_id,
        task_id = task_id,
    );

    let mut resp = client.delete(url.as_str()).send()?;

    match resp.status().is_success() {
        true => Ok(()),
        _ => Err(io_other_err(resp.text()?)),
    }
}

// Clears all completed tasks from the specified task list.
// The affected tasks will be marked as 'hidden' and no longer be returned by default when retrieving all tasks for a task list.
pub fn clear(client: &Client, tasklist_id: &str) -> Result<()> {
    let url = format!(
        "{base_url}/lists/{tasklist_id}/clear",
        base_url = BASE_URL,
        tasklist_id = tasklist_id,
    );

    let mut resp = client.post(url.as_str()).send()?;
    match resp.status().is_success() {
        true => Ok(()),
        _ => Err(io_other_err(resp.text()?)),
    }
}

// Moves the specified task to another position in the task list.
// This can include putting it as a child task under a new parent and/or move it to a different position among its sibling tasks.
pub fn move_task(
    client: &Client,
    tasklist_id: &str,
    task_id: &str,
    opts: InsertOptions,
) -> Result<Task> {
    let url = format!(
        "{base_url}/lists/{tasklist_id}/tasks/{task_id}/move",
        base_url = BASE_URL,
        tasklist_id = tasklist_id,
        task_id = task_id
    );

    let mut resp = client.post(url.as_str()).query(&opts).send()?;

    match resp.status().is_success() {
        true => Ok(resp.json::<Task>()?),
        _ => Err(io_other_err(resp.text()?)),
    }
}

// Updates the specified task. This method supports patch semantics.
pub fn patch(client: &Client, tasklist_id: &str, task_id: &str, v: Task) -> Result<Task> {
    let url = format!(
        "{base_url}/lists/{tasklist_id}/tasks/{task_id}",
        base_url = BASE_URL,
        tasklist_id = tasklist_id,
        task_id = task_id
    );

    let mut resp = client
        .patch(url.as_str())
        .body(serde_json::to_vec(&v)?)
        .send()?;

    match resp.status().is_success() {
        true => Ok(resp.json::<Task>()?),
        _ => Err(io_other_err(resp.text()?)),
    }
}
