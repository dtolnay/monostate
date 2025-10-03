use monostate::MustBe;

fn main() {
    let MustBe!("string") = MustBe!("string");
}
