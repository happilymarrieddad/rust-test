mod content;

use content::media::Media;
use content::catalog::Catalog;

// 'a -> lifetime annotation
// enum MightHaveAValue<'a> {
//     ThereIsAValue(&'a Media),
//     NoValueAvailable
// }

fn main() {
    let audio_book = Media::Audiobook{ title: String::from("An audiobook") };
    let book = Media::Book { title: String::from("A book") , author: String::from("An author")};
    let movie = Media::Movie { title: String::from("A movie"), director: String::from("a director") };
    let podcast = Media::Podcast(543);
    let placeholder = Media::Placeholder;

    // println!("{}",audio_book.description());
    // println!("{}",book.description());
    // println!("{}",movie.description());
    // println!("{}",podcast.description());
    // println!("{}",placeholder.description());

    // print_media(&audio_book);
    // print_media(&book);
    // print_media(&movie);

    let mut catalog = Catalog::new();

    catalog.add(audio_book);
    catalog.add(book);
    catalog.add(movie);
    catalog.add(podcast);
    catalog.add(placeholder);

    //println!("{:#?}", catalog);
    // match catalog.items.get(0) {
    //     Some(value) => {
    //         println!("Item: {:#?}", value);
    //     }
    //     None => {
    //         println!("Nothing at that index")
    //     }
    // }

    // let item = catalog.get_by_index(40);

    // println!("{:#?}", item);

    // match catalog.get_by_index(0) {
    //     MightHaveAValue::ThereIsAValue(value) => {
    //         println!("Item: {:#?}", value);
    //     }
    //     MightHaveAValue::NoValueAvailable => {
    //         println!("No value here")
    //     }
    // }
    // match catalog.get_by_index(500) {
    //     MightHaveAValue::ThereIsAValue(value) => {
    //         println!("Item: {:#?}", value);
    //     }
    //     MightHaveAValue::NoValueAvailable => {
    //         println!("No value here")
    //     }
    // }

    // match catalog.get_by_index(2) {
    //     Some(value) => {
    //         println!("Item: {:#?}", value.description());
    //     }
    //     None => {
    //         println!("No value here")
    //     }
    // }


    let item = catalog.get_by_index(7);

    println!("{:#?}",item.unwrap_or(&Media::Placeholder{}))
}
