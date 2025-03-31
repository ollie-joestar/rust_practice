use std::env::args;
use std::fs::{self};
use std::path::Path;
use std::path::PathBuf;

fn update_size(size: u128) {
    match size {
        0..1000 => println!("{} bytes", size),
        1000..1000000 => println!("{:.1} kilobytes", size as f64 / 1000.0),
        1000000..1000000000 => println!("{:.1} megabytes", size as f64 / 1000000.0),
        _ => println!("{:.1} gigabytes", size as f64 / 1000000000.0),
    }
}

fn calculate_dir_size(total_size: &mut u128, dir: PathBuf) {
    let metadata = match fs::metadata(dir.clone()) {
        Ok(result) => result,
        _ => return,
    };
    if metadata.is_dir() {
        let folder = match fs::read_dir(dir) {
            Ok(folder) => folder,
            _ => return,
        };
        for entry in folder.flatten() {
            calculate_dir_size(total_size, entry.path().to_path_buf());
        }
    } else {
        *total_size += metadata.len() as u128;
    }
}

fn main() {
    let args = args();
    let mut total_size: u128 = 0;
    for (i, arg) in args.enumerate() {
        if i != 0 {
            let dir = Path::new(&arg);
            calculate_dir_size(&mut total_size, dir.to_path_buf());
        }
    }
    update_size(total_size);
}
