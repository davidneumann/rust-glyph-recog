use std::{fs::File, io::{self, Read}};

fn main() -> io::Result<()> {
    let input = "/home/david/Downloads/dats/65/0.dat";
    let mut fin = File::open(input)?;

    let mut buffer = [0; 4];
    fin.read(&mut buffer)?;
    let width = i32::from_le_bytes(buffer);
    fin.read(&mut buffer)?;
    let _height = i32::from_le_bytes(buffer);
    let mut count = 0;
    println!("{},{}", width, _height);
    let mut buffer = [0; 1];
    loop {
        let read = fin.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        if buffer[0] == 0 {
            print!(" ");
        }
        else {
            let c = std::char::from_u32(65).unwrap();
            print!("{}", c);
        }
        count += 1;
        if count % width == 0 {
            println!("");
        }
    }
    Ok(())
}
