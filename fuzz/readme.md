# Fuzzing


Install [cargo fuzz](https://github.com/rust-fuzz/cargo-fuzz).

Compile:
```bash
cargo build
```

Run fuzzer:
```bash
cargo +nightly fuzz run csv_fuzz
```

### I found a bug, whats next?
Copy the tracelog from your terminal(output of the fuzzer) and the artifact to a github issue.   

