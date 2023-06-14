use aurorac_macros::generate_params;

struct X;

impl X {
    pub fn into_int_value(&self) {}
}

fn get_nth_param(x: i32) -> X {
    X
}

fn main() {
    let test = "gamma";
    let x = 0;
    let t = "into_int_value";

    generate_params!(test x t);
}
