<h1>rust-pxtone<br>
  <a href="https://github.com/PieKing1215/rust-pxtone/actions/workflows/rust_build_test.yml"><img alt="GitHub Workflow Status" src="https://img.shields.io/github/workflow/status/PieKing1215/rust-pxtone/Rust%20Build+Test"></a>
  <a href="https://crates.io/crates/pxtone"><img alt="Crates.io" src="https://img.shields.io/crates/v/pxtone"></a>
  <a href="https://github.com/PieKing1215/rust-pxtone/commits/master"><img alt="GitHub commits since latest release" src="https://img.shields.io/github/commits-since/PieKing1215/rust-pxtone/latest"></a>
</h1>

High level bindings to pxtone for Rust

Run `cargo run --release --example full_playback` for a demo.<br>
See [examples/](examples/) for sample code.

This project adds extra layers on top of the original pxtone library in order to provide a more Rust-friendly API.<br>
For low-level bindings, see [rust-pxtone-sys](https://github.com/PieKing1215/rust-pxtone-sys).

The code is structured into an [`interface`](https://github.com/PieKing1215/rust-pxtone/tree/master/src/pxtone/interface) module and an [`og_impl`](https://github.com/PieKing1215/rust-pxtone/tree/master/src/pxtone/og_impl) module.<br>The idea is that the `interface` module contains all of the traits and structure that define the API, and the `og_impl` module contains an implementation of these traits using the original pxtone library via [rust-pxtone-sys](https://github.com/PieKing1215/rust-pxtone-sys).<br>
(The `og_impl` module is controlled by the feature "og-impl" which is on by default)<br>
The hope is that in the future, this will allow building a custom, entirely Rust implementation of pxtone which can implement this interface, making the backend interchangeable (though work has not started on this custom backend yet).

## License

[pxtone](https://pxtone.org/) © [STUDIO PIXEL](https://studiopixel.jp)

rust-pxtone licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
