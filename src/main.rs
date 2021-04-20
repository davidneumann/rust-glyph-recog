use std::{cmp, convert::TryInto, fs::{self, File}, io::{self, Read}};

fn main() -> io::Result<()> {
    let input = "/home/david/Downloads/dats/65/";
    get_rays(&(input.to_owned() + "0.dat"));
    _parse_file(&(input.to_owned() + "0.dat"));
    //let mut max_width = 0;
    //for dir in fs::read_dir(input)? {
    //    let dir = dir?;
    //    let debug = dir.path();
    //    let debug2 = debug.file_name().unwrap().to_str().unwrap();
    //    max_width = cmp::max(max_width, _get_width(&(input.to_owned() + debug2)));
    //    println!("{}", debug2);
    //}
    //println!("Max width: {}", max_width);
    Ok(())
}

fn get_rays(input: &str) -> () {
    println!("Trying to open {}", input);
    let mut fin = File::open(input).unwrap();

    let mut buffer = [0; 4];
    fin.read(&mut buffer).unwrap();
    let width = i32::from_le_bytes(buffer);
    fin.read(&mut buffer).unwrap();
    let height = i32::from_le_bytes(buffer);
    println!("{},{}", width, height);

    let mut rays = GlyphRays { l2r: vec![width; height as usize],
    t2b: vec![height; width as usize], 
    r2l: vec![0; height as usize], 
    b2t: vec![0; width as usize] };

    //    let mut l2r = vec![width; height as usize];

    let mut buffer = [0; 1];

    let mut count = 0;
    loop {
        let read = fin.read(&mut buffer).unwrap();
        if read == 0 {
            break;
        }

        let x = count % width;
        let y = count / width;
        if buffer[0] != 0 {
            rays.l2r[(y as usize)] = cmp::min(x, rays.l2r[(y as usize)]);
            rays.r2l[(y as usize)] = cmp::max(x, rays.r2l[(y as usize)]);
            rays.t2b[(x as usize)] = cmp::min(y, rays.t2b[(x as usize)]);
            rays.b2t[(x as usize)] = cmp::max(y, rays.b2t[(x as usize)]);
            print!("X");
        }
        else { print! (" ");}
        //println!("x:{}, y:{}, {}", x, y, buffer[0]);
        count += 1;
        if count % width == 0 {println!("");}
    }
    println!();
    println!("{:?}", rays.l2r);
    println!("{:?}", rays.r2l);
    println!("{:?}", rays.t2b);
    println!("{:?}", rays.b2t);
}

fn _get_width(input: &str) -> i32{
    println!("Trying to open {}", input);
    let mut fin = File::open(input).unwrap();

    let mut buffer = [0; 4];
    fin.read(&mut buffer).unwrap();
    return i32::from_le_bytes(buffer);
}

fn _parse_file(input: &str) -> io::Result<()> {
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


struct GlyphRays{
    l2r: Vec<i32>,
    t2b: Vec<i32>,
    r2l: Vec<i32>,
    b2t: Vec<i32>
}
