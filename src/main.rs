use std::{cmp, fs::{self, File}, io::{self, Read}};
mod glyph_rays;
use glyph_rays::GlyphRays;
use std::collections::HashMap;
use std::time::Instant;

fn main() -> io::Result<()> {
    let input = "/home/david/Downloads/dats/53/";
    let _ = _parse_file(&(input.to_owned() + "199.dat"));
    let rays = &get_rays(&(input.to_owned() + "199.dat"));
    println!("53/199");
    print_rays(rays);

    let ray2 = &get_rays(&(input.to_owned() + "115.dat"));
    println!("53/115");
    print_rays(ray2);
    println!("Delta to Ref: {}", get_ray_delta(rays, ray2));



    let input2 = "/home/david/Downloads/dats/";
    let ray_l = &get_rays(&(input2.to_owned() + "54/229.dat"));
    println!("54/229");
    print_rays(ray_l);
    println!("Delta for l and \"I\": {}", get_ray_delta(ray2, ray_l));
    //panic!("");


    let mut glyph_dict : HashMap<(i32, i32), HashMap<String, GlyphRays>> = HashMap::new();
    //let glyph_dict : Vec<(String, GlyphRays)> = fs::read_dir(input2)?
    for x in fs::read_dir(input2)?.into_iter().filter(|x| x.as_ref().unwrap().path().is_dir())
    {
        let dir_name = x.unwrap().file_name().to_str().unwrap().to_owned();
        let c = std::char::from_u32(dir_name.parse::<u32>().unwrap()).unwrap().to_string();
        let files = fs::read_dir(input2.to_owned() + &dir_name).unwrap()
            .into_iter()
            .filter(|x| x.as_ref().unwrap().path().file_name().unwrap() != "0.dat")
            .map(|x| x.unwrap());
        for file in files {
            let file_name = file.path().file_name().unwrap().to_str().unwrap().to_owned();
            let size = get_sieze_from_dat(&(input2.to_owned() + &dir_name + "/" + &file_name));
            let sub_dict = glyph_dict.entry(size).or_insert(HashMap::new());
            if !sub_dict.contains_key(&c) {
                println!("Using {} for {} at {:?}", file_name, dir_name, &size);
                let ray = get_rays(&(input2.to_owned() + &dir_name + "/" + &file_name));
                glyph_dict.get_mut(&size).unwrap().entry(c.to_string()).or_insert(ray);
            }
        }
    }
    // for glyph in &glyph_dict {
    //     println!("{:?}", glyph.0);
    // }

    let dirs = fs::read_dir(input2)?
        .into_iter()
        .filter(|x| x.as_ref().unwrap().path().is_dir())
        .map(|x| x.unwrap());

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
            let ray = &get_rays(&(input2.to_owned() + &dir_name.to_owned() + "/" + &file_name));
            let best_match = (&glyph_dict[&(ray.width, ray.height)]).into_iter()
                .min_by_key(|x| get_ray_delta(ray, &x.1));
            if best_match.as_ref().unwrap().0 == &c { correct += 1; }
            else {
                println!("Incorrect match with {}/{}. Expected {} Got {}", &dir_name, &file_name, c, best_match.as_ref().unwrap().0);
            }
            total += 1;
        }
    }

    println!("Total: {}. Correct: {}. Took: {:?}", total, correct, start.elapsed());


    // let ray2 = &get_rays("/home/david/Downloads/dats/80/10.dat");
    // let best_match = &glyph_dict.into_iter()
    //     .min_by_key(|x| get_ray_delta(ray2, &x.1));
    // println!("Best match: {}", best_match.as_ref().unwrap().0);

    Ok(())
}

fn get_sieze_from_dat(input: &str) -> (i32, i32) {
    let mut fin = File::open(input).unwrap();

    let mut buffer = [0; 4];
    fin.read(&mut buffer).unwrap();
    let width = i32::from_le_bytes(buffer);
    fin.read(&mut buffer).unwrap();
    let height = i32::from_le_bytes(buffer);
    (width, height)
}

fn get_rays(input: &str) -> GlyphRays {
    //println!("Trying to open {}", input);
    let mut fin = File::open(input).unwrap();

    let mut buffer = [0; 4];
    fin.read(&mut buffer).unwrap();
    let width = i32::from_le_bytes(buffer);
    fin.read(&mut buffer).unwrap();
    let height = i32::from_le_bytes(buffer);
    // println!("{},{}", width, height);

    let mut rays = GlyphRays {
        l2r: vec![width; height as usize],
        t2b: vec![height; width as usize],
        r2l: vec![width; height as usize],
        b2t: vec![height; width as usize],
        m2l: vec![width / 2; height as usize],
        m2t: vec![height / 2; width as usize],
        m2r: vec![width / 2; height as usize],
        m2b: vec![height / 2; width as usize],
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
            rays.r2l[(y as usize)] = cmp::min(width - x - 1, rays.r2l[(y as usize)]);
            rays.t2b[(x as usize)] = cmp::min(y, rays.t2b[(x as usize)]);
            rays.b2t[(x as usize)] = cmp::min(height - y - 1, rays.b2t[(x as usize)]);
            if x <  width  / 2 { rays.m2l[(y as usize)] = cmp::min(width / 2 - x, rays.m2l[(y as usize)]); }
            if x >=  width  / 2 { rays.m2r[(y as usize)] = cmp::min(x - width / 2, rays.m2r[(y as usize)]); }
            if y <  height / 2 { rays.m2t[(x as usize)] = cmp::min(height / 2 - y, rays.m2t[(x as usize)]); }
            if y >=  height / 2 { rays.m2b[(x as usize)] = cmp::min(y - height / 2, rays.m2b[(x as usize)]); }
            // print!("X");
        }
        //else { print! (" ");}
        //println!("x:{}, y:{}, {}", x, y, buffer[0]);
        count += 1;
        //if count % width == 0 {println!("");}
    }

    return rays;
}

fn dispaly_vec_with_max(label: &str, first: &Vec<i32>, second: &Vec<i32>, _max: i32){
    let mut first_clone= first.clone();
    let mut second_clone = second.clone();
    if first_clone.len() < second_clone.len() { first_clone.push(first[first.len() - 1]); }
    else if second_clone.len() < first_clone.len() { second_clone.push(second[second.len() - 1]); }
    println!("{}\n{:?}\n{:?}", label, first_clone, second_clone);
}

fn get_ray_delta(r1: &GlyphRays, r2:&GlyphRays) -> i32 {
    let max_width = cmp::max(r1.width, r2.width) - 1;
    let max_height = cmp::max(r1.height, r2.height) - 1;
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
    let mut delta = 0;
    for y in 0..=max_height {
        delta += get_vec_delta(&r1.l2r, &r2.l2r, y as usize, max_width, 0);
        delta += get_vec_delta(&r1.r2l, &r2.r2l, y as usize, max_width, 0);
        delta += get_vec_delta(&r1.m2r, &r2.m2r, y as usize, max_width / 2, 0);
        delta += get_vec_delta(&r1.m2l, &r2.m2l, y as usize, max_width / 2, 0);
    }

    //Vertical vecs
    for x in 0..=max_width  {
        delta += get_vec_delta(&r1.t2b, &r2.t2b, x as usize, max_height, 2);
        delta += get_vec_delta(&r1.b2t, &r2.b2t, x as usize, max_height, 2);
        delta += get_vec_delta(&r1.m2b, &r2.m2b, x as usize, max_height / 2, 2);
        delta += get_vec_delta(&r1.m2t, &r2.m2t, x as usize, max_height / 2, 2);
    }

    delta
}

fn get_vec_delta(l:&Vec<i32>, r:&Vec<i32>, index:usize, max_value:i32, stretch_limit:usize) -> i32 {
    //let max_value =
    //    if index < l.len() && index < r.len() { cmp::max(l[index], r[index]) }
    //    else if index < l.len() { l[index] }
    //    else { r[index] };
    let left =
        if index < l.len() + stretch_limit { l[cmp::min(index, l.len() - 1)] } else { max_value };
    let right =
        if index < r.len() + stretch_limit { r[cmp::min(index, r.len() - 1)] } else { max_value };
    (right - left).abs()
}

fn print_rays(rays: &GlyphRays) -> () {
    println!();
    println!("l2r {:?}", rays.l2r);
    println!("r2l {:?}", rays.r2l);
    println!("t2b {:?}", rays.t2b);
    println!("b2t {:?}", rays.b2t);
    println!("width {:?} height {:?}", rays.width, rays.height);

    for y in 0..rays.height {
        for x in 0..rays.width {
            if rays.l2r[(y as usize)] == x      { print!("X"); }
            else if rays.width - 1 - rays.r2l[(y as usize)] == x { print!("X"); }
            else if rays.t2b[(x as usize)] == y { print!("X"); }
            else if rays.height - 1 - rays.b2t[(x as usize)] == y { print!("X"); }
            else if rays.width / 2 - rays.m2l[(y as usize)] == x { print!("X"); }
            else if rays.m2r[(y as usize)] + rays.width / 2 == x { print!("X"); }
            else if rays.height / 2 - rays.m2t[(x as usize)] == y { print!("X"); }
            else if rays.m2b[(x as usize)] + rays.height / 2 == y { print!("X"); }
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
    println!("{}: {},{}", &input, width, _height);
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
