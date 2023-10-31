gtasks
======

[![CodeScene Code Health](https://codescene.io/projects/45887/status-badges/code-health)](https://codescene.io/projects/45887)

Rust Client for [Google Tasks API v1](https://developers.google.com/tasks/v1/reference)

## Example

```toml
[dependencies]
gtasks = "0.4"
```
Read tasks

```rust,no_run
// create a service
let service = gtasks::Service::new("google_token").unwrap();

// obtain tasklist id
let tasklists = service.list_tasklists(None).unwrap();
let list_id = tasklists.items[0].id.unwrap();

// print tasks from the list
let opts = gtasks::TaskOptions{
    max_results: Some(5),
    show_completed: Some(true),
    show_hidden: Some(true),
};

let tasks = service.list_tasks(list_id, opts, None).unwrap();

if let Some(tasks) = tasks {
    let items = tasks.items.unwrap();

    for item in items {
        println!("{:?}", item.title);
    }
}
```

## License

License under either or:

* [MIT](LICENSE-MIT)
* [Apache License, Version 2.0](LICENSE-APACHE)
