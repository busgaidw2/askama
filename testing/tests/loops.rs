#[macro_use]
extern crate askama;

use askama::Template;

#[derive(Template)]
#[template(path = "for.html")]
struct ForTemplate<'a> {
    strings: Vec<&'a str>,
}

#[test]
fn test_for() {
    let s = ForTemplate {
        strings: vec!["A", "alfa", "1"],
    };
    assert_eq!(s.render().unwrap(), "0. A (first)\n1. alfa\n2. 1\n");
}

#[derive(Template)]
#[template(path = "nested-for.html")]
struct NestedForTemplate<'a> {
    seqs: Vec<&'a [&'a str]>,
}

#[test]
fn test_nested_for() {
    let alpha = vec!["a", "b", "c"];
    let numbers = vec!["one", "two"];
    let s = NestedForTemplate {
        seqs: vec![&alpha, &numbers],
    };
    assert_eq!(s.render().unwrap(), "1\n  0a1b2c2\n  0one1two");
}

#[derive(Template)]
#[template(path = "precedence-for.html")]
struct PrecedenceTemplate<'a> {
    strings: Vec<&'a str>,
}

#[test]
fn test_precedence_for() {
    let s = PrecedenceTemplate {
        strings: vec!["A", "alfa", "1"],
    };
    assert_eq!(s.render().unwrap(), "0. A (first)\n1. alfa\n2. 1\n");
}
