use std::collections::HashSet;
use std::{cmp, collections::HashMap, fs};
use super::{glyph::Glyph, glyph_rays::GlyphRays};
use super::glyph_recognizer::get_ray_delta;

pub struct GlyphDataset {
    glyph_dict: HashMap<u8, HashMap<u16, Vec<Glyph>>>,
    possible_overlaps: HashSet<String>,
    pub min_width: u16,
}

impl GlyphDataset {
    pub fn build_from_dir(input: &str) -> GlyphDataset {
        //let mut glyph_dict : HashMap<(u16, u8), HashMap<String, GlyphRays>> = HashMap::new();
        let mut glyph_dict : HashMap<u8, HashMap<u16, Vec<Glyph>>> = HashMap::new();
        let mut possible_overlaps = HashSet::new();
        let mut min_height = std::u8::MAX;
        let _debug = input.to_owned() + "overlaps/";
        let _debug1 = fs::read_dir(input.to_owned() + "overlaps/").unwrap().into_iter().map(|x| x.unwrap()).count();
        let mut min_width = std::u16::MAX;
        for dir in fs::read_dir(input.to_owned() + "overlaps/").unwrap().into_iter().filter(|x| x.as_ref().unwrap().path().is_file()) {
            let file_name = dir.unwrap().file_name().to_str().as_ref().unwrap().replace(".dat", "").to_owned();
            let overlaps : Vec<String> = file_name.split("_").to_owned().into_iter().map(|i| std::char::from_u32(i.to_owned().parse::<u32>().unwrap()).unwrap().to_string()).collect();
            //println!("New overlaps {:?}", overlaps);
            let _debug2 = possible_overlaps.len();
            possible_overlaps.extend(overlaps.clone());
        }
        for dir in fs::read_dir(input).unwrap().into_iter().filter(|x| x.as_ref().unwrap().path().is_dir())
        {
            let dir_name = dir.unwrap().file_name().to_str().unwrap().to_owned();
            if dir_name == "overlaps" {
                continue;
            }
            let c = std::char::from_u32(dir_name.parse::<u32>().unwrap()).unwrap().to_string();
            let files = fs::read_dir(input.to_owned() + &dir_name).unwrap()
                .into_iter()
                .filter(|x| x.as_ref().unwrap().path().file_name().unwrap() != "0.dat")
                .map(|x| x.unwrap());
            //Get samples by dimensions
            let mut samples: HashMap<(u16, u8), Vec<GlyphRays>> = HashMap::new();
            for file in files {
                let file_name = file.path().file_name().unwrap().to_str().unwrap().to_owned();
                //println!("Using {} for {} at {},{}", file_name, dir_name, height, width);
                let ray = GlyphRays::from_file(&(input.to_owned() + &dir_name + "/" + &file_name));
                min_height = cmp::min(min_height, ray.height);
                min_width = cmp::min(min_width, ray.width);
                let vec = samples.entry((ray.width, ray.height)).or_insert(Vec::new());
                vec.push(ray);
            }
            //Average samples by dimensions
            let averages: HashMap<&(u16, u8), GlyphRays> = samples.iter().map(|kvp| (kvp.0, GlyphRays::average(kvp.1))).collect();
            //println!("Found {} sizes for {}", averages.len(), &c);

            //Build dataset from average samples
            for kvp in averages {
                //super::diagnostics::print_rays(&kvp.1);
                let max_error = samples.get(kvp.0).unwrap().iter().map(|s| get_ray_delta(&kvp.1, s)).max().unwrap();
                //println!("Adding {} {},{}", &c, kvp.0.0, kvp.0.1);
                glyph_dict.entry(kvp.0.1).or_insert(HashMap::new()).entry(kvp.0.0).or_insert(Vec::new()).push(Glyph{
                    value: c.clone(),
                    max_error,
                    ray: kvp.1,
                });
            }
        }

        //println!("Possible overlaps");
        //for i in &possible_overlaps {
        //    println!("{}", i);
        //}

        //println!("Overlaps check");
        //let _debug3 = possible_overlaps.len();
        //for kvp1 in &glyph_dict {
        //    for kvp2 in kvp1.1 {
        //        for glyph in kvp2.1 {
        //            //println!("{}", &glyph.value);
        //            let _debug5 = possible_overlaps.len();
        //            let _debug4 = possible_overlaps.contains(&glyph.value);
        //            if _debug4 {
        //                //println!("");
        //            }
        //        }
        //    }
        //}

        GlyphDataset{
            glyph_dict,
            possible_overlaps,
            min_width,
        }
    }

    pub fn print_max_errors(&self) {
        for height in self.glyph_dict.keys() {
            for glyphs in self.glyph_dict.get(height).unwrap() {
                for glyph in glyphs.1.iter() {
                    println!("Max error: {},{} \"{}\" = {:?}", glyphs.0, height, glyph.value, glyph.max_error);
                }
            }
        }
    }

    pub fn get(&self, width:&u16, height:&u8) -> Option<&Vec<Glyph>> {
        let _debug = self.glyph_dict.get(height);
        match self.glyph_dict.get(height) {
            Some(d) => d.get(width),
            None => None
        }
    }

    //Scan from left to right getting new valid height
    //Add all those valid heights up into a new vec
    pub fn fuzzy_get(&self, overlap:&GlyphRays) -> Option<Vec<&Glyph>> {
        let mut results = Vec::new();
        for width in self.min_width..=overlap.width {
            let (valid_top, height) = get_valid_info(overlap, width);
            let candidates = self.get(&width, &height);
            match candidates {
                Some(candidates) => {
                    for candidate in candidates.iter().filter(|c| self.possible_overlaps.contains(&c.value)) {
                        if (candidate.ray.pixels_from_top - valid_top).abs() <= 2 {
                            results.push(candidate);
                        }
                    }
                }
                None => (),
            }
        }

        if results.len() > 0 { Some(results) }
        else { None }
    }
}

//Get new valid top of line using t2b
//Get new height from t2b and b2t
fn get_valid_info(overlap: &GlyphRays, width:u16) -> (i8, u8) {
    let mut min_t2b = std::u8::MAX;
    let mut min_b2t = std::u8::MAX;
    for i in 0..width as usize {
        min_t2b = cmp::min(min_t2b, overlap.t2b[i]);
        min_b2t = cmp::min(min_b2t, overlap.b2t[i]);
    }
    let pixels_from_top = overlap.pixels_from_top + (min_t2b as i8);
    let height = overlap.height - min_b2t - min_t2b;
    (pixels_from_top, height)
}

