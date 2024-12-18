use std::{fs::File, io::Read};

#[derive(Debug)]
enum Block {
    Files(i32, i32),  // size, index
    Empty(i32),       // size
}

#[derive(Debug)]
struct Disk {
    blocks: Vec<Block>,
}

impl Block {
    fn new_from_char(c: char, index: i32) -> Block {
        match c.to_digit(10) {
            Some(digit) => {
                if index % 2 == 0 {
                    Block::Files(digit as i32, index / 2)
                } else {
                    Block::Empty(digit as i32)
                }
            },
            None => Block::Empty(0),
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

        Disk { blocks }
    }

    fn get_first_empty(&self) -> Option<&Block> {
        // Renvoyer le premier block vide
        for block in &self.blocks {
            if let Block::Empty(..) = block {
                return Some(block);
            }
        }
        None
    }

    fn get_last_filles(&self) -> Option<&Block> {
        // Renvoyer le dernier block de type Files
        for block in self.blocks.iter().rev() {
            if let Block::Files(..) = block {
                return Some(block);
            }
        }
        None
    }
}

fn transpose (){
    
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    let mut f = File::open("data.csv")?;
    f.read_to_string(&mut buffer)?;
    let loulou = Disk::new_from_string(buffer);
    println!("{:?}", loulou);

    Ok(())
}

//a faire
//parcorire le vecteur et déplacer les chiffre vers la gauche en partans de la droite telle que 111...22.33     113322
//utiliser des neouveaux énum, boucle etc...