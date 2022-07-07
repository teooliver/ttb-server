# Tips:

### Hot Reload:

Run cargo watch in terminal:

```rs
cargo watch -x run
```

### Calculate Duration between Datetimes

```rs
let duration = end_time.to_chrono() - initial_time.to_chrono();
println!("DURATION {:?}", duration);
```

### Convert UTC String to chrono::DateTime<Utc> to bson::DateTime

```rs
 let chrono_dt: chrono::DateTime<Utc> = "2021-10-19T20:25:17.734Z".parse().unwrap();
    let initial_time: bson::DateTime = chrono_dt.into();
    println!("{:?}", initial_time.to_string());
```

### Implement default traits for structs when needed:

> https://doc.rust-lang.org/std/default/trait.Default.html

# TODO:

#### Backend

- Remove dependency on custom `Result` type and use the `std Result` instead
- Add user's id field to each task. (So we can use authorization to allow deletion and creation etc)
- Update `clap` lib impls (using deprecated methods at the moment)
- Remove thiserror dependency
- Add proper error handling for DB interactions, no more `?` or unsafe `unwraps` all around.
- Refactor all db object types
- Refactor project into a library and a binary so its easier to run non blackbox tests.
- Move routes in main to their own files
- Create better types for json errors.
- Better return messages to routes success (instead of just 200);
- Extract Routes to its own files
- Add Archive option for Clients and Projects
- Add Delete option for Clients and Projects
- Dockerize frontend and backend.
- Better Error handling in general:
- - BodyDeserializeError
- - CorsForbidden
- - UNPROCESSABLE_ENTITY (invalid ID for example)
- Add filter requests based on headers (for example team names) so we can log and understand from where and how the requests are being done.
- Add health_check route
- Add tracing to all routes
- Limit fake events data to maximum of `8 hours a day`. (Good challenge)
- Make use of Default Trait. I.e: impl Default for PaginationQuery/Pagination
- ~~ Rename the `models` folder to `types`~~
- ~~Add authentication and authorization~~
- ~~Pagination to `Get All Tasks Grouped By Date` route, one page per week. `api/tasks?page=1&size=2`~~
- ~~Add Users~~
- ~~Extract Error handling functions to its own `lib`~~

# IMPORTANT INTEGRATION TESTS

- Add Task
- Edit Task
- Add Project
- Delete Project
- Add Client
- Delete Client

- Whole user journey from seeding tasks, create tasks, projects and clients, check Charts and so on.

# KNOWN BUGS:

---

### Script

- Add script to the frontend and backend repos that creates a docker-compose/Dockerfile configuration in the parent folder so to start the project with just one command.

### Run Project CLI example:

- cargo run -- --db-host localhost --log-level warn --db-name rust-time-tracker-base --db-port 5000 --db-password 12345

### Study

- Better understading of how `tracing` and `tracing-subscriber` works;
- How would I implement Datadog here?

### Questions:

In the filters chaing, when expecting a `query` from the url, is it better to just use `warp::query()` and later build a struct or whatever from it, or use it as a generic recieving the apropriate type from the beginning, example: warp::query::<tasks::PaginationQuery>()
