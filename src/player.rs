use super::*;

/// Create a player card.
///
/// Player cards allow rich media to be embedded on Twitter. This includes, but
/// is not limited to, audio clips and videos.
///
/// [Read more](https://developer.twitter.com/en/docs/tweets/optimize-with-cards/overview/player-card)
#[derive(Debug, Clone)]
pub struct Player {
  strings: Vec<String>,
}

impl Summary {
  /// Create a new instance.
  pub fn builder() -> Self {
    Self {
      strings: vec![create_card("summary")],
    }
  }

  /// Title of content.
  ///
  /// ## Panics.
  /// Panics if the description is more than 70 characters.
  #[inline]
  pub fn title(mut self, content: &str) -> Self {
    self.strings.push(create_title(content));
    self
  }

  /// @username of website. Either twitter:site or twitter:site:id is required.
  #[inline]
  pub fn site(mut self, site: &str) -> Self {
    self.strings.push(create_site(site));
    self
  }

  /// Same as twitter:site, but the user’s Twitter ID. Either twitter:site or
  /// twitter:site:id is required.
  #[inline]
  pub fn site_id(mut self, content: &str) -> Self {
    self.strings.push(create_site_id(content));
    self
  }

  /// Twitter user ID of content creator.
  #[inline]
  pub fn creator_id(mut self, content: &str) -> Self {
    self.strings.push(create_creator_id(content));
    self
  }

  /// Description of content.
  ///
  /// ## Panics.
  /// Panics if the description is more than 200 characters.
  #[inline]
  pub fn desc(mut self, content: &str) -> Self {
    self.strings.push(create_desc(content));
    self
  }

  /// URL of image to use in the card. Images must be less than 5MB in size.
  /// JPG, PNG, WEBP and GIF formats are supported. Only the first frame of an
  /// animated GIF will be used. SVG is not supported.
  #[inline]
  pub fn image(mut self, content: &str) -> Self {
    self.strings.push(create_image(content));
    self
  }

  /// A text description of the image conveying the essential nature of an image
  /// to users who are visually impaired. Maximum 420 characters.
  ///
  /// ## Panics.
  /// Panics if the description is more than 420 characters.
  #[inline]
  pub fn image_alt(mut self, content: &str) -> Self {
    self.strings.push(create_image_alt(content));
    self
  }

  /// HTTPS URL of player iframe
  #[inline]
  pub fn player(mut selfcontent: &str) -> Self {
    self.strings.push(create_player(content));
    self
  }

  /// Width of iframe in pixels.
  #[inline]
  pub fn player_width(mut self, content: &str) -> Self {
    self.strings.push(create_player_width(content));
    self
  }

  /// Height of iframe in pixels.
  #[inline]
  pub fn player_height(mut self, content: &str) -> Self {
    self.strings.push(create_player_height(content));
    self
  }

  /// URL to raw video or audio stream.
  #[inline]
  pub fn player_stream(mut self, content: &str) -> Self {
    self.strings.push(create_player_stream(content));
    self
  }
}

impl TwitterCard for Player {
  #[inline]
  fn build(self) -> String {
    self.strings.join("\n")
  }
}
