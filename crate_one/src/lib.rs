pub fn a_callable_fn() {
    if cfg!(feature = "feat_one") {
        println!("[crate: crate_one] (feature = feat_one) ENABLED");
    } else {
        println!("[crate: crate_one] (feature = feat_one) DISABLED");
    }
}
