```
forge install
forge test --match-test test_shouldSign -vv
cargo build
cargo run
forge test --match-test test_shouldVerifyZKP -vv
```
