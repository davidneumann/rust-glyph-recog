use std::{cmp, fs::{File}, io::{self, Read}};
mod glyph_rays;
use glyph_rays::GlyphRays;

fn main() -> io::Result<()> {
    let input = "/home/david/Downloads/dats/77/";
    let _ = _parse_file(&(input.to_owned() + "0.dat"));
    let rays = &get_rays(&(input.to_owned() + "0.dat"));
    print_rays(rays);
    let ray2 = &get_rays(&(input.to_owned() + "1.dat"));
    println!("Delta: {}", get_ray_delta(rays, ray2));
    print_rays(ray2);
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

fn get_rays(input: &str) -> GlyphRays {
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
    r2l: vec![-1; height as usize], 
    b2t: vec![-1; width as usize],
    m2l: vec![-1; height as usize],
    m2t: vec![-1; width as usize], 
    m2r: vec![width; height as usize],
    m2b: vec![height; width as usize],
    width,
    height, 
    };

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
            if x <  width  / 2 { rays.m2l[(y as usize)] = cmp::max(x, rays.m2l[(y as usize)]); }
            if x >  width  / 2 { rays.m2r[(y as usize)] = cmp::min(x, rays.m2r[(y as usize)]); }
            if y <  height / 2 { rays.m2t[(x as usize)] = cmp::max(y, rays.m2t[(x as usize)]); }
            if y >  height / 2 { rays.m2b[(x as usize)] = cmp::min(y, rays.m2b[(x as usize)]); }
            print!("X");
        }
        else { print! (" ");}
        //println!("x:{}, y:{}, {}", x, y, buffer[0]);
        count += 1;
        if count % width == 0 {println!("");}
    }

    return rays;
}

fn get_ray_delta(r1: &GlyphRays, r2:&GlyphRays) -> i32 {
    let max_width = cmp::max(r1.width, r2.width);
    let max_height = cmp::max(r1.height, r2.height);
    //Horizontal vecs
    let mut delta = 0;
    for y in 0..max_height {
        delta += get_vec_delta(&r1.l2r, &r2.l2r, y as usize, max_width);
        delta += get_vec_delta(&r1.r2l, &r2.r2l, y as usize, 0);
        delta += get_vec_delta(&r1.m2r, &r2.m2r, y as usize, max_width);
        delta += get_vec_delta(&r1.m2l, &r2.m2l, y as usize, 0);
    }

    for x in 0..max_width {
        delta += get_vec_delta(&r1.t2b, &r2.t2b, x as usize, max_height);
        delta += get_vec_delta(&r1.b2t, &r2.b2t, x as usize, 0);
        delta += get_vec_delta(&r1.m2b, &r2.m2b, x as usize, max_height);
        delta += get_vec_delta(&r1.m2t, &r2.m2t, x as usize, 0);
    }

    delta
}

fn get_vec_delta(l:&Vec<i32>, r:&Vec<i32>, index:usize, max_value:i32) -> i32 {
    let left =
        if index < l.len() { l[index] } else { max_value };
    let right = 
        if index < r.len() { r[index] } else { max_value };
    (right - left).abs()
}

fn print_rays(rays: &GlyphRays) -> () {
    println!();
    println!("{:?}", rays.l2r);
    println!("{:?}", rays.r2l);
    println!("{:?}", rays.t2b);
    println!("{:?}", rays.b2t);
    for y in 0..rays.height {
        for x in 0..rays.width {
            if rays.l2r[(y as usize)] == x { print!("X"); }
            else if rays.r2l[(y as usize)] == x { print!("X"); }
            else if rays.t2b[(x as usize)] == y { print!("X"); }
            else if rays.b2t[(x as usize)] == y { print!("X"); }
            else if rays.m2l[(y as usize)] == x { print!("X"); }
            else if rays.m2r[(y as usize)] == x { print!("X"); }
            else if rays.m2t[(x as usize)] == y { print!("X"); }
            else if rays.m2b[(x as usize)] == y { print!("X"); }
            else { print!(" "); }
        }
        println!();
    }
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
    println!("Count: {}", count);
    Ok(())
}
