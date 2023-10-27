# chatgpt-rusttls-client-rs

This is a brief exploration into using ChatGPT to churn out trivial boilerplate code, and what's interesting is that we can progressively add more constraints to the request to obtain a more concrete product fit for our use.

The output initially provided compiles although needs some cleaning up; and I've noticed a small error too.

"Create a Rust crate (library) that fetches data from an external service using Tokio and is async using reqwest and rustls. Errors should be boxed in errors.rs

Confiig data should be stored in a record struct that is a Mutex Arc and initialised in Main. This should be static"