## 2018-11-18, Version 1.0.1
### Commits
- [[`b04dd4aefb`](https://github.com/rust-net-web/twitter-card/commit/b04dd4aefb3eebc4a5cfa65f0367862b38611f2c)] (cargo-release) version 1.0.1 (Yoshua Wuyts)
- [[`64667356f4`](https://github.com/rust-net-web/twitter-card/commit/64667356f4e65a57fe508e0d65e8768e4d85f7bb)] fix missing apis (Yoshua Wuyts)
- [[`040b4a0c97`](https://github.com/rust-net-web/twitter-card/commit/040b4a0c97f04655da13c15b6c0a274979cf9c5a)] cargo fmt (Yoshua Wuyts)
- [[`616ca0c980`](https://github.com/rust-net-web/twitter-card/commit/616ca0c9808cbb0369dd9f3c5c1a2b187ac43580)] Fix typo (#1) (Jan-Erik Rediger)
- [[`8989f074b9`](https://github.com/rust-net-web/twitter-card/commit/8989f074b9768bbd0f348e126e493edb424a5ec1)] Update changelog (Yoshua Wuyts)

### Stats
```diff
 CHANGELOG.md  | 38 ++++++++++++++++++++++++++++++++++++++
 Cargo.toml    |  2 +-
 src/app.rs    |  2 +-
 src/lib.rs    |  6 ++++++
 src/player.rs |  6 +++---
 tests/test.rs |  2 +-
 6 files changed, 50 insertions(+), 6 deletions(-)
```


## 2018-11-17, Version 1.0.0
### Commits
- [[`62da38ba71`](https://github.com/rust-net-web/twitter-card/commit/62da38ba71a679a5ab799216aa66a25cc97595a5)] (cargo-release) version 1.0.0 (Yoshua Wuyts)
- [[`19eabf0fae`](https://github.com/rust-net-web/twitter-card/commit/19eabf0faefd25edfb81ba622793907975aa54f7)] app card + finalize (Yoshua Wuyts)
- [[`cb8cb958fe`](https://github.com/rust-net-web/twitter-card/commit/cb8cb958feed5994370896b61233d0eae3d4a5ff)] player (Yoshua Wuyts)
- [[`5a9a45993e`](https://github.com/rust-net-web/twitter-card/commit/5a9a45993e2d3b3dfe00df38a518f6db66fb57d4)] summary large image (Yoshua Wuyts)
- [[`7ac8b97875`](https://github.com/rust-net-web/twitter-card/commit/7ac8b97875c766b8d858c696bace36815fd70709)] init (Yoshua Wuyts)

### Stats
```diff
 .github/CODE_OF_CONDUCT.md                |  75 ++++++++-
 .github/CONTRIBUTING.md                   |  63 ++++++-
 .github/ISSUE_TEMPLATE.md                 |   9 +-
 .github/ISSUE_TEMPLATE/bug_report.md      |  23 ++-
 .github/ISSUE_TEMPLATE/feature_request.md |  43 ++++-
 .github/ISSUE_TEMPLATE/question.md        |  18 ++-
 .github/PULL_REQUEST_TEMPLATE.md          |  21 ++-
 .github/stale.yml                         |  17 ++-
 .gitignore                                |   7 +-
 .travis.yml                               |  13 +-
 CERTIFICATE                               |  37 ++++-
 Cargo.toml                                |  16 ++-
 LICENSE-APACHE                            | 190 ++++++++++++++++++++-
 LICENSE-MIT                               |  21 ++-
 README.md                                 |  68 +++++++-
 examples/example.rs                       |  14 +-
 rustfmt.toml                              |   2 +-
 src/app.rs                                | 121 ++++++++++++-
 src/error.rs                              |  73 ++++++++-
 src/lib.rs                                | 301 +++++++++++++++++++++++++++++++-
 src/player.rs                             | 118 ++++++++++++-
 src/summary.rs                            |  85 +++++++++-
 src/summary_large_image.rs                |  92 +++++++++-
 tests/test.rs                             |   9 +-
 24 files changed, 1436 insertions(+)
```


