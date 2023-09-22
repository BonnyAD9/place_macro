# Unreleased
- add special case for macro ignore, so that it can ignore following
  macro without the parenthesies (e.g `__ignore__ __tail__(lol)` would expand
  to `lol`)
- Add case conversion
