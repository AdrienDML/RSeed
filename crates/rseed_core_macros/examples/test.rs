use rseed_core_macros::set_get;

fn main() {

}

struct Test {
    test : String,
}

impl Test {
    set_get!{test : String}
}