---
description: Check clippy and fmt after changes
---

Run cargo formatting and clippy checks to ensure code quality after making changes.

// turbo-all
1. Run cargo fmt to format the code
```bash
cargo fmt
```

2. Run cargo clippy to catch common mistakes and improve Rust code
```bash
cargo clippy --workspace --all-targets --all-features
```
