use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn ReadFile(pathText: &str) -> std::fs::File
{
    let s = pathText.to_string();
    let path = Path::new(&s);
    let display = path.display();

    let f = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.to_string()),

        Ok(file) => file,
    };

    return f;
}

pub fn ReadFileToRead(a: &str) -> BufReader<File>
{
    let file = ReadFile(a);
    return BufReader::new(file);
}

pub fn ReadTxtToVec(a: &str, v: &mut Box<Vec<String>>)
{
    //let mut v = Vec::<String>::with_capacity(1024);
    let file = ReadFileToRead(a);
    for line in file.lines() {
        (*v).push(line.unwrap());
    }
}
