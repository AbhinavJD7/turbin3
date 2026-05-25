# AMM (Assignment 1)

Repository containing an Automated Market Maker (AMM) implementation for the Assignment 1.

Summary
- Implemented AMM program (Anchor/Rust) with instructions: `init`, `deposit`, `withdraw`, `swap`, `collect_fees`, and admin setters.
- Unit tests for the on-chain logic in `programs/amm` passed locally.

Commit
- Commit checked: `fd0d8fb` (short hash)

Build & Test (local)
1. Install Rust toolchain matching the project (rust-toolchain.toml is present).
2. From repository root run:

```bash
cd /Users/abhinavrai/Dev/turbin3/learn3/amm
cargo build
cargo test
```

Expected test output (unit tests):

```
running 12 tests
...
test result: ok. 12 passed; 0 failed
```

Submit
- Include this repository and a screenshot showing the passing tests.
- Place the screenshot as `tests/screenshot.png` in the repo before zipping, or attach it separately.

Notes
- If you want, I can run the TypeScript/JS integration tests next (`npm test`) and add the passing screenshot file to the repo.
