use std::{
    fs::{self, ReadDir},
    io,
    path::{Path, PathBuf},
};

use ffprobe::ffprobe;
use inquire::Confirm;
use lexical_sort::{natural_lexical_cmp, PathSort};
use log::*;
use strsim::normalized_damerau_levenshtein as ndl;

use crate::utils::{args_parse::Args, is_close};

pub fn gather(args: &Args, path: ReadDir) -> io::Result<Vec<PathBuf>> {
    let mut file_list = vec![];

    for file in path {
        let file = file?;
        if args.extensions.contains(
            &file
                .path()
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_lowercase(),
        ) {
            file_list.push(file.path());
        }
    }

    file_list.path_sort(natural_lexical_cmp);

    Ok(file_list)
}

pub fn rename_files(
    args: &Args,
    source: Vec<PathBuf>,
    target: &[PathBuf],
) -> io::Result<Vec<PathBuf>> {
    let mut file_list = vec![];
    for (i, s_file) in source.iter().enumerate() {
        let s_folder = s_file.parent().unwrap();

        if let Some(t_file) = target.get(i) {
            let s_str = s_file.file_name().unwrap().to_string_lossy().to_string();
            let t_str = t_file.file_name().unwrap().to_string_lossy().to_string();
            let new_file = s_folder.join(&t_str);

            if s_str != t_str && ndl(&s_str, &t_str) > 0.8 {
                if !new_file.is_file() {
                    info!("Rename file from:\n    <b><magenta>{s_file:?}</></b>\nto:\n    <b><magenta>{new_file:?}</></b>");

                    if args.rename || args.dry {
                        if !args.dry {
                            fs::rename(s_file, &new_file)?;
                        }

                        file_list.push(new_file);
                    } else {
                        let rename = Confirm::new("Rename:")
                            .with_default(false)
                            .prompt()
                            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

                        if rename {
                            fs::rename(s_file, &new_file)?;

                            file_list.push(new_file);
                        }
                    }
                } else {
                    error!("Source file with target name, already exists: <b><magenta>{new_file:?}</></b>");
                }
            } else if s_str == t_str {
                info!("File names match (<b><magenta>{s_str:?}</></b>), nothing to rename...");

                file_list.push(new_file);
            } else if !args.dry {
                warn!("File names differs to much, rename anyway?\n    Old: <b><magenta>{s_file:?}</></b>\n    New: <b><magenta>{new_file:?}</></b>");

                let rename = Confirm::new("Rename:")
                    .with_default(false)
                    .prompt()
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

                if rename {
                    fs::rename(s_file, &new_file)?;

                    file_list.push(new_file);
                }
            } else {
                warn!("File names differs to much\n    Old: <b><magenta>{s_file:?}</></b>\n    New: <b><magenta>{new_file:?}</></b>");
            }
        } else {
            warn!("More files exist in the source folder than in the destination. Check manually: <b><magenta>{s_folder:?}</></b>");
        }
    }

    Ok(file_list)
}

pub fn copy_files(args: &Args, source: Vec<PathBuf>, target: &Path) -> io::Result<()> {
    for file in source {
        let file_name = file.file_name().unwrap();
        let new_file = target.join(file_name);

        if !args.dry {
            if args.check {
                let probe_src =
                    ffprobe(&file).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                let probe_tgt =
                    ffprobe(&new_file).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

                let dur_src = probe_src
                    .format
                    .duration
                    .unwrap_or("0.0".to_string())
                    .parse()
                    .unwrap();
                let dur_tgt = probe_tgt
                    .format
                    .duration
                    .unwrap_or("0.0".to_string())
                    .parse()
                    .unwrap();

                if dur_src > 0.0 && dur_tgt > 0.0 && !is_close(dur_src, dur_tgt, 0.9) {
                    error!("Source and target durations differs: source <yellow>{dur_src:.3?}</>, target: <yellow>{dur_tgt:.3?}</>");

                    let over = Confirm::new("Override:")
                        .with_default(true)
                        .prompt()
                        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

                    if !over {
                        continue;
                    }
                }
            }
            if args.r#override {
                fs::remove_file(&new_file)?;
                fs::copy(&file, &new_file)?;
            } else {
                info!("Move file and override existing:\n    <b><magenta>{file:?}</></b>\nto:\n    <b><magenta>{new_file:?}</></b>");

                let over = Confirm::new("Override:")
                    .with_default(false)
                    .prompt()
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

                if over {
                    fs::remove_file(&new_file)?;
                    fs::copy(&file, &new_file)?;
                }
            }
        }

        info!("Copy file to: <b><magenta>{new_file:?}</></b>");
    }

    Ok(())
}
