use std::fs::{ ReadDir, DirEntry };
use std::{
    os::{
        linux::fs::MetadataExt,
        unix::fs::{FileTypeExt, PermissionsExt},
    },
};

const MAX_FILENAME_LEN: usize = 38;

pub struct Permissions {
    pub group_access: String,
    pub user_access: String,
    pub other_access: String
}

pub struct ObjectType {
    pub type_name: String,
    pub display_color: String,
}

pub struct DisplayableObject {
    pub size: u64,
    pub objname: String,
    pub objtype: ObjectType,
    pub permissions: Permissions
}

pub struct Objects {
    pub dirs: Vec<DisplayableObject>,
    pub files: Vec<DisplayableObject>,
    pub max_name_length: usize,
    pub total_files_size: usize
}

fn get_file_type(file: &DirEntry) -> ObjectType {
    let filetype = file.file_type().unwrap();
    
    if filetype.is_dir() {
        ObjectType {
            type_name: String::from("DIR"),
            display_color: String::from("\x1b[34;1m")
        }
    } else if filetype.is_file() 
        && file.metadata().expect("Unable to load file metadata")
                          .permissions().mode() & 0o111 != 0
    {
        ObjectType {
            type_name: String::from("EXEC"),
            display_color: String::from("\x1b[32;1m"),
        }
    } else if filetype.is_file() {
        ObjectType {
            type_name: String::from("FILE"),
            display_color: String::from(""),
        }
    } else if filetype.is_symlink() {
        ObjectType {
            type_name: String::from("SYML"),
            display_color: String::from("\x1b[31;1m"),
        }
    } else if filetype.is_block_device() {
        ObjectType {
            type_name: String::from("BLOCK"),
            display_color: String::from("\x1b[33;1m"),
        }
    } else if filetype.is_char_device() {
        ObjectType {
            type_name: String::from("CHAR"),
            display_color: String::from("\x1b[37;1m"),
        }
    } else {
        ObjectType {
            type_name: String::from("UNDEF"),
            display_color: String::from(""),
        }
    }
}

fn get_file_permissions(file: &DirEntry) -> Permissions {
    let mode = file.metadata()
        .expect("Unable to load file metadata")
        .permissions()
        .mode();
    let mut ret = Permissions {
        group_access: String::from(""),
        user_access: String::from(""),
        other_access: String::from("")
    };

    ret.user_access.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    ret.user_access.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    ret.user_access.push(if mode & 0o100 != 0 { 'x' } else { '-' });

    ret.group_access.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    ret.group_access.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    ret.group_access.push(if mode & 0o010 != 0 { 'x' } else { '-' });

    ret.other_access.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    ret.other_access.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    ret.other_access.push(if mode & 0o001 != 0 { 'x' } else { '-' });

    ret
}

pub fn get_objects(objects: ReadDir, flag_all: &bool, flag_fullname: &bool) -> Objects {
    let mut ret = Objects {
        dirs: Vec::new(),
        files: Vec::new(),
        max_name_length: 0,
        total_files_size: 0
    };

    for obj in objects {
        let obj: DirEntry = obj.unwrap();

        let mut filename = obj
            .file_name()
            .into_string()
            .expect("Error while converting OsString to String");

        if filename.starts_with('.') && !*flag_all {
            continue;
        }

        let name_length = filename.len();
        let filetype = get_file_type(&obj);

        let size = if filetype.type_name == "FILE" || filetype.type_name == "EXEC" {
            obj.metadata()
               .unwrap()
               .st_size()
        } else {
            0
        };

        ret.total_files_size += size as usize;

        if !*flag_fullname && name_length > MAX_FILENAME_LEN {
            let truncated: String = filename.chars()
                                            .take(MAX_FILENAME_LEN)
                                            .collect();
            filename = truncated + "..";
            ret.max_name_length = MAX_FILENAME_LEN + 2;
        } else if ret.max_name_length < name_length {
            ret.max_name_length = name_length;
        }

        let display_obj = DisplayableObject {
            size: size,
            objname: if filetype.type_name == "DIR" { filename + "/" } else { filename.clone() },
            objtype: filetype,
            permissions: get_file_permissions(&obj),
        };

        if display_obj.objtype.type_name == "DIR" {
            ret.dirs.push(display_obj);
        } else {
            ret.files.push(display_obj);
        }
    }

    ret
}