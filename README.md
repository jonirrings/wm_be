# warehouse management backend
> for cyus requirement
> 
> inspired by torrust (most code was copied from torrust)

# Run
you could run with `auto-reload` by system_fd
```shell
systemfd --no-pid -s http::6000 -- cargo watch -x "run --bin wm"
```
or just run by cargo
```shell
cargo run --bin wm
```