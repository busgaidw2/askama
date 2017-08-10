#[macro_use]
extern crate askama;

use askama::Template;

#[derive(Template)] // this will generate the code...
#[template(path = "hello.html", print = "all")] // using the template in this path, relative
                                 // to the templates dir in the crate root
struct HelloTemplate<'a> { // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
}

#[test]
fn main() {
    let hello = HelloTemplate { name: "world" }; // instantiate your struct
    assert_eq!("Hello, world!", hello.render().unwrap()); // then render it.
}
