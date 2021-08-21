# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Internal

- Updated to [space-partitioning](https://crates.io/crates/space-partitioning/0.5.0) `0.5.0`
  for intersection test improvements. This update reduced
  the rendering time for the default 1280 x 800 pixel image to 6.1 minutes on
  an i7-6700K CPU @ 4.00GHz with 4 cores + HT.

## 0.3.0 - 2021-08-21

### Internal

- Updated to [space-partitioning](https://crates.io/crates/space-partitioning/0.4.0) `0.4.0`
  for [hashbrown](https://crates.io/crates/hashbrown) sets and use of
  callback functions for intersection candidate generation. This update reduced
  the rendering time for the default 1280 x 800 pixel image to 7.1 minutes on
  an i7-6700K CPU @ 4.00GHz with 4 cores + HT.

## 0.2.0 - 2021-08-18

### Added

- QuadTrees \[sic!\] are now used to speed up ray/sphere intersection tests.
  The default 1280 x 800 pixel render now takes about 10.2 minutes on
  an i7-6700K CPU @ 4.00GHz with 4 cores + HT.

## 0.1.0 - 2021-07-31

### Added

- Initial implementation with row-wise multi-threading. 
  The default 1280 x 800 pixel render takes about 19.5 minutes on
  an i7-6700K CPU @ 4.00GHz with 4 cores + HT.
