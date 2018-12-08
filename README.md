# Askama

[![Documentation](https://docs.rs/askama/badge.svg)](https://docs.rs/askama/)
[![Latest version](https://img.shields.io/crates/v/askama.svg)](https://crates.io/crates/askama)
[![Build status](https://api.travis-ci.org/djc/askama.svg?branch=master)](https://travis-ci.org/djc/askama)
[![Windows build](https://ci.appveyor.com/api/projects/status/github/djc/askama?svg=true)](https://ci.appveyor.com/project/djc/askama)
[![Chat](https://badges.gitter.im/gitterHQ/gitter.svg)](https://gitter.im/djc/askama)

Askama implements a template rendering engine based on Jinja.
It generates Rust code from your templates at compile time
based on a user-defined `struct` to hold the template's context.
See below for an example, or read [the documentation][docs].

**"I use Askama for actix's TechEmpower benchmarks."** --
[Nikolay Kim][fafhrd91], creator of actix-web

**"Pretty exciting. I would love to use this already."** --
[Armin Ronacher][mitsuhiko], creator of Jinja

All feedback welcome. Feel free to file bugs, requests for documentation and
any other feedback to the [issue tracker][issues] or [tweet me][twitter].
Many thanks to [David Tolnay][dtolnay] for his support in improving Askama.

Askama was created by and is maintained by Dirkjan Ochtman. If you are in a
position to support ongoing maintenance and further development or use it
in a for-profit context, please consider supporting my open source work on
[Patreon][patreon].

### Feature highlights

* Construct templates using a familiar, easy-to-use syntax
* Template code is compiled into your crate for [optimal performance][benchmarks]
* Benefit from the safety provided by Rust's type system
* Optional built-in support for Actix, Iron and Rocket web frameworks
* Debugging features to assist you in template development
* Templates must be valid UTF-8 and produce UTF-8 when rendered
* Works on stable Rust

### Supported in templates

* Template inheritance
* Loops, if/else statements and include support
* Macro support
* Variables (no mutability allowed)
* Some built-in filters, and the ability to use your own
* Whitespace suppressing with '-' markers
* Opt-out HTML escaping
* Syntax customization

### Limitations

* A limited number of built-in filters have been implemented

[docs]: https://docs.rs/askama
[fafhrd91]: https://github.com/fafhrd91
[mitsuhiko]: http://lucumr.pocoo.org/
[issues]: https://github.com/djc/askama/issues
[twitter]: https://twitter.com/djco/
[dtolnay]: https://github.com/dtolnay
[patreon]: https://www.patreon.com/dochtman
[benchmarks]: https://github.com/djc/template-benchmarks-rs


How to get started
------------------

First, add the following to your crate's `Cargo.toml`:

```toml
# in section [dependencies]
askama = "0.7"

# in section [build-dependencies]
askama = "0.7"
```

Because Askama will generate Rust code from your template files,
the crate will need to be recompiled when your templates change.
This is supported by adding a build script, `build.rs`, to your crate.
It needs askama as a build dependency:

```rust
extern crate askama;

fn main() {
    askama::rerun_if_templates_changed();
}
```

Now create a directory called `templates` in your crate root.
In it, create a file called `hello.html`, containing the following:

```
Hello, {{ name }}!
```

In any Rust file inside your crate, add the following:

```rust
extern crate askama; // for the Template trait and custom derive macro

use askama::Template; // bring trait in scope

#[derive(Template)] // this will generate the code...
#[template(path = "hello.html")] // using the template in this path, relative
                                 // to the templates dir in the crate root
struct HelloTemplate<'a> { // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
}
   
fn main() {
    let hello = HelloTemplate { name: "world" }; // instantiate your struct
    println!("{}", hello.render().unwrap()); // then render it.
}
```

You should now be able to compile and run this code.

Review the [test cases] for more examples.

[test cases]: https://github.com/djc/askama/tree/master/testing


Debugging and troubleshooting
-----------------------------

You can view the parse tree for a template as well as the generated code by
changing the `template` attribute item list for the template struct:

```rust
#[derive(Template)]
#[template(path = "hello.html", print = "all")]
struct HelloTemplate<'a> { ... }
```

The `print` key can take one of four values:

* `none` (the default value)
* `ast` (print the parse tree)
* `code` (print the generated code)
* `all` (print both parse tree and code)

The resulting output will be printed to `stderr` during the compilation process.

The parse tree looks like this for the example template:

```
[Lit("", "Hello,", " "), Expr(WS(false, false), Var("name")),
Lit("", "!", "\n")]
```

The generated code looks like this:

```rust
impl < 'a > ::askama::Template for HelloTemplate< 'a > {
    fn render_into(&self, writer: &mut ::std::fmt::Write) -> ::askama::Result<()> {
        write!(
            writer,
            "Hello, {expr0}!",
            expr0 = &::askama::MarkupDisplay::from(&self.name),
        )?;
        Ok(())
    }
    fn extension() -> Option<&'static str> {
        Some("html")
    }
}
impl < 'a > ::std::fmt::Display for HelloTemplate< 'a > {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::askama::Template::render_into(self, f).map_err(|_| ::std::fmt::Error {})
    }
}
```
