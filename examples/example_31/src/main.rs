use std::{
    fs::File,
    io::{BufRead, BufReader, ErrorKind, Write},
};

fn main() {
    let path = "lines.txt";
    let output = File::create(path);
    let mut output: File = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating the file: {:?}", error);
        }
    };
    write!(output, "Just some random text").expect("Failed to write to file");

    let input = File::open(path).unwrap();
    let buffered: BufReader<File> = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    let output2 = File::create("rand.txt");
    let output2: File = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", e),
            },
            _other_error => {
                panic!("Problem opening the file: {:?}", error);
            }
        },
    };
}
