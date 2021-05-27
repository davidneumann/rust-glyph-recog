use std::{cmp, io::Read};

use crate::{glyph::Glyph, glyph_dataset::GlyphDataset, glyph_rays::GlyphRays};
use trees::{Node, Tree, tr};

pub struct GlyphRecognizer<'a> {
    pub dataset: &'a GlyphDataset,
}

pub enum RecogKind<'a> {
    Match(&'a Glyph, u32),
    Penalty(u32),
}

impl GlyphRecognizer<'_> {
    pub fn get_overlap_paths(&self, overlap: &GlyphRays) -> Vec<Tree<RecogKind>> {
        //println!("Overlap: {},{}", overlap.width, overlap.height);
        //overlap.print_raw();
        // if overlap.width < self.dataset.min_width {
        //     println!("To skiny");
        // }
        let mut results = Vec::new();
        let candidates = self.dataset.fuzzy_get(&overlap);
        match candidates {
            None =>  (),//println!("No candidates found"),
            Some(candidates) => {
                for candidate in candidates {
                    //print!("Trying {}", &candidate.value);
                    let sub = overlap.get_sub_glyph(0, candidate.ray.width);
                    if sub.is_none() { continue; }
                    let sub = sub.unwrap();
                    let score = get_ray_delta(&sub, &candidate.ray) as f64;
                    //println!(" score {}. Max error {}", score, candidate.max_error);
                    if score <= f64::max(1500f64, candidate.max_error as f64 * 1.5) {
                        //println!("Passing candidate: {}", candidate.value);
                        //Make entry for this possibly correct item
                        let mut new_node = tr(RecogKind::Match(candidate, score as u32));

                        //Try to find any children
                        let new_width = overlap.width - candidate.ray.width;
                        let sub = overlap.get_sub_glyph(candidate.ray.width, new_width);
                        match sub {
                            Some(sub) => {
                                //sub.print_raw();
                                //println!("Trying to find child children");
                                let children = self.get_overlap_paths(&sub);
                                //println!("{} children found", children.len());
                                for child in children { new_node.push_back(child) }
                            },
                            None => (),//println!("Could not make sub glyph"),
                        }

                        //Add passing candidate
                        results.push(new_node);
                    }
                }
            }
        }
        //Give a penalty to failure to match to anything
        if results.len() == 0 {
            results.push(tr(RecogKind::Penalty(5000 * overlap.width as u32)));
        }
        return results
    }

    pub fn parse_glyph_from_stream<T: Read>(&self, mut stream: T) -> String {
        let ray = GlyphRays::from_read(&mut stream);
        let best_match = self.dataset.get(&ray.width, &ray.height).unwrap().into_iter()
            .filter(|glyph| (ray.pixels_from_top - glyph.ray.pixels_from_top).abs() <= 2)
            .min_by_key(|glyph| get_ray_delta(&ray, &glyph.ray));
        match best_match {
            Some(best) => return best.value.clone(),
            None => {
                let paths = self.get_overlap_paths(&ray);
                let mut tree = tr(RecogKind::Penalty(0));
                for i in paths { tree.push_back(i); }
                let (resolved_str, _) = get_flat_trees(&tree);
                return resolved_str
            }
        }
    }
}

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

fn get_flat_trees(node: &Node<RecogKind>) -> (String, u32) {
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
            let (child_str, score) = get_flat_trees(i);
            if score <= min {
                min = score;
                min_child_str = child_str;
            }
        }
        (format!("{}{}", my_str, min_child_str), my_val + min)
    }
}
