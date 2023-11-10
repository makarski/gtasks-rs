gtasks
======

[![CodeScene Code Health](https://codescene.io/projects/45887/status-badges/code-health)](https://codescene.io/projects/45887)

Rust Client for [Google Tasks API v1](https://developers.google.com/tasks/v1/reference)

## Example

```toml
[dependencies]
gtasks = "0.5"
```
Read tasks

```rust,no_run
use gtasks::Service;

async fn main() {
    // Option 1: use static token
    let task_srvc = Service::with_token("access_token").unwrap();
    read_tasks(&task_srvc).await;

    // Option 2: use closure to obtain auth token
    let token_provider = || {
        Ok("access_token".to_owned())
    };

    let task_srvc = Service::with_auth(token_provider).unwrap();
    read_tasks(&task_srvc).await;
}

async fn read_tasks(task_srvc: &Service) {
    let tasklists = task_srvc.list_tasklists(None).await.unwrap();
    for tasklist in tasklists.items.iter() {
        println!("tasklist: {}", tasklist.title.as_ref().unwrap());
    }

    let list_id = tasklists.items[0].id.as_ref().unwrap();

    // print tasks from the list
    let opts = gtasks::TaskOptions {
        max_results: Some(5),
        show_completed: Some(true),
        show_hidden: Some(true),
        ..Default::default()
    };

    let tasks = task_srvc
        .list_tasks(list_id, Some(opts), None)
        .await
        .unwrap();

    if let Some(tasks) = tasks {
        let items = tasks.items.unwrap();

        for item in items {
            println!("{:?}", item.title);
        }
    }
}
```

## License

License under either or:

* [MIT](LICENSE-MIT)
* [Apache License, Version 2.0](LICENSE-APACHE)
