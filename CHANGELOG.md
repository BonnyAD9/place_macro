# Changelog

# v1.0.0
- Proper error handling
- More robust input for macros
- Proper arguments to `to_case` macro

# v0.2.0
- add special case for macro ignore, so that it can ignore following
  macro without the parenthesies (e.g `__ignore__ __tail__(lol)` would expand
  to `lol`)
- Add case conversion
