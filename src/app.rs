use super::*;

/// Create an app card.
///
/// The App Card is a great way to represent mobile applications on Twitter and
/// to drive installs. We designed the App Card to allow for a name, description
/// and icon, and also to highlight attributes such as the rating and the price.
///
/// [Read more](https://developer.twitter.com/en/docs/tweets/optimize-with-cards/overview/app-card)
#[derive(Debug, Clone)]
pub struct App {
  strings: Vec<String>,
}

impl App {
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

  /// Your app ID in the iTunes App Store (Note: NOT your bundle ID).
  #[inline]
  pub fn app_id_iphone(mut self, content: &str) -> Self {
    self.strings.push(create_app_id_iphone(content));
    self
  }

  /// Name of your iPhone app.
  #[inline]
  pub fn app_name_iphone(mut self, content: &str) -> Self {
    self.strings.push(create_app_name_iphone(content));
    self
  }

  /// Your app’s custom URL scheme (you must include ”://” after your scheme
  /// name).
  #[inline]
  pub fn app_url_iphone(mut self, content: &str) -> Self {
    self.strings.push(create_app_url_iphone(content));
    self
  }

  /// Name of your iPad app.
  #[inline]
  pub fn app_name_ipad(mut self, content: &str) -> Self {
    self.strings.push(create_app_name_ipad(content));
    self
  }

  /// Create a Twitter app id iPad tag.
  ///
  /// Your app ID in the iTunes App Store (Note: NOT your bundle ID).
  #[inline]
  pub fn app_id_ipad(mut self, content: &str) -> Self {
    self.strings.push(create_app_id_ipad(content));
    self
  }

  /// Your app’s custom URL scheme (you must include ”://” after your scheme
  /// name).
  #[inline]
  pub fn app_url_ipad(mut self, content: &str) -> Self {
    self.strings.push(create_app_url_ipad(content));
    self
  }

  /// Name of your Android app.
  #[inline]
  pub fn app_name_googleplay(mut self, content: &str) -> Self {
    self.strings.push(create_app_name_googleplay(content));
    self
  }

  /// Your app ID in the Google Play Store.
  #[inline]
  pub fn app_id_googleplay(mut self, content: &str) -> Self {
    self.strings.push(create_app_id_googleplay(content));
    self
  }

  /// Your app’s custom URL scheme.
  #[inline]
  pub fn app_url_googleplay(mut self, content: &str) -> Self {
    self.strings.push(create_app_url_googleplay(content));
    self
  }
}

impl TwitterCard for App {
  #[inline]
  fn build(self) -> String {
    self.strings.join("\n")
  }
}
