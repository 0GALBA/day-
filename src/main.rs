use std::{fs::File, io::{Lines, Read}};

#[derive(Debug, Clone)]
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

    fn get_last_filles(&mut self) -> Option<(Block,usize)> {
        // Renvoyer le dernier block de type Files
        for (i, block,) in self.blocks.iter().rev().enumerate() {
            if let Block::Files(..) = block {
                return Some((block.clone(), self.blocks.len() - i - 1));
            }
        }
        None
    }
}

fn reorganize_blocks(disk: &mut Disk) {
    for ref mut pos in 0..disk.blocks.len() {
        let block = disk.blocks.get(*pos).unwrap();
        if let Block::Empty(empty_size) = *block {
            if let Some((b, last_pos)) = disk.get_last_filles() {
                if last_pos < *pos {
                    break;
                }

                if let Block::Files(file_size, file_index) = b {
                    if file_size <= empty_size {
                        let file = Block::Files(file_size, file_index);
                        disk.blocks.remove(*pos);
                        disk.blocks.insert(*pos, file);
                        disk.blocks.remove(last_pos);
                        let file = Block::Empty(empty_size - file_size);
                        disk.blocks.insert(*pos + 1, file);
                    } else if empty_size < file_size {
                        let file = Block::Files(empty_size, file_index);
                        disk.blocks.remove(*pos);
                        disk.blocks.insert(*pos, file);
                        *pos += 1;
                        let file = Block::Files(file_size - empty_size, file_index);
                        disk.blocks.remove(last_pos);
                        disk.blocks.insert(last_pos, file);
                        
                    }
                }
            }
        }
    }
}
//faire une première passe ou on fais en sorte de ne rien supprimer
//regarder le 1er empty et reconaitre sa longeur pour ajouter la meme longeure a la place du empty a partir du fichier suit a cela supprimer le empty et la longuerre du empty au fichier.
//insert        remouve 

fn calcul(disk:Disk){
    let mut truc:i128 = 0;
    let mut res:i128 = 0;
    for block in disk.blocks {
        if let Block::Files(size, id) = block {
            for _i in 0..size {
                res += truc * id as i128;
                truc += 1;
            }
        } else if let Block::Empty(size) = block {
            truc += size as i128;
        }
    }
    println!("{}", res);
}
//multiplier chaque chiffre avec sont index et aditionner tout les chiffre


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    let mut f = File::open("data.csv")?;
    f.read_to_string(&mut buffer)?;
    let mut loulou = Disk::new_from_string(buffer);
    reorganize_blocks(&mut loulou);
    println!("{:?}", loulou.blocks);
    calcul(loulou);
    Ok(())
}

//a faire
//parcorire le vecteur et déplacer les chiffre vers la gauche en partans de la droite telle que 111...22.33     113322
//utiliser des neouveaux énum, boucle etc...