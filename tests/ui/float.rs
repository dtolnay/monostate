use monostate::MustBe;

fn main() {
    let _ = MustBe!(1.0);
    let _ = MustBe!(b"...");
    let _ = MustBe!(c"...");
}
