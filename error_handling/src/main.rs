use std::fs::File;
use std::io::Read;
use std::process::exit;

// struct configuration {
//     source_file: File,
//     is_dirty: bool,
// }

// fn main() -> Result<(), std::io::Error> {
//     // let file = File::open("invisible.txt")?;
//     // Ok(())

//     reading_file()
// }

fn main() -> Result<(), std::io::Error> {
    let mut content = String::new();
    let file_m = File::open("invisible.txt")?;
    Ok(())
}

// fn main() {
//     let file_m = File::open("invisible.txt");

//     let mut content = String::new();

//     match file_m {
//         Ok(mut f) => {
//             let result = f.read_to_string(&mut content);
//             println!("{:?}", result);
//         }
//         Err(err) => {
//             eprintln!("Error reading {:?}", err);
//             exit(1);
//         }
//     }
// }

// what is ? is  basically the macro of the match
