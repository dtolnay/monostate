// This error should be unreachable using the public API of monostate. Testing
// it anyway.
type __Private = (
    monostate::alphabet::len<8>,
    (
        monostate::alphabet::a,
        monostate::alphabet::s,
        monostate::alphabet::d,
        monostate::alphabet::f,
    ),
);

struct N<const N: usize>;

fn main() {
    let _ = N::<{ monostate::MustBeStr::<__Private>::VALUE.len() }>;
}
