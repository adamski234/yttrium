use rand;
use std::fs;
use std::io::Write;

fn main() {
    let max_file_len = 1_000_000;
    let mut file = fs::File::create("./longars.txt").unwrap();
    let char_list = ['{', '}', ':', 'a'];
    loop {
        let to_write = char_list[rand::random::<usize>() % 4];
        file.write(&[to_write as u8]).unwrap();
        if file.metadata().unwrap().len() >= max_file_len {
            break;
        }
    }
    println!("written");
}
