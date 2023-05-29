# Git Ignore Copy (gicopy)
Copies a directory to another location, ignoring files listed in .gitignore files
## Usage
```
Usage: gicopy.exe [OPTIONS] <SOURCE> <TARGET>

Arguments:
  <SOURCE>  The path of the directory to copy from
  <TARGET>  The path of the directory to copy to

Options:
  -v, --verbose                        Be verbose
  -i, --ignore-file <IGNORE_FILE>      The name of the file with the list of files to ignore [default: .gitignore]
  -o, --other-ignored <OTHER_IGNORED>  Other files to ignore [default: .git]
  -h, --help                           Print help
```

## Installation
### Building from soure and installing using cargo
```
cargo install gicopy
```
