use std::{fs::{self, File}, io::{self, Read}, sync::{Arc, Mutex}};
mod glyph_rays;
mod glyph;
mod glyph_dataset;
mod diagnostics;
use glyph_dataset::GlyphDataset;
use glyph_rays::GlyphRays;

use std::time::Instant;
use rayon::prelude::*;

use crate::glyph::Glyph;

// =OVERLAPS= 0/39 is a good (T

fn main() -> io::Result<()> {
    let input2 = "/home/david/Downloads/dats/";

    let dataset = GlyphDataset::build_from_dir(&input2);

    // dataset.print_max_errors();
    // panic!();

    // let input = "/home/david/Downloads/0/";
    // let _ = _parse_file(&(input.to_owned() + "5017.dat"));
    // let rays = GlyphRays::from_file(&(input.to_owned() + "5017.dat"));
    // resolve_overlap(rays, &dataset);
    // panic!();

    // for file in fs::read_dir(input)?{
    //     _parse_file(&(input.to_owned() + file?.path().file_name().unwrap().to_str().unwrap()));
    // }
    // panic!();
    // println!("66/0");
    // let _debug = "test";

    // let debug = rays.get_sub_glyph(5, rays.width - 5);
    // print_rays(rays);
    // print_rays(&debug);


    // let ray2 = &GlyphRays::from_file(&(input.to_owned() + "115.dat"));
    // println!("66/115");
    // print_rays(ray2);
    // println!("Delta to Ref: {}", get_ray_delta(rays, ray2));

    // let input2 = "/home/david/Downloads/dats/";
    // let ray_l = &GlyphRays::from_file(&(input2.to_owned() + "54/229.dat"));
    // println!("54/229");
    // print_rays(ray_l);
    // println!("Delta for l and \"I\": {}", get_ray_delta(ray2, ray_l));
    // //panic!("");


    let dirs:Vec<std::path::PathBuf> = fs::read_dir(input2).unwrap()
        .filter(|x| x.as_ref().unwrap().path().is_dir())
        .map(|x| x.unwrap().path())
        .collect();

    let start = Instant::now();
    let match_results:Vec<bool> = dirs.par_iter()
        .map(|dir| -> Vec<bool> {
            // //for dir in dirs {
            let dir_name = dir.file_name().unwrap().to_str().unwrap().to_owned();
            let c = std::char::from_u32(dir_name.parse::<u32>().unwrap()).unwrap().to_string();
            fs::read_dir(input2.to_owned() + dir.file_name().unwrap().to_str().unwrap()).unwrap()
                .into_iter()
                .par_bridge()
                .map(|x| x.unwrap())
                .map(|file| {
                    let file_path = file.path();
                    let file_name = file_path.file_name().unwrap().to_str().unwrap();
                    let ray = &GlyphRays::from_file(&(input2.to_owned() + &dir_name.to_owned() + "/" + &file_name));
                    let best_match = dataset.get(&ray.width, &ray.height).unwrap().into_iter()
                        .filter(|glyph| (ray.pixels_from_top - glyph.ray.pixels_from_top).abs() <= 2)
                        .min_by_key(|glyph| GlyphDataset::get_ray_delta(ray, &glyph.ray));
                    if &best_match.as_ref().unwrap().value == &c { return true; }
                    else {
                        println!("Incorrect match with {}/{}. Expected {} Got {}", &dir_name, &file_name, c, best_match.as_ref().unwrap().value);
                        return false;
                    }
                })
            .collect()
                //}
        })
    .flatten()
        .collect();

    let mut total = 0;
    let mut correct = 0;
    for did_match in match_results {
        if did_match == true {
            correct += 1;
        }
        total += 1;
    }

    println!("Total: {}. Correct: {}. Took: {:?}", total, correct, start.elapsed());

    //let start = Instant::now();
    //let overlap_dir = "/home/david/Downloads/0/";
    //let total = Arc::new(Mutex::new(0));
    //let matches_found = Arc::new(Mutex::new(0));
    //fs::read_dir(overlap_dir)?
    //    .into_iter()
    //    //.par_bridge()
    //    .map(|x| x.unwrap())
    //    .for_each(|file| {
    //        //for file in files {
    //        let file_path = file.path();
    //        let file_name = file_path.file_name().unwrap().to_str().unwrap();
    //        let ray = GlyphRays::from_file(&(overlap_dir.to_owned() + file_name));
    //        match dataset.get(&ray.width, &ray.height) {
    //            Some(glyphs) => {
    //                let best_match = glyphs.into_iter()
    //                    .filter(|glyph| (ray.pixels_from_top - glyph.ray.pixels_from_top).abs() <= 2)
    //                    .min_by_key(|glyph| GlyphDataset::get_ray_delta(&ray, &glyph.ray));
    //                match best_match {
    //                    Some(best_match) => {
    //                        let error = &GlyphDataset::get_ray_delta(&ray, &best_match.ray);
    //                        if error <= &best_match.max_error {
    //                            println!("{} found a match with {}. Error {}. Known max error {}", file_name, best_match.value, error,
    //                                     &best_match.max_error);
    //                            *matches_found.lock().unwrap() += 1;
    //                        }
    //                        else {
    //                            println!("Overlap detected {}", file_name);
    //                            resolve_overlap(ray, &dataset);
    //                        }
    //                    },
    //                    _ => (),
    //                }
    //            },
    //            _ => (),
    //        }
    //        *total.lock().unwrap() += 1;
    //    });

    // println!("Total overlaps: {}. Found matches: {}. Took {:?}", total.lock().unwrap(), matches_found.lock().unwrap(), start.elapsed());


    Ok(())
}

fn _display_vec_with_max(label: &str, first: &Vec<i32>, second: &Vec<i32>, _max: i32){
    let mut first_clone= first.clone();
    let mut second_clone = second.clone();
    if first_clone.len() < second_clone.len() { first_clone.push(first[first.len() - 1]); }
    else if second_clone.len() < first_clone.len() { second_clone.push(second[second.len() - 1]); }
    println!("{}\n{:?}\n{:?}", label, first_clone, second_clone);
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

fn resolve_overlap(mut overlap: GlyphRays, dataset:&GlyphDataset) {
    println!("Trying to resolve overlap");
    diagnostics::print_rays(&overlap);
    while overlap.width >= dataset.min_width {
        let candidates = dataset.fuzzy_get(&overlap);
        if candidates.is_none() { println!("Found 0 candidates for overlap"); break; }
        let candidates = candidates.unwrap();
        println!("Found {} candidates", candidates.len());
        let mut best_candidate:Option<(u32, &Glyph)> = None;
        for candidate in candidates {
            let sub = overlap.get_sub_glyph(0, candidate.ray.width);
            if sub.is_none() { continue; }
            let sub = sub.unwrap();
            let score = GlyphDataset::get_ray_delta(&sub, &candidate.ray);
            println!("Candidate {} with score {}", &candidate.value, &score);
            let (best_score, _) = best_candidate.get_or_insert((score, candidate));
            if score < *best_score {
                best_candidate = Some((score, candidate));
            }
            //print_rays(&candidate.ray);
        }
        match best_candidate {
            Some((score, glyph)) => {
                println!("Best match was {} with a score of {}", glyph.value, score);
                let new_width = overlap.width - glyph.ray.width;
                if new_width < dataset.min_width { break; }
                let debug = overlap.get_sub_glyph(glyph.ray.width, new_width);
                match debug {
                    Some(debug) => {
                        println!("Remaining after chop");
                        diagnostics::print_rays(&debug);
                        overlap = debug;
                    },
                    _ => break,
                }
            },
            _ => {
                println!("No matches found for overlap");
                break;
            }
        }
    }
    println!("");
}
