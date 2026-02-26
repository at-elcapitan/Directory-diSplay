pub struct Args {
    pub help: bool,
    pub version: bool,
    pub access: bool,
    pub all: bool,
    pub size: bool,
    pub full_name: bool,
    pub dir: String
}

pub fn parse_args(args: Vec<String>) -> Result<Args, String> {
    let mut ret = Args {
        help: false,
        version: false,
        access: false,
        all: false,
        size: false,
        full_name: false,
        dir: String::from("")
    };

    for mut arg in args {
        match arg.as_str() {
            "--help" => {
                ret.help = true;
                break;
            },
            "--version" => {
                ret.version = true;
                break;
            },
            "--access" => ret.access = true,
            "--all" => ret.all = true,
            "--size" => ret.size = true,
            "--full-name" => ret.full_name = true,
            _ => {
                if !arg.starts_with("-") {
                    ret.dir = arg;
                    continue;
                }
                
                assert_eq!(arg.remove(0), '-');

                for c in arg.chars() {
                    match c {
                        'h' => {              
                            ret.help = true;
                            break;
                        },
                        'A' => ret.access = true,
                        'a' => ret.all = true,
                        's' => ret.size = true,
                        'f' => ret.full_name = true,
                        _ => {
                            return Err(
                                format!("Unknown argument -{}. Use -h or --help for arguments list", c)
                            );
                        }
                    }
                }
            }
        }
    }

    Ok(ret)
}