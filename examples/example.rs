extern crate twitter_card;

use twitter_card::{Summary, TwitterCard};

fn main() {
  let card = Summary::builder()
    .site("@flickr")
    .title("Small Island Developing States Photo Submission")
    .desc("View the album on Flickr.")
    .image("https://farm6.staticflickr.com/5510/14338202952_93595258ff_z.jpg")
    .build();

  println!("{}", card);
}
