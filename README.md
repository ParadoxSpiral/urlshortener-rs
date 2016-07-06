# urlshortener-rs 
[![](https://meritbadge.herokuapp.com/urlshortener)](https://crates.io/crates/urlshortener) [![](https://travis-ci.org/vityafx/urlshortener-rs.svg?branch=master)](https://travis-ci.org/vityafx/urlshortener-rs) [![](https://img.shields.io/badge/docs-online-2020ff.svg)](https://vityafx.github.io/urlshortener-rs/master/urlshortener/)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.md)


A very simple urlshortener for Rust.

This library aims to implement only URL shortener services which do not
require any authentication (ex: goo.gl, bit.ly) and to provide an interface as
minimal and simple as possible.

The reason for this is that users of such a library might only need to get the
shortened URL, rather than a service using authentication. This is also a reason
that this library aims to use as few dependencies as possible.

Currently the following URL shorteners are implemented:

- `bn.gy`
- `is.gd`
- `v.gd`


## Usage

```rust
extern crate urlshortener;

use urlshortener::UrlShortener;

fn main() {
    let us = UrlShortener::new();
    let long_url = "https://google.com";
    println!("Short url for google: {:?}", us.try_generate(long_url));
}
```
