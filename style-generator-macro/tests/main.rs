use style_generator_macro::css;

#[test]
fn it_works() {
    let c = css!("rounded    ");

    assert_eq!(c, "rounded");

    let c = css!("p-2  m-4 ");

    assert_eq!(c, "p-2 m-4");

    let c = css!("  sr-only");

    assert_eq!(c, "sr-only");

    let c = css!(" rounded p-2     hover:-translate-x-0.5   ");

    assert_eq!(c, "rounded p-2 hover:-translate-x-0.5");
}
