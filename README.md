### `cargo +nightly run`

```
[crate: crate-one] (feature = feat_one) DISABLED
[crate: root] (feature = feat_one) DISABLED
```

### ✔️ cargo +nightly run --features 'crate-one/feat-one' -Z package-features

```
 cargo +nightly run --features 'crate-one/feat-one' -Z package-features
   Compiling crate-one v0.1.0 (\ws\workspace-features-experiment\crate-one)
   Compiling workspace-features-experiment v0.1.0 (\ws\workspace-features-experiment)
    Finished dev [unoptimized + debuginfo] target(s) in 1.10s
     Running `target\debug\workspace-features-experiment.exe`
[crate: crate-one] (feature = feat-one) ENABLED
[crate: root] (feature = feat-one) DISABLED

```

### cargo run --features 'crate-one/feat-one'

```
Compiling crate-one v0.1.0 (\ws\workspace-features-experiment\crate-one)
   Compiling workspace-features-experiment v0.1.0 (\ws\workspace-features-experiment)
    Finished dev [unoptimized + debuginfo] target(s) in 1.24s
     Running `target\debug\workspace-features-experiment.exe`
[crate: crate-one] (feature = feat-one) ENABLED
[crate: root] (feature = feat-one) DISABLED

```