# local-ip

Get your local ip address in Rust.

```toml
local-ip = "0.1"
```

then

```rust
extern crate local_ip;

let ip = local_ip::get().unwrap();
println!("local ip address: {:?}", ip.to_string());
```

**TODO:** Windows support.

## License

MIT or Apache-2.0, at your option.
