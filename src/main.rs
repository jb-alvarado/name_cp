use std::{fs, io};

use log::*;

mod utils;

use utils::{
    args_parse::init_args,
    file::{copy_files, gather, rename_files},
    logging::init_logging,
};

fn main() -> io::Result<()> {
    let _logger = init_logging()?;
    let args = init_args()?;

    let source_dir = fs::read_dir(&args.source)?;
    let target_dir = fs::read_dir(&args.target)?;
    let source_list = gather(&args, source_dir)?;
    let target_list = gather(&args, target_dir)?;

    if !source_list.is_empty() && !target_list.is_empty() {
        let new_file_list = rename_files(&args, source_list, &target_list)?;
        let target_folder = target_list[0].parent().expect("target parent folder");

        copy_files(&args, new_file_list, target_folder)?;
    } else {
        if source_list.is_empty() {
            error!("Source list is empty!");
        }

        if target_list.is_empty() {
            error!("Target list is empty!");
        }
    }

    Ok(())
}
