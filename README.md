# twitter-card
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Generate HTML for Twitter Card integration.

- [Documentation][8]
- [Crates.io][2]
- [Releases][releases]

## Examples
__Basic usage__
```rust
use twitter_card::{Summary, TwitterCard};

let card = Summary::builder()
  .site("@flickr")
  .title("Small Island Developing States Photo Submission")
  .desc("View the album on Flickr.")
  .image("https://farm6.staticflickr.com/5510/14338202952_93595258ff_z.jpg")
  .build();
```

```html
<--! Output -->
<meta name="twitter:card" content="summary" />
<meta name="twitter:site" content="@flickr" />
<meta name="twitter:title" content="Small Island Developing States Photo Submission" />
<meta name="twitter:description" content="View the album on Flickr." />
<meta name="twitter:image" content="https://farm6.staticflickr.com/5510/14338202952_93595258ff_z.jpg" />
```

## Installation
```sh
$ cargo add twitter-card
```

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

## References
- [Twitter Card Validator](https://cards-dev.twitter.com/validator)
- [Twitter Card Overview](https://developer.twitter.com/en/docs/tweets/optimize-with-cards/overview/markup)

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/twitter-card.svg?style=flat-square
[2]: https://crates.io/crates/twitter-card
[3]: https://img.shields.io/travis/chooxide/twitter-card/master.svg?style=flat-square
[4]: https://travis-ci.org/chooxide/twitter-card
[5]: https://img.shields.io/crates/d/twitter-card.svg?style=flat-square
[6]: https://crates.io/crates/twitter-card
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/twitter-card

[releases]: https://github.com/chooxide/twitter-card/releases
[contributing]: https://github.com/chooxide/twitter-card/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/chooxide/twitter-card/labels/good%20first%20issue
[help-wanted]: https://github.com/chooxide/twitter-card/labels/help%20wanted
