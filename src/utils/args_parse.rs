use std::{io, path::PathBuf, process::exit};

use clap::Parser;
use inquire::Confirm;
use log::*;

#[derive(Parser, Debug, Default, Clone)]
#[clap(version,
    about = "Name CP",
    long_about = Some("Compare two folders, apply names from files in second folder
 to files from first folder and copy files from A -> B.
    "),
next_line_help = false,
)]
pub struct Args {
    #[clap(
        long,
        help = "Dry run: do not perform any file operations, only print the various operations"
    )]
    pub dry: bool,

    #[clap(long, help = "Check media duration from source and target file")]
    pub check: bool,

    #[clap(short, long, help = "Source folder (copy from)")]
    pub source: PathBuf,

    #[clap(short, long, help = "Target folder (copy to)")]
    pub target: PathBuf,

    #[clap(short, long, help = "File extensions to search for", num_args = 1..)]
    pub extensions: Vec<String>,

    #[clap(short, long, help = "Rename files automatically")]
    pub rename: bool,

    #[clap(short, long, help = "Override files automatically")]
    pub r#override: bool,
}

pub fn init_args() -> io::Result<Args> {
    let mut args = Args::parse();

    if !args.source.is_dir() {
        error!(
            "Source directory <b><magenta>{:?}</></b> not exists!",
            args.source
        );
        exit(1);
    }

    if !args.target.is_dir() {
        error!(
            "Target directory <b><magenta>{:?}</></b> not exists!",
            args.target
        );
        exit(1);
    }

    if args.extensions.is_empty() {
        error!("Need a list of entenions to look for, like: <b><magenta>-e mp4 mkv webm</></b>");
        exit(1);
    }

    if args.rename {
        warn!("Are you sure you want to rename the files automatically? You can also decide individually file by file.");

        args.rename = Confirm::new("Rename all:")
            .with_default(false)
            .prompt()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    }

    if args.r#override {
        warn!("Are you sure you want to overwrite the target files automatically? You can also decide individually file by file.");

        args.r#override = Confirm::new("Override all:")
            .with_default(false)
            .prompt()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    }

    Ok(args)
}
