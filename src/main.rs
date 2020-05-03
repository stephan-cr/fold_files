#![warn(rust_2018_idioms)]

use std::env;
use std::fs::{metadata, read_dir, DirEntry};
use std::io::{self, StdoutLock, Write};
use std::os::linux::fs::MetadataExt;
use std::path::Path;

use clap::{App, Arg};

fn transform_filename(filename: &str) -> String {
    let mut transformed = filename.replace("/", ";");
    if transformed.starts_with(';') {
        transformed.remove(0);
    }

    transformed
}

fn visit(path: &Path, cb: &dyn Fn(&DirEntry), dev: Option<u64>, mut stdout: &mut StdoutLock<'_>) {
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
                stdout
                    .write_fmt(format_args!("{} {}\n", transformed, size))
                    .unwrap();
            }
        } else if path.is_dir() {
            visit(&path, &cb, dev, &mut stdout);
        }
    }
}

fn main() -> Result<(), io::Error> {
    let matches = App::new(
        Path::new(&env::args().next().unwrap())
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
    )
    .version("0.1.0")
    .arg(
        Arg::with_name("xdev")
            .help("Do not descend into directories on other filesystems")
            .long("xdev")
            .takes_value(false),
    )
    .arg(
        Arg::with_name("DIRECTORY")
            .help("Sets the root directory to use")
            .required(true)
            .index(1),
    )
    .get_matches();
    let _stdout = io::stdout();
    let mut stdout = _stdout.lock();
    if let Some(ref value) = matches.value_of("DIRECTORY") {
        visit(&Path::new(value), &|_x| {}, None, &mut stdout);
    }
    stdout.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_transform_filename() {
        assert_eq!(super::transform_filename("a/b"), "a;b");
        assert_eq!(super::transform_filename("/a/b"), "a;b");
    }
}
