#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;
extern crate quote;
extern crate syn;

#[cfg(feature = "serde-json")]
extern crate serde;
#[cfg(feature = "serde-json")]
extern crate serde_json;

pub use escaping::MarkupDisplay;
pub use errors::{Error, Result};
pub mod filters;
pub mod path;

mod escaping;
mod generator;
mod parser;

use std::borrow::Cow;
use std::path::PathBuf;


/// Takes a `syn::DeriveInput` and generates source code for it
///
/// Reads the metadata from the `template()` attribute to get the template
/// metadata, then fetches the source from the filesystem. The source is
/// parsed, and the parse tree is fed to the code generator. Will print
/// the parse tree and/or generated source according to the `print` key's
/// value as passed to the `template()` attribute.
pub fn build_template(ast: &syn::DeriveInput) -> String {
    let data = TemplateInput::new(ast);
    let nodes = parser::parse(data.source.as_ref());
    if data.meta.print == Print::Ast || data.meta.print == Print::All {
        println!("{:?}", nodes);
    }
    let code = generator::generate(data.ast, &data.path, &nodes);
    if data.meta.print == Print::Code || data.meta.print == Print::All {
        println!("{}", code);
    }
    code
}

struct TemplateInput<'a> {
    pub ast: &'a syn::DeriveInput,
    pub meta: TemplateMeta<'a>,
    pub path: PathBuf,
    pub source: Cow<'a, str>,
}

impl<'a> TemplateInput<'a> {
    fn new(ast: &'a syn::DeriveInput) -> TemplateInput<'a> {
        let meta = TemplateMeta::new(ast);
        let (path, source) = match meta.source {
            Source::Source(s) => (PathBuf::new(), Cow::Borrowed(s)),
            Source::Path(s) => {
                let path = path::find_template_from_path(&s, None);
                let src = path::get_template_source(&path);
                (path, Cow::Owned(src))
            },
        };
        TemplateInput { ast, meta, path, source }
    }
}

mod errors {
    error_chain! {
        foreign_links {
            Fmt(::std::fmt::Error);
            Json(::serde_json::Error) #[cfg(feature = "serde-json")];
        }
    }
}

// Holds metadata for the template, based on the `template()` attribute.
struct TemplateMeta<'a> {
    source: Source<'a>,
    print: Print,
}

impl<'a> TemplateMeta<'a> {
    fn new(ast: &'a syn::DeriveInput) -> TemplateMeta<'a> {
        let attr = ast.attrs.iter().find(|a| a.name() == "template");
        if attr.is_none() {
            let msg = format!("'template' attribute not found on struct '{}'",
                              ast.ident.as_ref());
            panic!(msg);
        }

        let attr = attr.unwrap();
        let mut source = None;
        let mut print = Print::None;
        if let syn::MetaItem::List(_, ref inner) = attr.value {
            for nm_item in inner {
                if let syn::NestedMetaItem::MetaItem(ref item) = *nm_item {
                    if let syn::MetaItem::NameValue(ref key, ref val) = *item {
                        match key.as_ref() {
                            "path" => if let syn::Lit::Str(ref s, _) = *val {
                                source = Some(Source::Path(s.as_ref()));
                            } else {
                                panic!("template path must be string literal");
                            },
                            "source" => if let syn::Lit::Str(ref s, _) = *val {
                                source = Some(Source::Source(s.as_ref()));
                            } else {
                                panic!("template source must be string literal");
                            },
                            "print" => if let syn::Lit::Str(ref s, _) = *val {
                                print = s.into();
                            } else {
                                panic!("print value must be string literal");
                            },
                            _ => { panic!("unsupported annotation key found") }
                        }
                    }
                }
            }
        }

        match source {
            Some(s) => TemplateMeta { source: s, print },
            None => panic!("template path or source not found in struct attributes"),
        }
    }
}


enum Source<'a> {
    Path(&'a str),
    Source(&'a str),
}

#[derive(PartialEq)]
enum Print {
    All,
    Ast,
    Code,
    None,
}

impl<'a> From<&'a String> for Print {
    fn from(s: &'a String) -> Print {
        use Print::*;
        match s.as_ref() {
            "all" => All,
            "ast" => Ast,
            "code" => Code,
            "none" => None,
            v => panic!("invalid value for print option: {}", v),
        }
    }
}
