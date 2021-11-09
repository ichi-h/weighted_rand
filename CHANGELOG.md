# Change Log

## 0.3.1 - 2021-11-9

### Added

- Implementation of `WalkerTable::next_rng()` method.
- Add benchmark test.

### Changed

- Performance Improvement.
  - Fix type of aliases from `Vec<u32>` to `Vec<usize>`. ([b362a3f](https://github.com/ichi-h/weighted_rand/commit/b362a3f11ba1505fd733ed208562c2f91e5b1f2a))
  - Remove unnecessary else statement. ([87909d7](https://github.com/ichi-h/weighted_rand/commit/87909d744339eb4307b1192aaaac7807c13fadaf))
  - Change threshold (`Vec<u32>`) to probabilities (`Vec<f32>`). ([fccb34d](https://github.com/ichi-h/weighted_rand/commit/fccb34d60e3a13c7fefaca09fd594819f15a5ff8))

## 0.3.0 - 2021-11-9

### Added

- Implementation of `Vec<f32>` type for `index_weights`. [#2](https://github.com/ichi-h/weighted_rand/pull/2)

## 0.2.0 - 2021-11-8

### Changed

- `WalkerTableBuilder::build()` returns `WalkerTable` instead of `Result<WalkerTable>`.

## 0.1.0 - 2021-11-8

- First release.
