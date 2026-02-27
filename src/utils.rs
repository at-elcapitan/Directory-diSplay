use std::borrow::Cow;
use crate::file::DisplayableObject;

pub struct Format {
    pub final_size: f64,
    pub size_name: String
}

pub fn format_size(size: u64) -> Format {
    let size = size as f64;
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * KB;
    const GB: f64 = 1024.0 * MB;

    if size < KB {
        Format {
            final_size: size,
            size_name: " B".to_string()
        }
    } else if size < MB {
        Format {
            final_size: size / KB,
            size_name: "KB".to_string()
        }
    } else if size < GB {
        Format {
            final_size: size / MB,
            size_name: "MB".to_string()
        }
    } else {
        Format { 
            final_size: size / GB,
            size_name: "GB".to_string()
        }
    }
}

pub fn display_help() {
    println!("Usage: ds [OPTION] [DIRECTORY]");
    println!();
    println!("Display directory contents");
    println!();
    println!("Options:");
    println!("  -a, --all             display all directory content");
    println!("  -s, --size            display files size");
    println!("  -f, --full-name       disable file name shortening");
    println!("  -A, --access          display access options");
    println!("      --version         display current program version");
    println!("      --help            display this message)");
    println!();
    println!("Directory diSplay home page: <https://github.com/at-elcapitan/Directory-diSplay>");
}

pub fn display_version() {
    println!("ds (Directory diSplay) 2.1.1");
    println!();
    println!(
        "This is free software; see the source for copying conditions. There is NO"
    );
    println!(
        "warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE."
    );
    println!();
    println!("Written by Vladislav 'ElCapitan' Nazarov");
}

pub fn display_top_line(max_length: &usize, 
                        current_dir_name: Cow<str>,
                        current_dir_name_len: usize,
                        flag_access: &bool,
                        flag_size: &bool) {
    let mut additional: String = String::new();

    if *flag_size {
        additional.push_str("   SIZE\t\t")
    }

    if *flag_access {
        additional.push_str("PUSER\tGROUP\tOTHER")
    }

    let spaces = max_length + 7 - current_dir_name_len;

    println!(
        "{}/{}\tTYPE\t\t{}",
        current_dir_name,
        " ".repeat(spaces),
        additional
    );
}

pub fn display_objects(
        objects: Vec<DisplayableObject>, 
        max_length: &usize,
        flag_access: &bool,
        flag_size: &bool,
        final_objects: bool
    ) {
    for (i, obj) in objects.iter().enumerate() {
        let mut flags_data = String::new();
        let objtype = &obj.objtype;

        if *flag_size && (objtype.type_name == "FILE" || objtype.type_name == "EXEC") {
            let format = format_size(obj.size);

            flags_data.push_str(
                &format!("{:.2}\t{}\t", 
                format.final_size,
                format.size_name));
        } else if *flag_size {
            flags_data.push_str("\t -\t")
        }

        if *flag_access {
            let permissions = &obj.permissions;

            flags_data.push_str(&format!(
                " {}\t {}\t {}",
                permissions.user_access,
                permissions.group_access,
                permissions.other_access
            ));
        }

        let symbol;

        if final_objects && i == objects.len() - 1 {
            symbol = String::from("└");
        } else {
            symbol = String::from("├");
        }
        
        println!(
            "  {} {}{:<width$}\x1b[0m\t{}\t\t{flags_data}",
            symbol,
            objtype.display_color,
            obj.objname,
            objtype.type_name,
            width = max_length + 4
        );
    }
}