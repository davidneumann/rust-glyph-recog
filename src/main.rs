use std::{cmp, fs::{self, File}, io::{self, Read}};
mod glyph_rays;
mod glyph;
mod glyph_dataset;
use glyph_dataset::GlyphDataset;
use glyph_rays::GlyphRays;
use std::collections::HashMap;
use std::time::Instant;

// =OVERLAPS= 0/39 is a good (T

fn main() -> io::Result<()> {
    //let input = "/home/david/Downloads/dats/0/";
    ////for file in fs::read_dir(input)?{
    ////    _parse_file(&(input.to_owned() + file?.path().file_name().unwrap().to_str().unwrap()));
    ////}
    ////panic!();
    //let _ = _parse_file(&(input.to_owned() + "0.dat"));
    //let rays = &GlyphRays::from_file(&(input.to_owned() + "0.dat"));
    //println!("66/0");
    //let _debug = "test";
    //print_rays(rays);

    //let ray2 = &GlyphRays::from_file(&(input.to_owned() + "115.dat"));
    //println!("66/115");
    //print_rays(ray2);
    //println!("Delta to Ref: {}", get_ray_delta(rays, ray2));

    //let input2 = "/home/david/Downloads/dats/";
    //let ray_l = &GlyphRays::from_file(&(input2.to_owned() + "54/229.dat"));
    //println!("54/229");
    //print_rays(ray_l);
    //println!("Delta for l and \"I\": {}", get_ray_delta(ray2, ray_l));
    ////panic!("");


    let input2 = "/home/david/Downloads/dats/";
    // Update this to use glyph_dataset
    let dataset = GlyphDataset::build_from_dir(&input2);
    let dirs = fs::read_dir(input2)?
        .into_iter()
        .filter(|x| x.as_ref().unwrap().path().is_dir())
        .map(|x| x.unwrap());

    let mut max_error: HashMap<(u16, u8, &String), i32> = HashMap::new();
    let start = Instant::now();
    let mut correct = 0;
    let mut total = 0;
    for dir in dirs {
        let dir_name = dir.file_name().to_str().unwrap().to_owned();
        let c = std::char::from_u32(dir_name.parse::<u32>().unwrap()).unwrap().to_string();
        let files = fs::read_dir(input2.to_owned() + dir.path().file_name().unwrap().to_str().unwrap())?
            .into_iter()
            .filter(|x| x.as_ref().unwrap().path().file_name().unwrap() != "0.dat")
            .map(|x| x.unwrap());
        for file in files {
            let file_path = file.path();
            let file_name = file_path.file_name().unwrap().to_str().unwrap();
            let ray = &GlyphRays::from_file(&(input2.to_owned() + &dir_name.to_owned() + "/" + &file_name));
            let best_match = dataset.get(&ray.width, &ray.height).into_iter()
                .filter(|glyph| (ray.pixels_from_top - glyph.ray.pixels_from_top).abs() <= 2)
                .min_by_key(|glyph| get_ray_delta(ray, &glyph.ray));
            let score = get_ray_delta(ray, &best_match.unwrap().ray) as i32;
            let key = (ray.width, ray.height, &best_match.unwrap().value);
            if score > *max_error.entry(key).or_insert(0) {
                *max_error.get_mut(&key).unwrap() = score;
            }
            if &best_match.as_ref().unwrap().value == &c { correct += 1; }
            else {
                println!("Incorrect match with {}/{}. Expected {} Got {}", &dir_name, &file_name, c, best_match.as_ref().unwrap().value);
            }
            total += 1;
        }
    }

    println!("Total: {}. Correct: {}. Took: {:?}", total, correct, start.elapsed());
    //for item in max_error {
    //    println!("Max error: {:?} = {:?}", item.0, item.1);
    //}


    // let ray2 = &GlyphRays::from_file("/home/david/Downloads/dats/80/10.dat");
    // let best_match = &glyph_dict.into_iter()
    //     .min_by_key(|x| get_ray_delta(ray2, &x.1));
    // println!("Best match: {}", best_match.as_ref().unwrap().0);

    Ok(())
}

fn dispaly_vec_with_max(label: &str, first: &Vec<i32>, second: &Vec<i32>, _max: i32){
    let mut first_clone= first.clone();
    let mut second_clone = second.clone();
    if first_clone.len() < second_clone.len() { first_clone.push(first[first.len() - 1]); }
    else if second_clone.len() < first_clone.len() { second_clone.push(second[second.len() - 1]); }
    println!("{}\n{:?}\n{:?}", label, first_clone, second_clone);
}

fn get_ray_delta(r1: &GlyphRays, r2:&GlyphRays) -> u32 {
    let max_width = cmp::max(r1.width, r2.width) - 1;
    let max_height = (cmp::max(r1.height, r2.height) - 1) as u16;
    let _debug = cmp::max(r1.r2l.len(), r2.r2l.len());

    //println!("Max width: {}. Max height: {}", max_width, max_height);
    //dispaly_vec_with_max("l2r", &r1.l2r, &r2.l2r, max_width);
    //dispaly_vec_with_max("r2l", &r1.r2l, &r2.r2l, max_width);
    //dispaly_vec_with_max("t2b", &r1.t2b, &r2.t2b, max_height);
    //dispaly_vec_with_max("b2t", &r1.b2t, &r2.b2t, max_height);
    //dispaly_vec_with_max("m2l", &r1.m2l, &r2.m2l, max_width);
    //dispaly_vec_with_max("m2r", &r1.m2r, &r2.m2r, max_width);
    //dispaly_vec_with_max("m2t", &r1.m2t, &r2.m2t, max_height);
    //dispaly_vec_with_max("m2b", &r1.m2b, &r2.m2b, max_height);

    //Horizontal vecs
    let mut horiz_delta = 0.0;
    for y in 0..=max_height {
        horiz_delta += get_vec_delta(&r1.l2r, &r2.l2r, y as usize, max_width, 0)     as f64;
        horiz_delta += get_vec_delta(&r1.r2l, &r2.r2l, y as usize, max_width, 0)     as f64;
        horiz_delta += get_vec_delta(&r1.m2r, &r2.m2r, y as usize, max_width / 2, 0) as f64;
        horiz_delta += get_vec_delta(&r1.m2l, &r2.m2l, y as usize, max_width / 2, 0) as f64;
    }
    horiz_delta = horiz_delta / max_width as f64;

    //Vertical vecs
    let mut vert_delta = 0.0;
    for x in 0..=max_width  {
        vert_delta += get_vec_delta(&r1.t2b, &r2.t2b, x as usize, max_height, 2)     as f64;
        vert_delta += get_vec_delta(&r1.b2t, &r2.b2t, x as usize, max_height, 2)     as f64;
        vert_delta += get_vec_delta(&r1.m2b, &r2.m2b, x as usize, max_height / 2, 2) as f64;
        vert_delta += get_vec_delta(&r1.m2t, &r2.m2t, x as usize, max_height / 2, 2) as f64;
    }
    vert_delta = vert_delta / max_height as f64;

    ((vert_delta + horiz_delta) * 100.0) as u32
}

fn get_vec_delta(l:&Vec<u16>, r:&Vec<u16>, index:usize, max_value:u16, stretch_limit:usize) -> u16 {
    //let max_value =
    //    if index < l.len() && index < r.len() { cmp::max(l[index], r[index]) }
    //    else if index < l.len() { l[index] }
    //    else { r[index] };
    let left =
        if index < l.len() + stretch_limit { l[cmp::min(index, l.len() - 1)] } else { max_value };
    let right =
        if index < r.len() + stretch_limit { r[cmp::min(index, r.len() - 1)] } else { max_value };
    if right > left {
        right - left
    }
    else {
        left - right
    }
}

fn print_rays(rays: &GlyphRays) -> () {
    println!();
    println!("l2r {:?}", rays.l2r);
    println!("r2l {:?}", rays.r2l);
    println!("t2b {:?}", rays.t2b);
    println!("b2t {:?}", rays.b2t);
    println!("m2l {:?}", rays.m2l);
    println!("m2r {:?}", rays.m2r);
    println!("m2t {:?}", rays.m2t);
    println!("m2b {:?}", rays.m2b);
    println!("width {:?} height {:?}", rays.width, rays.height);

    let height = rays.height as u16;
    for y in 0..height {
        for x in 0..rays.width {
            if rays.l2r[(y as usize)] == x      { print!("X"); }
            else if rays.width - 1 - rays.r2l[(y as usize)] == x { print!("X"); }
            else if rays.t2b[(x as usize)] == y { print!("X"); }
            else if height - 1 - rays.b2t[(x as usize)] == y { print!("X"); }
            else if rays.width / 2 - rays.m2l[(y as usize)] == x { print!("X"); }
            else if rays.m2r[(y as usize)] + rays.width / 2 == x { print!("X"); }
            else if height / 2 - rays.m2t[(x as usize)] == y { print!("X"); }
            else if rays.m2b[(x as usize)] + height / 2 == y { print!("X"); }
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

    let mut buffer = [0; 2];
    fin.read(&mut buffer)?;
    let width = u16::from_le_bytes(buffer);
    let mut buffer = [0; 1];
    fin.read(&mut buffer)?;
    let height = u8::from_le_bytes(buffer);
    fin.read(&mut buffer)?;
    let _pixels_from_top = u8::from_le_bytes(buffer);
    let mut count = 0;
    println!("{}: {},{} {}", &input, width, height, _pixels_from_top);
    let mut buffer = [0; 1];
    loop {
        let read = fin.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        for i in 0..8{
            let pixel = (buffer[0] & (1 << 7 - i)) != 0;
            if !pixel {
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
    }
    println!("Count: {}", count);
    Ok(())
}
