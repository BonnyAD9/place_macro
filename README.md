# Place-macro
Macros you wish you had while you were writing your non-proc macro.

This library privides some macros that make writing regural non-proc
macros much simpler, readable and with less dirty tricks.

The main macro of this library will be `place` but it is not finished yet.
It will be able to expand the macros in this library in reverse expansion
order.

## Simple macros
- `ignore`: expands to nothing
- `identity`: expands to what is given, it will be skipped by the `place` macro
- `dollar`: expands to dollar sign `$`
- `string`: concats the contents into single string, see the doc
- `identifier`: concats the contents into sintle identifier in the same way as
  string
- `head`: expands to the first token
- `tail`: expands to all but the first token
- `start`: expands to all but the last token
- `last`: expands to the last token
- `reverse`: expands to the tokens in reverse order
