#![warn(rust_2018_idioms)]
#![warn(clippy::pedantic)]

use std::env;
use std::fs::{metadata, read_dir};
use std::io::{self, BufWriter, StdoutLock, Write};
use std::os::linux::fs::MetadataExt;
use std::path::Path;

use clap::{crate_name, crate_version, Arg, ArgAction, Command};

fn transform_filename(filename: &str) -> String {
    filename.trim_start_matches('/').replace('/', ";")
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct XDev(u64);

fn visit(
    path: &Path,
    filter_xdev: bool,
    dev: Option<XDev>,
    mut writer: &mut dyn Write,
) -> io::Result<()> {
    let dev = dev.or(Some(XDev(metadata(path)?.st_dev())));

    for entry in read_dir(path)? {
        let path = entry?.path();
        if path.is_file() {
            let meta = metadata(&path)?;
            let size = meta.len();
            let current_dev_id = XDev(meta.st_dev());
            if filter_xdev {
                if let Some(dev_id) = dev {
                    if dev_id != current_dev_id {
                        return Ok(());
                    }
                }
            }
            if let Some(filename_str) = path.to_str() {
                let transformed = transform_filename(filename_str);
                writer.write_fmt(format_args!("{transformed} {size}\n"))?;
            }
        } else if path.is_dir() {
            visit(&path, filter_xdev, dev, &mut writer)?;
        }
    }

    Ok(())
}

fn main() -> Result<(), io::Error> {
    let matches = Command::new(crate_name!())
        .version(crate_version!())
        .arg(
            Arg::new("xdev")
                .help("Do not descend into directories on other filesystems")
                .long("xdev")
                .num_args(0)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("buffered")
                .help("Buffer IO")
                .long("buffered")
                .num_args(0)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("DIRECTORY")
                .help("Sets the root directory to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let stdout = io::stdout();
    let stdout: StdoutLock<'_> = stdout.lock();
    let mut writer: &mut Box<dyn Write> = &mut if matches.get_flag("buffered") {
        Box::new(BufWriter::new(stdout))
    } else {
        Box::new(stdout)
    };
    let filter_xdev = matches.get_flag("xdev");
    if let Some(ref value) = matches.get_one::<String>("DIRECTORY").map(String::as_str) {
        visit(Path::new(value), filter_xdev, None, &mut writer)?;
    }

    writer.flush()?;

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
