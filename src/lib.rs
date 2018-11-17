#![forbid(unsafe_code, future_incompatible)]
#![forbid(rust_2018_idioms, rust_2018_compatibility)]
#![deny(missing_debug_implementations, bad_style)]
#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! ## Example
//! ```rust
//! use twitter_card::{Summary, TwitterCard};
//!
//! let card = Summary::builder()
//!   .site("@flickr")
//!   .title("Small Island Developing States Photo Submission")
//!   .desc("View the album on Flickr.")
//!   .image("https://farm6.staticflickr.com/5510/14338202952_93595258ff_z.jpg")
//!   .build();
//! ```
//!
//! ```html
//! <meta name="twitter:card" content="summary" />
//! <meta name="twitter:site" content="@flickr" />
//! <meta name="twitter:title" content="Small Island Developing States Photo Submission" />
//! <meta name="twitter:description" content="View the album on Flickr." />
//! <meta name="twitter:image" content="https://farm6.staticflickr.com/5510/14338202952_93595258ff_z.jpg" />
//! ```

mod summary;
mod summary_large_image;

pub use crate::summary::Summary;
pub use crate::summary_large_image::SummaryLargeImage;

/// Twitter Card.
pub trait TwitterCard {
  /// Convert the Twitter Card to a string.
  fn build(self) -> String;
}

/// Create a Twitter card tag.
///
/// The card type.
///
/// _Used with all cards._
#[inline]
pub fn create_card(content: &str) -> String {
  create("twitter:card", content)
}

/// Create a Twitter site tag.
///
/// @username of website. Either twitter:site or twitter:site:id is required.
///
/// _Used with summary, summary_large_image, app, player cards._
#[inline]
pub fn create_site(content: &str) -> String {
  create("twitter:site", content)
}

/// Create a Twitter site id tag.
///
/// Same as twitter:site, but the user’s Twitter ID. Either twitter:site or
/// twitter:site:id is required.
///
/// _Used with summary, summary_large_image, player cards_
#[inline]
pub fn create_site_id(content: &str) -> String {
  create("twitter:site:id", content)
}

/// Create a Twitter description tag.
///
/// Description of content.
///
/// _Used with summary, summary_large_image, player cards._
///
/// ## Panics.
/// Panics if the description is more than 200 characters.
#[inline]
pub fn create_desc(content: &str) -> String {
  debug_assert!(
    content.len() <= 200,
    "Description has a maximum of 200 characters"
  );
  create("twitter:description", content)
}

/// Create a Twitter title tag.
///
/// Title of content.
///
/// _Used with summary, summary_large_image, player cards._
///
/// ## Panics.
/// Panics if the description is more than 70 characters.
#[inline]
pub fn create_title(content: &str) -> String {
  debug_assert!(content.len() <= 70, "Title has a maximum of 70 characters");
  create("twitter:title", content)
}

/// Create a Twitter image tag.
///
/// URL of image to use in the card. Images must be less than 5MB in size. JPG,
/// PNG, WEBP and GIF formats are supported. Only the first frame of an animated
/// GIF will be used. SVG is not supported.
///
/// _Used with summary, summary_large_image, player cards._
///
#[inline]
pub fn create_image(content: &str) -> String {
  create("twitter:image", content)
}

/// Create a Twitter image alt tag.
///
/// A text description of the image conveying the essential nature of an image
/// to users who are visually impaired. Maximum 420 characters.
///
/// _Used with summary, summary_large_image, player cards._
///
/// ## Panics.
/// Panics if the description is more than 420 characters.
#[inline]
pub fn create_image_alt(content: &str) -> String {
  debug_assert!(
    content.len() <= 420,
    "Image alt has a maximum of 420 characters"
  );
  create("twitter:image:alt", content)
}

/// Create a Twitter creator id tag.
///
/// Twitter user ID of content creator.
///
/// _Used with summary, summary_large_image cards_
#[inline]
pub fn create_creator_id(content: &str) -> String {
  create("twitter:creator:id", content)
}

/// Create a Twitter creator tag.
///
/// @username of content creator.
///
/// _Used with summary_large_image cards._
#[inline]
pub fn create_creator(content: &str) -> String {
  create("twitter:creator", content)
}

/// Create a Twitter player tag.
///
/// HTTPS URL of player iframe
///
/// _Used with player card_
///
#[inline]
pub fn create_player(content: &str) -> String {
  create("twitter:player", content)
}

/// Create a Twitter player width tag.
///
/// Width of iframe in pixels.
///
/// _Used with player card_
///
#[inline]
pub fn create_player_width(content: &str) -> String {
  create("twitter:player:width", content)
}

/// Create a Twitter player height tag.
///
/// Height of iframe in pixels.
///
/// _Used with player card_
///
#[inline]
pub fn create_player_height(content: &str) -> String {
  create("twitter:player:height", content)
}

/// Create a Twitter player stream tag.
///
/// URL to raw video or audio stream.
///
/// _Used with player card_
///
#[inline]
pub fn create_player_stream(content: &str) -> String {
  create("twitter:player:stream", content)
}

/// Create a Twitter app name iPhone tag.
///
/// Name of your iPhone app.
///
/// _Used with app card_
///
#[inline]
pub fn create_app_name_iphone(content: &str) -> String {
  create("twitter:app:name:iphone", content)
}

/// Create a Twitter app id iPhone tag.
///
/// Your app ID in the iTunes App Store (Note: NOT your bundle ID).
///
/// _Used with app card_
///
#[inline]
pub fn create_app_id_iphone(content: &str) -> String {
  create("twitter:app:id:iphone", content)
}

/// Create a Twitter app url iPhone tag.
///
/// Your app’s custom URL scheme (you must include ”://” after your scheme
/// name).
///
/// _Used with app card_
///
#[inline]
pub fn create_app_url_iphone(content: &str) -> String {
  create("twitter:app:url:iphone", content)
}

/// Create a Twitter app name iPad tag.
///
/// Name of your iPad app.
///
/// _Used with app card_
///
#[inline]
pub fn create_app_name_ipad(content: &str) -> String {
  create("twitter:app:name:ipad", content)
}

/// Create a Twitter app id iPad tag.
///
/// Your app ID in the iTunes App Store (Note: NOT your bundle ID).
///
/// _Used with app card_
///
#[inline]
pub fn create_app_id_ipad(content: &str) -> String {
  create("twitter:app:id:ipad", content)
}

/// Create a Twitter app url iPad tag.
///
/// Your app’s custom URL scheme (you must include ”://” after your scheme
/// name).
///
/// _Used with app card_
///
#[inline]
pub fn create_app_url_ipad(content: &str) -> String {
  create("twitter:app:url:ipad", content)
}

/// Create a Twitter app name Google Play tag.
///
/// Name of your Android app.
///
/// _Used with app card_
///
#[inline]
pub fn create_app_name_googleplay(content: &str) -> String {
  create("twitter:app:name:googleplay", content)
}

/// Create a Twitter app id Google Play tag.
///
/// Your app ID in the Google Play Store.
///
/// _Used with app card_
///
#[inline]
pub fn create_app_id_googleplay(content: &str) -> String {
  create("twitter:app:id:googleplay", content)
}

/// Create a Twitter app url Google Play tag.
///
/// Your app’s custom URL scheme.
///
/// _Used with app card_
///
#[inline]
pub fn create_app_url_googleplay(content: &str) -> String {
  create("twitter:app:url:googleplay", content)
}

/// Create an HTML tag
#[inline]
fn create(name: &str, content: &str) -> String {
  format!(r#"<meta name="{}" content="{}" />"#, name, content)
}
