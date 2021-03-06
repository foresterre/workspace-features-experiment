use crate_one::a_callable_fn;

fn main() {
    a_callable_fn();

    if cfg!(feature = "feat-one") {
        println!("[crate: root] (feature = feat-one) ENABLED");
    } else {
        println!("[crate: root] (feature = feat-one) DISABLED");
    }
}
