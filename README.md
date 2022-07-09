# Time Tracker Base (v3)

A fullstack time tracker app inspired by toggl/clockify.

## The Stack

### Backend

https://github.com/teooliver/ttb-server

- Rust
- Warp
- Mongodb
- Docker

> I started the previous version of this app using Express/Mongoose (Typescript) in the backend. You can check it here:
> https://github.com/teooliver/time-tracker-base

### FrontEnd

https://github.com/teooliver/ttb-client

- Typescript
- React

> This project uses MongoDB and provides a simple docker-compose.yml file to set it up in your system.

### Next Steps:

- Refactor the code using the `rust-analyzer` style and contributions guide:
  https://github.com/rust-lang/rust-analyzer/blob/master/docs/dev/style.md
- Remove dependency on custom `Result` type and use the `std Result` instead
- Update `clap` lib impls (using deprecated methods at the moment)
- Remove thiserror dependency

### Notes and Todos:

[Notes](./notes.md)
