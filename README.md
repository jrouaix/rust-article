# rust-article
Some rust examples for an article


```bash
cargo run --example helloworld
cargo run --example hello_closure
cargo run --example borrowchecker

cargo test --example typesystem_enums_operators

rustdoc --test ./examples/doctests.rs

docker run -it --rm redis
cargo run --example tide_api
cd loadtest && cargo run -- --host http://127.0.0.1:8080 -u500 -r20

cargo run --example optimus_prime1

cargo test --example optimus_prime1 -- --nocapture
cargo test --example optimus_prime2 -- --nocapture

cargo expand --example macros
```
