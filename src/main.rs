use clap::{Arg, App};
use colored::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use walkdir::WalkDir;

fn main() {
    let template = "\
Usage: grep [OPTIONS] <pattern> <files...>\n\
Options:\n\
-i               Case-insensitive search\n\
-n               Print line numbers\n\
-v               Invert match (exclude lines that match the pattern)\n\
-r               Recursive directory search\n\
-f               Print filenames\n\
-c               Enable colored output\n\
-h, --help       Show help information";

    let app = App::new("grep")
        .template(template) 
        .arg(Arg::with_name("pattern")
            .required_unless_one(&["help"])
            .index(1))
        .arg(Arg::with_name("files")
            .required_unless_one(&["help"])
            .multiple(true)
            .index(2))
        .arg(Arg::with_name("insensitive")
            .short("i"))
        .arg(Arg::with_name("linenum")
            .short("n"))
        .arg(Arg::with_name("invert")
            .short("v"))
        .arg(Arg::with_name("recursive")
            .short("r"))
        .arg(Arg::with_name("filename")
            .short("f"))
        .arg(Arg::with_name("colored")
            .short("c"))
        .arg(Arg::with_name("help")
            .short("h"));

    let matches = app.clone().get_matches();

    if matches.is_present("help") {
        app.write_help(&mut std::io::stdout()).unwrap();
        println!();  
        return;
    }

    let pattern = matches.value_of("pattern").unwrap();
    let files = matches.values_of("files").unwrap().collect::<Vec<_>>();
    let insensitive = matches.is_present("insensitive");
    let linenum = matches.is_present("linenum");
    let invert = matches.is_present("invert");
    let recursive = matches.is_present("recursive");
    let filename = matches.is_present("filename");
    let colored = matches.is_present("colored");

    if recursive {
        for file_path in files {
            for entry in WalkDir::new(file_path).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    search_in_file(pattern, entry.path().to_str().unwrap(), insensitive, linenum, invert, filename, colored);
                }
            }
        }
    } else {
        for file_path in files {
            search_in_file(pattern, file_path, insensitive, linenum, invert, filename, colored);
        }
    } 
}

fn search_in_file(pattern: &str, file_path: &str, insensitive: bool, linenum: bool, invert: bool, filename: bool, colored: bool) {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut search_line: String;
    let mut search_pattern: String;
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if insensitive{
            search_line = line.to_lowercase();
            search_pattern = pattern.to_lowercase();
        }else {
            search_line = line.clone();
            search_pattern = pattern.to_string();
        };
        let search_contains = search_line.contains(&search_pattern);
        // search_contains = True: search line contains target
        // search_contains = False: search line doesn't contain target
        // invert = True: print lines without target
        // invert = False: print lines with target 
        // search_contains = True, invert = False -> print lines with target
        // search_contains = False, invert = True -> print lines without target 
        
        if search_contains != invert {
            let mut output = String::new();
            if filename {
                output.push_str(&format!("{}: ", file_path));
            }
            if linenum {
                output.push_str(&format!("{}: ", index + 1));
            }
            // Don't add color if both -c and -v are active
            let apply_coloring = colored && !invert; 
            if apply_coloring {
                // Perform case-insensitive replacement while preserving the original case
                let mut last_index = 0;
                let mut colored_output = String::new();
                let lower_pattern_len = search_pattern.len();
                // Search for the case-insensitive matches and color the parts
                for (start, _) in search_line.match_indices(&search_pattern) {
                    // push chars before the match
                    colored_output.push_str(&line[last_index..start]);
                    // add color to the matching part (original case preserved)
                    colored_output.push_str(&line[start..start + lower_pattern_len].red().to_string());
                    // Update last_index to continue searching
                    last_index = start + lower_pattern_len;
                }
                // add the rest of the line
                colored_output.push_str(&line[last_index..]);
                output.push_str(&colored_output);
            } else {
                output.push_str(&line);
            }
            println!("{}", output);
        }
    }
}
