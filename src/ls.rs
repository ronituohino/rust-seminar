use clap::{self, Arg, ArgAction};
use human_bytes::human_bytes;
use std::fs::{DirEntry, read_dir};

mod sorting;

pub fn ls() {
    let cmd = clap::Command::new("rust-sem")
        .arg(
            Arg::new("sort_by_size")
                .long("size")
                .short('s')
                .action(ArgAction::SetTrue)
                .help("Sort by size"),
        )
        .arg(
            Arg::new("sort_by_name")
                .long("name")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Sort by name"),
        );

    let matches = cmd.get_matches();
    let sort_by_size = matches.get_flag("sort_by_size");
    let sort_by_name = matches.get_flag("sort_by_name");

    let result = read_dir(".");
    match result {
        Ok(read_dir) => {
            let mut entries: Vec<DirEntry> = read_dir.map(|x| x.unwrap()).collect();

            sorting::sort_my_files(sort_by_size, sort_by_name, &mut entries);

            entries.into_iter().for_each(|e| {
                let is_dir = e.metadata().unwrap().is_dir();
                let f_len = e.metadata().unwrap().len();

                println!(
                    "{:<15} {:<15} {:<15}",
                    e.file_name().to_string_lossy(),
                    is_dir,
                    human_bytes(f_len as f64)
                );
            });
        }
        Err(_) => panic!("Unable to read file."),
    }
}
