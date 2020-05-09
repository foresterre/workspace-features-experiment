use crate_one::a_callable_fn;

fn main() {
    a_callable_fn();

    if cfg!(feature = "feat_one") {
        println!("[crate: root] (feature = feat_one) ENABLED");
    } else {
        println!("[crate: root] (feature = feat_one) DISABLED");
    }
}
