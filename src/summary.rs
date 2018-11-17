use super::*;

/// Create a summary card.
#[derive(Debug, Clone)]
pub struct Summary {
  strings: Vec<String>,
}

impl Summary {
  /// Create a new instance.
  pub fn builder() -> Self {
    Self {
      strings: vec![create_card("summary")],
    }
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

  /// Description of content.
  ///
  /// ## Panics.
  /// Panics if the description is more than 200 characters.
  #[inline]
  pub fn desc(mut self, content: &str) -> Self {
    self.strings.push(create_desc(content));
    self
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

  /// URL of image to use in the card. Images must be less than 5MB in size. JPG,
  /// PNG, WEBP and GIF formats are supported. Only the first frame of an animated
  /// GIF will be used. SVG is not supported.
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
}

impl TwitterCard for Summary {
  #[inline]
  fn build(self) -> String {
    self.strings.join("\n")
  }
}
