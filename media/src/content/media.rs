#[derive(Debug)]
pub enum Media {
    Book { title: String, author: String },
    Movie {title: String, director: String},
    Audiobook {title: String},
    Podcast(u32),
    Placeholder
}

impl Media {

    pub fn description(&self) -> String {
        /* Cool but way too verbose 
        if let Media::Book{title, author} = self {
            format!("Book: {} {}", title, author)
        } else if let Media::Movie{title, director} = self {
            format!("Movie: {} {}", title, director)
        } else if let Media::Audiobook{title} = self {
            format!("AudioBook: {}", title)
        } else {
            String::from("Media description")
        }
        */

        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("AudioBook: {}", title)
            }
            Media::Podcast(episode_number) => {
                format!("AudioBook: {}", episode_number)
            }
            _default => {
                format!("Unknown type")
            }
        }
    }
}