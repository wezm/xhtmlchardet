# xhtmlchardet

Basic character set detection for XML and HTML in Rust.

[![Build Status](https://travis-ci.org/wezm/xhtmlchardet.svg)](https://travis-ci.org/wezm/xhtmlchardet)

[API Documentation](http://doc.wezm.net/xhtmlchardet/)

## Example

```rust
use std::io::Cursor;
extern crate xhtmlchardet;

let text = b"<?xml version=\"1.0\" encoding=\"ISO-8859-1\"?><channel><title>Example</title></channel>";
let mut text_cursor = Cursor::new(text.to_vec());
let detected_charsets: Vec<String> = xhtmlchardet::detect(&mut text_cursor, None);
assert_eq!(detected_charsets, vec!["iso-8859-1".to_string()]);
```

## Resources

* [Documentation](http://doc.wezm.net/xhtmlchardet/)

## Rationale

I wrote a feed crawler that needed to determine the character set of fetched
content so that it could be normalised to UTF-8. Initially I used the
[uchardet] crate but I encountered some situations where it misdetected the
charset. I collected all these edge cases together and built a test suite. Then
I implemented this crate, which passes all of those tests. It uses a fairly
naïve approach derived from [section F of the XML specification][xmlspec].

[uchardet]: https://crates.io/crates/uchardet
[xmlspec]: http://www.w3.org/TR/2004/REC-xml-20040204/#sec-guessing
