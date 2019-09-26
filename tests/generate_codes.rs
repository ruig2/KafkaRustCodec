use codegen::Scope;

#[test]
fn generate_code_with_codegen() {
    let mut scope = Scope::new();

    scope
        .new_struct("Foo")
        .derive("Debug")
        .field("one", "usize")
        .field("two", "String");

    println!("{}", scope.to_string());
}
