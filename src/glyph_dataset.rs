use std::{cmp, collections::HashMap, fs::{self, File}, io::Read};
use super::{glyph::Glyph, glyph_rays::GlyphRays};

pub struct GlyphDataset {
    glyph_dict: HashMap<u8, HashMap<u16, Vec<Glyph>>>,
    min_height: u8,
    pub min_width: u16,
}

impl GlyphDataset {
    pub fn build_from_dir(input: &str) -> GlyphDataset {
        //let mut glyph_dict : HashMap<(u16, u8), HashMap<String, GlyphRays>> = HashMap::new();
        let mut glyph_dict : HashMap<u8, HashMap<u16, Vec<Glyph>>> = HashMap::new();
        let mut min_height = std::u8::MAX;
        let mut min_width = std::u16::MAX;
        for x in fs::read_dir(input).unwrap().into_iter().filter(|x| x.as_ref().unwrap().path().is_dir())
        {
            let dir_name = x.unwrap().file_name().to_str().unwrap().to_owned();
            let c = std::char::from_u32(dir_name.parse::<u32>().unwrap()).unwrap().to_string();
            let files = fs::read_dir(input.to_owned() + &dir_name).unwrap()
                .into_iter()
                .filter(|x| x.as_ref().unwrap().path().file_name().unwrap() != "0.dat")
                .map(|x| x.unwrap());
            for file in files {
                let file_name = file.path().file_name().unwrap().to_str().unwrap().to_owned();
                let (width, height) = get_size_from_dat(&(input.to_owned() + &dir_name + "/" + &file_name));
                let height_dict = glyph_dict.entry(height).or_insert(HashMap::new());
                let width_vec = height_dict.entry(width).or_insert(vec!());
                if !width_vec.iter().any(|g| g.value == c) {
                    //println!("Using {} for {} at {},{}", file_name, dir_name, height, width);
                    let ray = GlyphRays::from_file(&(input.to_owned() + &dir_name + "/" + &file_name));
                    min_height = cmp::min(min_height, ray.height);
                    min_width = cmp::min(min_width, ray.width);
                    width_vec.push(Glyph{
                        value: c.clone(),
                        ray,
                    });
                }
            }
        }

        GlyphDataset{
            glyph_dict,
            min_height,
            min_width,
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

fn get_size_from_dat(input: &str) -> (u16, u8) {
    let mut fin = File::open(input).unwrap();

    let mut buffer = [0; 2];
    fin.read(&mut buffer).unwrap();
    let width = u16::from_le_bytes(buffer);
    let mut buffer = [0; 1];
    fin.read(&mut buffer).unwrap();
    let height = u8::from_le_bytes(buffer);
    (width, height)
}
