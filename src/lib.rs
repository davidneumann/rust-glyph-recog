#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::{self, Read};
mod glyph_rays;
mod glyph;
mod glyph_dataset;
mod diagnostics;
pub mod glyph_recognizer;
use glyph_rays::GlyphRays;
use glyph_recognizer::RecogKind;

use trees::Node;

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

// struct CandidateMatch<'a> {
//     glyph: &'a Glyph,
//     score: u32,
// }

fn _get_flat_trees(node: &Node<RecogKind>) -> (String, u32) {
    let (my_str, my_val) = match node.data() {
        RecogKind::Match(s, v) => (s.value.clone(), *v),
        RecogKind::Penalty(s) => (String::new(), *s),
    };
    if node.has_no_child() {
        (my_str, my_val)
    }
    else {
        let mut min = std::u32::MAX;
        let mut min_child_str = String::new();
        for i in node.iter() {
            let (child_str, score) = _get_flat_trees(i);
            if score <= min {
                min = score;
                min_child_str = child_str;
            }
        }
        (format!("{}{}", my_str, min_child_str), my_val + min)
    }
}

fn _print_tree(node: &Node<RecogKind>) -> String {
    let val = match node.data() {
        RecogKind::Match(g, score) => format!("{}_{}", g.value.to_string(), score),
        RecogKind::Penalty(score) => format!("{}", score),
    };
    if node.has_no_child() {
        val
    } else {
        format!("{}( {})", val,
        node.iter().fold(String::new(), |s,c| s + &_print_tree(c) + &" "))
    }
}

#[cfg(test)]
mod common {
    use crate::glyph_dataset::GlyphDataset;
    use std::{fs, path::PathBuf};

    pub fn get_bench_assemble() -> (GlyphDataset, Vec<PathBuf>, &'static str) {
        let input_dir = "dats/";
        let dataset = GlyphDataset::build_from_dir(input_dir);

        let dirs:Vec<PathBuf> = fs::read_dir(input_dir).unwrap()
            .filter(|x| x.as_ref().unwrap().path().is_dir())
            .map(|x| x.unwrap().path())
            .collect();

        (dataset, dirs, input_dir)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::glyph_recognizer::{GlyphRecognizer, get_ray_delta};

    use super::*;
    use common::*;
    use trees::tr;

    #[test]
    fn test_samples() {
        let (dataset, dirs, input_dir) = get_bench_assemble();

        dirs.into_iter()
        .filter(|dir| dir.file_name().unwrap().to_str().unwrap() != "overlaps")
        .for_each(|dir| {
            // //for dir in dirs {
            let dir_name = dir.file_name().unwrap().to_str().unwrap().to_owned();
            let c = std::char::from_u32(dir_name.parse::<u32>().unwrap()).unwrap().to_string();
            fs::read_dir(input_dir.to_owned() + dir.file_name().unwrap().to_str().unwrap()).unwrap()
                .into_iter()
                .map(|x| x.unwrap())
                .for_each(|file| {
                    let file_path = file.path();
                    let file_name = file_path.file_name().unwrap().to_str().unwrap();
                    let ray = &GlyphRays::from_file(&(input_dir.to_owned() + &dir_name.to_owned() + "/" + &file_name));
                    let best_match = dataset.get(&ray.width, &ray.height).unwrap().into_iter()
                        .filter(|glyph| (ray.pixels_from_top - glyph.ray.pixels_from_top).abs() <= 2)
                        .min_by_key(|glyph| get_ray_delta(ray, &glyph.ray));
                    assert_eq!(&best_match.as_ref().unwrap().value, &c, "Incorrect match with {}/{}. Expected {} Got {}", &dir_name, &file_name, c, best_match.as_ref().unwrap().value);
                });
        });
    }

    #[test]
    fn test_overlaps() {
        let (_, dirs, input_dir) = get_bench_assemble();

        let recog = GlyphRecognizer::new_from_data_dir(input_dir);

        let overlap_dir = dirs.iter().find(|dir| dir.file_name().unwrap().to_str().unwrap() == "overlaps").unwrap();
        for file in fs::read_dir(overlap_dir).unwrap() .into_iter() .map(|x| x.unwrap()) {
            let file_path = file.path();
            let file_name = file_path.file_name().unwrap().to_str().unwrap();
            let safe_file_name = file_name.trim_end_matches(".dat");
            let correct: String = safe_file_name.split('_').map(|s| std::char::from_u32(s.parse::<u32>().unwrap()).unwrap()).collect();
            let ray = GlyphRays::from_file(&format!("{}/{}/{}", &input_dir, overlap_dir.file_name().unwrap().to_str().unwrap().to_owned(), file_name));
            let paths = recog.get_overlap_paths(&ray);
            let mut tree = tr(RecogKind::Penalty(0));
            for i in paths { tree.push_back(i); }
            let (resolved_str, _) = _get_flat_trees(&tree);
            assert_eq!(correct, resolved_str, "Failed to parse overlap {}. Expected {}. Got {}", file_name, &correct, &resolved_str);
        };
    }

    #[test]
    #[ignore]
    fn test_overlaps_verbose() {
        let (_, dirs, input_dir) = get_bench_assemble();

        let recog = GlyphRecognizer::new_from_data_dir(input_dir);

        let overlap_dir = dirs.iter().find(|dir| dir.file_name().unwrap().to_str().unwrap() == "overlaps").unwrap();
        fs::read_dir(overlap_dir).unwrap()
            .into_iter()
            .map(|x| x.unwrap())
            .for_each(|file| {
                let file_path = file.path();
                let file_name = file_path.file_name().unwrap().to_str().unwrap();
                let safe_file_name = file_name.trim_end_matches(".dat");
                let correct: String = safe_file_name.split('_').map(|s| std::char::from_u32(s.parse::<u32>().unwrap()).unwrap()).collect();
                let ray = GlyphRays::from_file(&format!("{}/{}/{}", &input_dir, overlap_dir.file_name().unwrap().to_str().unwrap().to_owned(), file_name));
                let paths = recog.get_overlap_paths(&ray);
                let mut tree = tr(RecogKind::Penalty(0));
                for i in paths { tree.push_back(i); }
                let (resolved_str, _) = _get_flat_trees(&tree);
                if correct != resolved_str {
                    ray.print_raw();
                }
                assert_eq!(correct, resolved_str, "Failed to parse overlap {}. Expected {}. Got {}", file_name, &correct, &resolved_str);
            });
    }
}

#[cfg(test)]
        mod benchs {
    use std::{fs, path::PathBuf};

    use common::*;
    use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
    use trees::tr;
    use crate::glyph_recognizer::{GlyphRecognizer, get_ray_delta};

    use super::*;
    use test::Bencher;

    use rand::seq::IteratorRandom;


    //#[bench]
    //fn bench_samples_all_single_thread(b: &mut Bencher) {
    //    let (dataset, dirs, input_dir) = get_bench_assemble();

    //    let targets:Vec<&PathBuf> = dirs.iter().filter(|dir| dir.file_name().unwrap().to_str().unwrap() != "overlaps").collect();

    //    b.iter(|| {
    //        targets.iter()
    //            .for_each(|dir:&&PathBuf| {
    //                // //for dir in dirs {
    //                let dir_name = dir.file_name().unwrap().to_str().unwrap().to_owned();
    //                fs::read_dir(input_dir.to_owned() + dir.file_name().unwrap().to_str().unwrap()).unwrap()
    //                    .map(|x| x.unwrap())
    //                    .for_each(|file| {
    //                        let file_path = file.path();
    //                        let file_name = file_path.file_name().unwrap().to_str().unwrap();
    //                        let ray = &GlyphRays::from_file(&(input_dir.to_owned() + &dir_name.to_owned() + "/" + &file_name));
    //                        dataset.get(&ray.width, &ray.height).unwrap().into_iter()
    //                            .filter(|glyph| (ray.pixels_from_top - glyph.ray.pixels_from_top).abs() <= 2)
    //                            .min_by_key(|glyph| get_ray_delta(ray, &glyph.ray));
    //            });
    //        });
    //    });
    //}

    #[bench]
    fn bench_samples_single_item(b: &mut Bencher) {
        let (dataset, dirs, input_dir) = get_bench_assemble();

        let targets:Vec<&PathBuf> = dirs.iter().filter(|dir| dir.file_name().unwrap().to_str().unwrap() != "overlaps").collect();

        let mut rng = rand::thread_rng();
        b.iter(|| {
            let dir = targets.iter().choose(&mut rng).unwrap();
            // //for dir in dirs {
            let dir_name = dir.file_name().unwrap().to_str().unwrap().to_owned();
            let file =
                fs::read_dir(input_dir.to_owned() + dir.file_name().unwrap().to_str().unwrap()).unwrap()
                    .map(|x| x.unwrap())
                    .choose(&mut rng).unwrap();
            let file_path = file.path();
            let file_name = file_path.file_name().unwrap().to_str().unwrap();
            let ray = &GlyphRays::from_file(&(input_dir.to_owned() + &dir_name.to_owned() + "/" + &file_name));
            dataset.get(&ray.width, &ray.height).unwrap().into_iter()
                .filter(|glyph| (ray.pixels_from_top - glyph.ray.pixels_from_top).abs() <= 2)
                .min_by_key(|glyph| get_ray_delta(ray, &glyph.ray));
        });
    }

    #[bench]
    fn bench_samples_all_multi_thread(b: &mut Bencher) {
        let (dataset, dirs, input_dir) = get_bench_assemble();

        let targets:Vec<&PathBuf> = dirs.iter().filter(|dir| dir.file_name().unwrap().to_str().unwrap() != "overlaps").collect();

        b.iter(|| {
            targets.par_iter()
                .for_each(|dir:&&PathBuf| {
                    // //for dir in dirs {
                    let dir_name = dir.file_name().unwrap().to_str().unwrap().to_owned();
                    fs::read_dir(input_dir.to_owned() + dir.file_name().unwrap().to_str().unwrap()).unwrap()
                        .par_bridge()
                        .map(|x| x.unwrap())
                        .for_each(|file| {
                            let file_path = file.path();
                            let file_name = file_path.file_name().unwrap().to_str().unwrap();
                            let ray = &GlyphRays::from_file(&(input_dir.to_owned() + &dir_name.to_owned() + "/" + &file_name));
                            dataset.get(&ray.width, &ray.height).unwrap().into_iter()
                                .filter(|glyph| (ray.pixels_from_top - glyph.ray.pixels_from_top).abs() <= 2)
                                .min_by_key(|glyph| get_ray_delta(ray, &glyph.ray));
                        });
                });
        });
    }

    #[bench]
    fn bench_overlaps_all_multi_threaded(b: &mut Bencher) {
        let (_, dirs, input_dir) = get_bench_assemble();

        let recog = GlyphRecognizer::new_from_data_dir(input_dir);

        let overlap_dir = dirs.iter().find(|dir| dir.file_name().unwrap().to_str().unwrap() == "overlaps").unwrap();
        b.iter(|| {
            fs::read_dir(overlap_dir).unwrap()
                .into_iter()
                .map(|x| x.unwrap())
                .par_bridge()
                .for_each(|file| {
                    let file_path = file.path();
                    let file_name = file_path.file_name().unwrap().to_str().unwrap();
                    let ray = GlyphRays::from_file(&format!("{}/{}/{}", input_dir, overlap_dir.file_name().unwrap().to_str().unwrap().to_owned(), file_name));
                    let paths = recog.get_overlap_paths(&ray);
                    let mut tree = tr(RecogKind::Penalty(0));
                    for i in paths { tree.push_back(i); }
                    _get_flat_trees(&tree);
                });
        });
    }

    //#[bench]
    //fn bench_overlaps_single_thread(b: &mut Bencher) {
    //    let (dataset, dirs, input_dir) = get_bench_assemble();

    //    let recog = GlyphRecognizer {
    //        dataset: &dataset,
    //    };

    //    let overlap_dir = dirs.iter().find(|dir| dir.file_name().unwrap().to_str().unwrap() == "overlaps").unwrap();
    //    b.iter(|| {
    //        fs::read_dir(overlap_dir).unwrap()
    //            .map(|x| x.unwrap())
    //            .into_iter()
    //            .for_each(|file| {
    //                let file_path = file.path();
    //                let file_name = file_path.file_name().unwrap().to_str().unwrap();
    //                let ray = GlyphRays::from_file(&format!("{}/{}/{}", input_dir, overlap_dir.file_name().unwrap().to_str().unwrap().to_owned(), file_name));
    //                let paths = recog.get_overlap_paths(&ray);
    //                let mut tree = tr(RecogKind::Penalty(0));
    //                for i in paths { tree.push_back(i); }
    //                get_flat_trees(&tree);
    //            });
    //    });
    //}

    #[bench]
    fn bench_overlaps_single_item(b: &mut Bencher) {
        let (_, dirs, input_dir) = get_bench_assemble();

        let recog = GlyphRecognizer::new_from_data_dir(input_dir);

        let mut rng = rand::thread_rng();

        let overlap_dir = dirs.iter().find(|dir| dir.file_name().unwrap().to_str().unwrap() == "overlaps").unwrap();
        b.iter(|| {
            let file = fs::read_dir(overlap_dir).unwrap()
                .map(|x| x.unwrap())
                .into_iter()
                .choose(&mut rng).unwrap();
            let file_path = file.path();
            let file_name = file_path.file_name().unwrap().to_str().unwrap();
            let ray = GlyphRays::from_file(&format!("{}/{}/{}", input_dir, overlap_dir.file_name().unwrap().to_str().unwrap().to_owned(), file_name));
            let paths = recog.get_overlap_paths(&ray);
            let mut tree = tr(RecogKind::Penalty(0));
            for i in paths { tree.push_back(i); }
            _get_flat_trees(&tree);
        });
    }
}
