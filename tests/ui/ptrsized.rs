use monostate::MustBe;

fn main() {
    let _ = MustBe!(1usize);
    let _ = MustBe!(1isize);
}
