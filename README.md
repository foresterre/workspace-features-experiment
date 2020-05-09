### `cargo +nightly build`

```
[crate: crate-one] (feature = feat_one) DISABLED
[crate: root] (feature = feat_one) DISABLED
```

### ❌ cargo +nightly build --features 'crate-one/feat-one' -Z package-features

```
error: failed to select a version for `crate-one`.
    ... required by package `workspace-features-experiment v0.1.0 (D:\Marwen\userdata\Wolk\LocalStack-Marwen\moi\ws\code\current\workspace-features-experiment)`
versions that meet the requirements `= 0.1.0` are: 0.1.0

the package `workspace-features-experiment` depends on `crate-one`, with features: `feat-one` but `crate-one` does not have these features.


failed to select a version for `crate-one` which could resolve this conflict
```


branch: [underscores](https://github.com/foresterre/workspace-features-experiment/tree/underscores)
### ✔️ cargo +nightly run --features 'crate_one/feat_one' -Z package-features

```
Finished dev [unoptimized + debuginfo] target(s) in 0.43s
Running `target\debug\workspace-features-experiment.exe`
[crate: crate_one] (feature = feat_one) ENABLED
[crate: root] (feature = feat_one) DISABLED
```