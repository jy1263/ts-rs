#![allow(dead_code)]
use serde::Serialize;
use ts_rs::TS;

#[derive(TS, Serialize)]
struct Bar {
    field: i32,
}

#[derive(TS, Serialize)]
struct Foo {
    bar: Bar,
}

#[derive(TS, Serialize)]
enum SimpleEnum {
    A(String),
    B(i32),
    C,
    D(String, i32),
    E(Foo),
    F { a: i32, b: String },
}

#[test]
fn test_stateful_enum() {
    assert_eq!(Foo::decl(), r#"interface Foo { bar: Bar, }"#);
    assert_eq!(
        SimpleEnum::decl(),
        r#"type SimpleEnum = { A: string } | { B: number } | "C" | { D: [string, number] } | { E: Foo } | { F: { a: number, b: string, } };"#
    );
}
