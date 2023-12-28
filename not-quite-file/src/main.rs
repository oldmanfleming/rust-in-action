mod file;
use crate::file::File;

fn main() {
    let mut file: File = File::new("file.txt", vec![114, 117, 115, 116, 33]);
    let mut buffer: Vec<u8> = vec![];

    file = file.open().unwrap();

    let file_length = file.read(&mut buffer).unwrap();

    file = file.close().unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", file);
    println!("{} is {} bytes long", &file.name, file_length);
    println!("{}", text);
}
