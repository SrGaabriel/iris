# iris-rs

A live messaging service built with Rust & Svelte and powered by raw WebSockets.

---

## Technologies

The main technologies implemented in this project are:

* **Websockets:** powered by `axum`
* **Rest API:** powered by `axum`
* **PostgreSQL:** powered by `diesel`
* **Protobuf _(soon to be dropped)_:** powered by `prost` and `protobuf.js`
* **Serialization:** powered by `serde`
* **Authentication:** powered by `jwt`
* **Hashing:** powered by `argon2`
* **Async:** powered by `tokio`
* **Frontend:** powered by `Svelte`

And many more...

---

## Features

- [x] **Real-time messaging:** send and receive messages in real-time
- [x] **Authentication:** secure your account with a password
- [x] **Dark mode:** toggle between light and dark mode
- [x] **Notifications:** receive notifications when you're mentioned
- [x] **Edit & delete messages:** edit and delete your messages
- [x] **Receipt indicators:** see when your messages are read
- [x] **Typing indicators:** see when someone is typing
- [x] **Reactions:** react to messages with emojis