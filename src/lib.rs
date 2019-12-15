extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;

mod tasklists;
mod tasks;

use std::{
    error::Error,
    io::{Error as io_err, ErrorKind as io_err_kind},
    result,
};

use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client,
};

pub use tasklists::{
    ListOptions as TasklistsOptions, {Tasklist, Tasklists},
};

pub use tasks::{
    InsertOptions as TaskInsertOptions, ListOptions as TaskOptions, {Task, TaskStatus, Tasks},
};

const BASE_URL: &'static str = "https://www.googleapis.com/tasks/v1";

pub type Result<T> = result::Result<T, Box<dyn Error>>;

fn io_other_err(msg: String) -> Box<dyn Error> {
    Box::new(io_err::new(io_err_kind::Other, msg))
}

fn io_invalid_input_err(msg: &str) -> Box<dyn Error> {
    Box::new(io_err::new(io_err_kind::InvalidInput, msg))
}

pub struct Service {
    http_client: Client,
}

impl Service {
    pub fn new(access_token: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(access_token)?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        Ok(Service {
            http_client: Client::builder().default_headers(headers).build()?,
        })
    }

    /// Returns all the authenticated user's task lists.
    pub fn list_tasklists(&self, opt: Option<TasklistsOptions>) -> Result<Tasklists> {
        tasklists::list(&self.http_client, opt)
    }

    /// Returns the authenticated user's specified task list.
    pub fn get_tasklist(&self, id: &str) -> Result<Tasklist> {
        tasklists::get(&self.http_client, id)
    }

    /// Creates a new task list and adds it to the authenticated user's task lists.
    pub fn insert_tasklist(&self, v: tasklists::Tasklist) -> Result<Tasklist> {
        tasklists::insert(&self.http_client, v)
    }

    /// Updates the authenticated user's specified task list.
    pub fn update_tasklist(&self, v: Tasklist) -> Result<Tasklist> {
        tasklists::update(&self.http_client, v)
    }

    /// Deletes the authenticated user's specified task list.
    pub fn delete_tasklist(&self, id: &str) -> Result<()> {
        tasklists::delete(&self.http_client, id)
    }

    /// Updates the authenticated user's specified task list. This method supports patch semantics.
    pub fn patch_tasklist(&self, tasklist_id: &str, v: Tasklist) -> Result<Tasklist> {
        tasklists::patch(&self.http_client, tasklist_id, v)
    }

    /// Returns all tasks in the specified task list.
    pub fn list_tasks(&self, tasklist_id: &str, opt: Option<TaskOptions>) -> Result<Tasks> {
        tasks::list(&self.http_client, tasklist_id, opt)
    }

    /// Returns the specified task.
    pub fn get_task(&self, tasklist_id: &str, task_id: &str) -> Result<Task> {
        tasks::get(&self.http_client, tasklist_id, task_id)
    }

    /// Creates a new task on the specified task list.
    pub fn insert_task(
        &self,
        tasklist_id: &str,
        v: Task,
        opts: Option<TaskInsertOptions>,
    ) -> Result<Task> {
        tasks::insert(&self.http_client, tasklist_id, v, opts)
    }

    /// Updates the specified task.
    pub fn update_task(&self, tasklist_id: &str, v: Task) -> Result<Task> {
        tasks::update(&self.http_client, tasklist_id, v)
    }

    /// Deletes the specified task from the task list.
    pub fn delete_task(&self, tasklist_id: &str, task_id: &str) -> Result<()> {
        tasks::delete(&self.http_client, tasklist_id, task_id)
    }

    /// Clears all completed tasks from the specified task list.
    /// The affected tasks will be marked as 'hidden' and no longer be returned by default when retrieving all tasks for a task list.
    pub fn clear_tasks(&self, tasklist_id: &str) -> Result<()> {
        tasks::clear(&self.http_client, tasklist_id)
    }

    /// Moves the specified task to another position in the task list.
    /// This can include putting it as a child task under a new parent and/or move it to a different position among its sibling tasks.
    pub fn move_task(
        &self,
        tasklist_id: &str,
        task_id: &str,
        opts: TaskInsertOptions,
    ) -> Result<Task> {
        tasks::move_task(&self.http_client, tasklist_id, task_id, opts)
    }

    /// Updates the specified task. This method supports patch semantics.
    pub fn patch_task(&self, tasklist_id: &str, task_id: &str, v: Task) -> Result<Task> {
        tasks::patch(&self.http_client, tasklist_id, task_id, v)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
