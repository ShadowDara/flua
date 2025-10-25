## Testing

cargo
```sh
cargo test
cargo nextest run --jobs=1 --no-fail-fast
```

Testing commands with Shell
```sh
chmod +x ./test/command_tester.sh
./test/command_tester.sh
```

Testing Lua
```sh
cargo run test/main.lua
cargo run test/data.lua
cargo run test/http_async.lua
cargo run test/api_server.lua
```

Testing Commands and Other Commands
```sh
cargo build && test/command_tester.sh
```
