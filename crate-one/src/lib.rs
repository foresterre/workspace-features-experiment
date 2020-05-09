pub fn a_callable_fn() {
    if cfg!(feature = "feat-one") {
        println!("[crate: crate-one] (feature = feat-one) ENABLED");
    } else {
        println!("[crate: crate-one] (feature = feat-one) DISABLED");
    }
}
