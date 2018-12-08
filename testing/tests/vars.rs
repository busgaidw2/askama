use askama::Template;

#[derive(Template)]
#[template(source = "{% let v = s %}{{ v }}", ext = "txt")]
struct LetTemplate<'a> {
    s: &'a str,
}

#[test]
fn test_let() {
    let t = LetTemplate { s: "foo" };
    assert_eq!(t.render().unwrap(), "foo");
}

#[derive(Template)]
#[template(path = "let-decl.html")]
struct LetDeclTemplate<'a> {
    cond: bool,
    s: &'a str,
}

#[test]
fn test_let_decl() {
    let t = LetDeclTemplate {
        cond: false,
        s: "bar",
    };
    assert_eq!(t.render().unwrap(), "bar");
}
