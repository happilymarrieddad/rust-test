use super::media::Media; // super is the parent module
// use crate::content::media::Media; <---- could do it this way too

#[derive(Debug)]
pub struct Catalog {
    items: Vec<Media>
}

impl Catalog {
    pub fn new() -> Self {
        Catalog { items: vec![] }
    }

    pub fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            // Good! We have something to return.
            return Some(&self.items[index]);
        }
        
        None
    }
}