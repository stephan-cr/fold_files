#![warn(rust_2018_idioms)]

use std::fs::{metadata, read_dir, DirEntry};
use std::os::linux::fs::MetadataExt;
use std::path::Path;

fn transform_filename(filename: &str) -> String {
    let mut transformed = filename.replace("/", ";");
    if transformed.starts_with(';') {
        transformed.remove(0);
    }

    transformed
}

fn visit(path: &Path, cb: &dyn Fn(&DirEntry), dev: Option<u64>) {
    let dev = if dev == None {
        Some(metadata(&path).unwrap().st_dev())
    } else {
        dev
    };
    for entry in read_dir(&path).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() {
            let meta = metadata(&path).unwrap();
            let size = meta.len();
            let current_dev_id = meta.st_dev();
            if let Some(dev_id) = dev {
                if dev_id != current_dev_id {
                    return;
                }
            }
            if let Some(ref filename_str) = path.to_str() {
                let transformed = transform_filename(filename_str);
                println!("{} {}", transformed, size);
            }
        } else if path.is_dir() {
            visit(&path, &cb, dev);
        }
    }
}

fn main() {
    visit(&Path::new("."), &|_x| {}, None);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_transform_filename() {
        assert_eq!(super::transform_filename("a/b"), "a;b");
        assert_eq!(super::transform_filename("/a/b"), "a;b");
    }
}
