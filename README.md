
# Directory-diSplay (DS)

A simple directory tree display application was created to make getting information about a folder easier.

### Installation

### Description

Directory-diSplay (DS) is a simple yet powerful command-line application that displays the current directory structure. It provides various options to customize the output, such as displaying file sizes, and access options for easier readability in different terminal environments.

### Usage

To display the current directory structure, use the following command:

```sh
ds [OPTION] [DIRECTORY]
```

### Options

- `-а, --all`          : Display all directory content
- `-f, --full-name`    : Disable file name shortening
- `-s, --size`         : Display file sizes
- `-A, --access`       : Display access options
- `--version`          : Display current program version
- `--help`             : Display help message

### Example

Just displaying directory tree:

```sh
ds
```

Displaying directory tree with file sizes and full names:

```sh
ds -sf
```

or

```sh
ds --size --full-name
```

