use reqwest::Response;

mod errors;
mod http;
mod tasklists;
mod tasks;

use errors::{Result, TasksError::ResponseError};
use http::{AuthMiddleware, HttpClient};

pub use tasklists::{
    ListOptions as TasklistsOptions, {Tasklist, Tasklists},
};

pub use tasks::{
    InsertOptions as TaskInsertOptions, ListOptions as TaskOptions,
    {Task, TaskLink, TaskStatus, Tasks},
};

const BASE_URL: &str = "https://www.googleapis.com/tasks/v1";

/// Service is an abstraction over google tasks.
pub struct Service {
    http_client: HttpClient,
}

impl Service {
    /// Creates a new service with the given token provider.
    pub fn with_auth<P>(token_provider: P) -> Result<Self>
    where
        P: http::TokenProvider,
    {
        let http_client = AuthMiddleware::new(token_provider).init_http_client()?;

        Ok(Service { http_client })
    }

    /// Creates a new service with the given access token.
    pub fn with_token(access_token: &str) -> Result<Self> {
        let access_token = access_token.to_owned();
        Self::with_auth(move || Ok(access_token.clone()))
    }

    /// Creates a new service with the given access token.
    #[deprecated(since = "0.5.0", note = "Please use `Service::with_token` instead")]
    pub fn new(access_token: &str) -> Result<Self> {
        Self::with_token(access_token)
    }

    /// Returns all the authenticated user's task lists.
    pub async fn list_tasklists(&self, opt: Option<TasklistsOptions>) -> Result<Tasklists> {
        tasklists::list(&self.http_client, opt).await
    }

    /// Returns the authenticated user's specified task list.
    pub async fn get_tasklist(&self, id: &str) -> Result<Tasklist> {
        tasklists::get(&self.http_client, id).await
    }

    /// Creates a new task list and adds it to the authenticated user's task lists.
    pub async fn insert_tasklist(&self, v: tasklists::Tasklist) -> Result<Tasklist> {
        tasklists::insert(&self.http_client, v).await
    }

    /// Updates the authenticated user's specified task list.
    pub async fn update_tasklist(&self, v: Tasklist) -> Result<Tasklist> {
        tasklists::update(&self.http_client, v).await
    }

    /// Deletes the authenticated user's specified task list.
    pub async fn delete_tasklist(&self, id: &str) -> Result<()> {
        tasklists::delete(&self.http_client, id).await
    }

    /// Updates the authenticated user's specified task list. This method supports patch semantics.
    pub async fn patch_tasklist(&self, tasklist_id: &str, v: Tasklist) -> Result<Tasklist> {
        tasklists::patch(&self.http_client, tasklist_id, v).await
    }

    /// Returns all tasks in the specified task list.
    pub async fn list_tasks(
        &self,
        tasklist_id: &str,
        opt: Option<TaskOptions>,
        etag: Option<String>,
    ) -> Result<Option<Tasks>> {
        tasks::list(&self.http_client, tasklist_id, opt, etag).await
    }

    /// Returns the specified task.
    pub async fn get_task(
        &self,
        tasklist_id: &str,
        task_id: &str,
        etag: Option<String>,
    ) -> Result<Option<Task>> {
        tasks::get(&self.http_client, tasklist_id, task_id, etag).await
    }

    /// Creates a new task on the specified task list.
    pub async fn insert_task(
        &self,
        tasklist_id: &str,
        v: Task,
        opts: Option<TaskInsertOptions>,
    ) -> Result<Task> {
        tasks::insert(&self.http_client, tasklist_id, v, opts).await
    }

    /// Updates the specified task.
    pub async fn update_task(&self, tasklist_id: &str, v: Task) -> Result<Task> {
        tasks::update(&self.http_client, tasklist_id, v).await
    }

    /// Deletes the specified task from the task list.
    pub async fn delete_task(&self, tasklist_id: &str, task_id: &str) -> Result<()> {
        tasks::delete(&self.http_client, tasklist_id, task_id).await
    }

    /// Clears all completed tasks from the specified task list.
    /// The affected tasks will be marked as 'hidden' and no longer be returned by default when retrieving all tasks for a task list.
    pub async fn clear_tasks(&self, tasklist_id: &str) -> Result<()> {
        tasks::clear(&self.http_client, tasklist_id).await
    }

    /// Moves the specified task to another position in the task list.
    /// This can include putting it as a child task under a new parent and/or move it to a different position among its sibling tasks.
    pub async fn move_task(
        &self,
        tasklist_id: &str,
        task_id: &str,
        opts: TaskInsertOptions,
    ) -> Result<Task> {
        tasks::move_task(&self.http_client, tasklist_id, task_id, opts).await
    }

    /// Updates the specified task. This method supports patch semantics.
    pub async fn patch_task(&self, tasklist_id: &str, task_id: &str, v: Task) -> Result<Task> {
        tasks::patch(&self.http_client, tasklist_id, task_id, v).await
    }
}

async fn ensure_status_success(resp: Response) -> Result<Response> {
    if !resp.status().is_success() {
        return Err(ResponseError(resp.text().await?));
    }

    Ok(resp)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
