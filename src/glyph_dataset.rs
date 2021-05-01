use std::{collections::HashMap, fs::{self, File}, io::Read};
use super::{glyph::Glyph, glyph_rays::GlyphRays};

pub struct GlyphDataset {
    glyph_dict: HashMap<u8, HashMap<u16, Vec<Glyph>>>,
}

impl GlyphDataset {
    pub fn build_from_dir(input: &str) -> GlyphDataset {
        //let mut glyph_dict : HashMap<(u16, u8), HashMap<String, GlyphRays>> = HashMap::new();
        let mut glyph_dict : HashMap<u8, HashMap<u16, Vec<Glyph>>> = HashMap::new();
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
                    println!("Using {} for {} at {},{}", file_name, dir_name, height, width);
                    let ray = GlyphRays::from_file(&(input.to_owned() + &dir_name + "/" + &file_name));
                    width_vec.push(Glyph{
                        value: c.clone(),
                        ray,
                    });
                }
            }
        }

        GlyphDataset{
            glyph_dict,
        }
    }

    pub fn get(&self, width:&u16, height:&u8) -> Option<&Vec<Glyph>> {
        let _debug = self.glyph_dict.get(height);
        match self.glyph_dict.get(height) {
            Some(d) => d.get(width),
            None => None
        }
    }
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
