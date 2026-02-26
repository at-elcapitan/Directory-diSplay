use std::path::PathBuf;
use std::borrow::Cow;
use std::{ env };
use std::fs;

mod file;
mod utils;
mod argparser;

fn main() -> std::io::Result<()> {
    // Getting arguments
    let raw_args = env::args().skip(1).collect();
    let parsed_args = match argparser::parse_args(raw_args) {
        Ok(ret) => ret,
        Err(error) => {
            println!("{}", error);
            return Ok(());
        }
    };

    // Displaying help
    if parsed_args.help {
        utils::display_help();
        return Ok(());
    }

    // Displaying version
    if parsed_args.version {
        utils::display_version();
        return Ok(());
    }

    // Getting current dir
    let current_dir: PathBuf = if parsed_args.dir.is_empty() {
        std::env::current_dir().expect("Failed to get current working directory")
    } else {
        PathBuf::from(&parsed_args.dir)
    };

    // Checking wether current dir is empty
    if current_dir.read_dir()?.next().is_none() {
        return Ok(());
    }

    // Loading objects metadata
    let objects: fs::ReadDir = current_dir.read_dir().expect("Error while reading directory");
    let mut files: file::Objects = file::get_objects(
        objects, 
        &parsed_args.all,
        &parsed_args.full_name
    );

    // Displaying the top line and setting the final size
    if let Some(dir_name) = current_dir.file_name() {
        let current_dir_name: Cow<str> = dir_name.to_string_lossy();

        let current_dir_name_len: usize = current_dir_name.chars()
                                                          .count();

        files.max_name_length = files.max_name_length.max(current_dir_name_len);
        utils::display_top_line(
            &files.max_name_length, 
            current_dir_name, 
            current_dir_name_len, 
            &parsed_args.access, 
            &parsed_args.size
        );
    } else {
        utils::display_top_line(
            &files.max_name_length, 
            Cow::Borrowed(""),
            0, 
            &parsed_args.access, 
            &parsed_args.size
        );
    }
    
    // Printing empty line
    println!("  │");
    
    // Displaying dirs
    utils::display_objects(
        files.dirs, 
        &files.max_name_length, 
        &parsed_args.access, 
        &parsed_args.size, 
        false
    );
    // Displaying files
    utils::display_objects(
        files.files, 
        &files.max_name_length, 
        &parsed_args.access, 
        &parsed_args.size, 
        true
    );

    // Displaying size
    if parsed_args.size {
        let size: utils::Format = utils::format_size(
            files.total_files_size as u64
        );

        println!(
            "Total size: {:.2} {}", 
            size.final_size,
            size.size_name
        );
    }

    Ok(())
}
