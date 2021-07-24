use style_generator_macro::css;

#[test]
fn it_cleans_and_validate_class_names() {
    assert_eq!(
        css!("rounded  md:slashed-zero  stacked-fractions space-y-60  "),
        "rounded md:slashed-zero stacked-fractions space-y-60"
    );

    assert_eq!(css!("rounded    "), "rounded");

    assert_eq!(css!("rounded   rounded "), "rounded");

    assert_eq!(css!("p-2  m-4 "), "p-2 m-4");

    assert_eq!(css!("  sr-only"), "sr-only");

    assert_eq!(
        css!(" rounded p-2     hover:-translate-x-0.5   "),
        "rounded p-2 hover:-translate-x-0.5"
    );
}
