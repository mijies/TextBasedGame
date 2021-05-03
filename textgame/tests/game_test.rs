use textgame_macro::{EnumVariantCount, decorate};

#[derive(EnumVariantCount)]
enum TestEnum {
    One,
    Two,
}

#[decorate(wrapper)]
fn wrapped(b: bool) -> bool {
    b
}

fn wrapper<F>(f: F, b:bool) -> bool
    where F: Fn(bool) -> bool
{
    !f(b)
}

#[test]
fn derive_enum_variant_count_test() {
    assert_eq!(TestEnum::variant_count(), 2);
}

#[test]
fn decorate_test() {
    assert!(!wrapped(true));
}