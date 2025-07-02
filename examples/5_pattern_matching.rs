enum Test {
    A,
    B(u32),
    C(f32),
    D(),
}
fn main() {
    let a = Test::D();

    let b = match a {
        Test::A => 1,
        Test::B(_) => 2,
        Test::C(_) => 3,
        Test::D() => 4,
    };
    assert_eq!(b, 4);

    let c = Some(1);
    if let Some(num) = c {
        assert_eq!(num, 1);
    }

    let d = Test::B(8u32);
    let Test::B(e) = d else {
        return;
    };
    assert_eq!(e, 8);
}
