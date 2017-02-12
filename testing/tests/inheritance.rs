extern crate askama;
#[macro_use]
extern crate askama_derive;

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
    assert_eq!(t.render(), "Foo\n\nCopyright 2017\n");
}

#[test]
fn test_simple_extends() {
    let t = ChildTemplate { _parent: BaseTemplate { title: "Bar" } };
    assert_eq!(t.render(), "Bar\nContent goes here\nCopyright 2017\n");
}
