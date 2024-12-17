use std::{fs::File, io::{empty, Read}, os::unix::raw::blksize_t};

#[derive(Debug)]
enum Block {
    Files (i32, i32), // size, index
    Empty (i32), // size

}

#[derive(Debug)]
struct Disk {
    blocks: Vec<Block>,
}

impl Block {
    fn new_from_char(c: char, index: i32) -> Block {

        if index%2 == 0 {
            return Block::Files(c.to_digit(10).unwrap() as i32, index/2);
        } else {
            return Block::Empty(c.to_digit(10).unwrap() as i32);
        }
    }
}

impl Disk {
    fn new_from_string(s: String) -> Disk {
        let mut blocks: Vec<Block> = Vec::new();
        let mut index = 0;
        for fifi in s.chars() {
            let gugus = Block::new_from_char(fifi, index);
            index += 1;
            blocks.push(gugus);
        }
        Disk {
            blocks,
        }
    }
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    let mut f = File::open("data.csv")?;
    f.read_to_string(&mut buffer)?;
    let loulou = Disk::new_from_string(buffer);
    println!("{:?}", loulou);
    Ok(())
}