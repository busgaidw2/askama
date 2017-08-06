use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

fn template_dir() -> PathBuf {
    let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    path.push("templates");
    path
}

pub fn find_template_from_path<'a>(path: &str, start_at: Option<&str>) -> PathBuf {
    let root = template_dir();
    match start_at {
        Some(rel) => {
            let mut fs_rel_path = root.clone();
            fs_rel_path.push(rel);
            fs_rel_path = fs_rel_path.with_file_name(path);
            if fs_rel_path.exists() {
                return fs_rel_path.strip_prefix(&root).unwrap().to_owned();
            }
        },
        None => {},
    }

    let mut fs_abs_path = root.clone();
    let path = Path::new(path);
    fs_abs_path.push(Path::new(path));
    if fs_abs_path.exists() {
        path.to_owned()
    } else {
        panic!(format!("template '{:?}' not found", path.to_str()));
    }
}

pub fn get_template_source(tpl_file: &str) -> String {
    let mut path = template_dir();
    path.push(Path::new(tpl_file));
    let mut f = match File::open(&path) {
        Err(_) => {
            let msg = format!("unable to open template file '{}'",
                              &path.to_str().unwrap());
            panic!(msg);
        },
        Ok(f) => f,
    };
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s
}

#[cfg(test)]
mod tests {
    use super::find_template_from_path;
    use super::Path;

    #[test]
    fn find_absolute() {
        let path = find_template_from_path("sub/b.html", Some("a.html"));
        assert_eq!(path, Path::new("sub/b.html"));
    }

    #[test]
    #[should_panic]
    fn find_relative_nonexistent() {
        find_template_from_path("b.html", Some("a.html"));
    }

    #[test]
    fn find_relative() {
        let path = find_template_from_path("c.html", Some("sub/b.html"));
        assert_eq!(path, Path::new("sub/c.html"));
    }

    #[test]
    fn find_relative_sub() {
        let path = find_template_from_path("sub1/d.html", Some("sub/b.html"));
        assert_eq!(path, Path::new("sub/sub1/d.html"));
    }
}
