use std::{cmp, collections::HashMap, fs};
use super::{glyph::Glyph, glyph_rays::GlyphRays};

pub struct GlyphDataset {
    glyph_dict: HashMap<u8, HashMap<u16, Vec<Glyph>>>,
    pub min_width: u16,
}

impl GlyphDataset {
    pub fn build_from_dir(input: &str) -> GlyphDataset {
        //let mut glyph_dict : HashMap<(u16, u8), HashMap<String, GlyphRays>> = HashMap::new();
        let mut glyph_dict : HashMap<u8, HashMap<u16, Vec<Glyph>>> = HashMap::new();
        let mut min_height = std::u8::MAX;
        let mut min_width = std::u16::MAX;
        for dir in fs::read_dir(input).unwrap().into_iter().filter(|x| x.as_ref().unwrap().path().is_dir())
        {
            let dir_name = dir.unwrap().file_name().to_str().unwrap().to_owned();
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
                let max_error = samples.get(kvp.0).unwrap().iter().map(|s| GlyphDataset::get_ray_delta(&kvp.1, s)).max().unwrap();
                glyph_dict.entry(kvp.0.1).or_insert(HashMap::new()).entry(kvp.0.0).or_insert(Vec::new()).push(Glyph{
                    value: c.clone(),
                    max_error,
                    ray: kvp.1,
                });
            }
        }

        GlyphDataset{
            glyph_dict,
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
                    for candidate in candidates {
                        if (candidate.ray.pixels_from_top - valid_top).abs() < 2 {
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

impl GlyphDataset {
    pub fn get_ray_delta(r1: &GlyphRays, r2:&GlyphRays) -> u32 {
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
        let mut horiz_delta = 0.0;
        for y in 0..=max_height {
            horiz_delta += get_vec_delta(&r1.l2r, &r2.l2r, y as usize, max_width, 0)     as f64;
            horiz_delta += get_vec_delta(&r1.r2l, &r2.r2l, y as usize, max_width, 0)     as f64;
            horiz_delta += get_vec_delta(&r1.m2r, &r2.m2r, y as usize, max_width / 2, 0) as f64;
            horiz_delta += get_vec_delta(&r1.m2l, &r2.m2l, y as usize, max_width / 2, 0) as f64;
        }
        horiz_delta = horiz_delta / max_height as f64 / max_width as f64;

        //Vertical vecs
        let mut vert_delta = 0.0;
        for x in 0..=max_width  {
            vert_delta += get_vec_delta_u8(&r1.t2b, &r2.t2b, x as usize, max_height, 2)     as f64;
            vert_delta += get_vec_delta_u8(&r1.b2t, &r2.b2t, x as usize, max_height, 2)     as f64;
            vert_delta += get_vec_delta_u8(&r1.m2b, &r2.m2b, x as usize, max_height / 2, 2) as f64;
            vert_delta += get_vec_delta_u8(&r1.m2t, &r2.m2t, x as usize, max_height / 2, 2) as f64;
        }
        vert_delta = vert_delta / max_width as f64 / max_height as f64;

        ((vert_delta + horiz_delta) * 10000.0) as u32
    }
}

fn get_vec_delta_u8(l:&Vec<u8>, r:&Vec<u8>, index:usize, max_value:u8, stretch_limit:usize) -> u8 {
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
