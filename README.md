# local-ip

Get your local ip address in Rust.

```toml
local-ip = "0.2"
```

then

```rust
extern crate local_ip;

let ip = local_ip::get().unwrap();
println!("local ip address: {:?}", ip.to_string());
```

## Design

`local-ip` should not rely on any syscall or FFI interfaces (there are other crates for this purpose). As a result, installation should be trivial. Instead, it parses the output of `ipconfig` or `ifconfig` depending on the OS.

## License

MIT or Apache-2.0, at your option.
