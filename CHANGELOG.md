# Change Log

## 0.4.2 - 2023-10-2

### Changes

- [#5](https://github.com/ichi-h/weighted_rand/pull/5) Thanks [@DanArmor](https://github.com/DanArmor)! - fix: return 1 for gcd_for_slice if all elements are 0.0

## 0.4.1 - 2023-7-3

### Changes

- [#3](https://github.com/ichi-h/weighted_rand/pull/3) Thanks [@ramon54321](https://github.com/ramon54321)! - Allow any Rng implementing type in next_rng
- [#4](https://github.com/ichi-h/weighted_rand/pull/4) Thanks [@Braymatter](https://github.com/Braymatter)! - Added Clone and Default derives to WalkerTable

## 0.3.2 - 2021-11-11

### Changes

- Update basic usage example.
- Change unnecessary `&Vec<T>` to `&[T]`

## 0.3.1 - 2021-11-9

### Changes

- Implementation of `WalkerTable::next_rng()` method.
- Add benchmark test.
- Performance improvement.
  - Fix type of aliases from `Vec<u32>` to `Vec<usize>`. ([b362a3f](https://github.com/ichi-h/weighted_rand/commit/b362a3f11ba1505fd733ed208562c2f91e5b1f2a))
  - Remove unnecessary else statement. ([87909d7](https://github.com/ichi-h/weighted_rand/commit/87909d744339eb4307b1192aaaac7807c13fadaf))
  - Change threshold (`Vec<u32>`) to probabilities (`Vec<f32>`). ([fccb34d](https://github.com/ichi-h/weighted_rand/commit/fccb34d60e3a13c7fefaca09fd594819f15a5ff8))

## 0.3.0 - 2021-11-9

### Changes

- Implementation of `Vec<f32>` type for `index_weights`. [#2](https://github.com/ichi-h/weighted_rand/pull/2)

## 0.2.0 - 2021-11-8

### Changes

- `WalkerTableBuilder::build()` returns `WalkerTable` instead of `Result<WalkerTable>`.

## 0.1.0 - 2021-11-8

- First release.
