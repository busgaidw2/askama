#[macro_use]
extern crate askama;

use askama::Template;

#[derive(Template)]
#[template(path = "base.html")]
struct BaseTemplate<'a> {
    title: &'a str,
}

#[derive(Template)]
#[template(path = "child.html")]
struct ChildTemplate<'a> {
    _parent: BaseTemplate<'a>,
}

#[test]
fn test_use_base_directly() {
    let t = BaseTemplate { title: "Foo" };
    assert_eq!(t.render().unwrap(), "Foo\n\nFoo\nCopyright 2017");
}

#[test]
fn test_simple_extends() {
    let t = ChildTemplate { _parent: BaseTemplate { title: "Bar" } };
    assert_eq!(t.render().unwrap(), "Bar\n(Bar) Content goes here\nFoo\nCopyright 2017");
}
