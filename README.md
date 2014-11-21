
TIFF Format support for Rust
============================

- Author: Gavin Baker <gavinb@antonym.org>
- Date: 25th March 2014

Introduction
------------

This package provides native support for reading and writing a subset of
image files according to the TIFF specification.  TIFF is a complex format,
with many variations, flavours, custom tags and loose interpretations of the
spec.  This does not attempt to provide comprehensive support for all
formats.  The goal of the current implementation is to support strict saving
of greyscale and RGB images, and loading similarly conformant images.

Purpose
-------

The aim of this module was to learn Rust by developing a native non-trivial
library.  A secondary goal was to provide a useful codec for my image
processing work.  Hopefully it is useful for you too!

Status
------

This library is highly experimental. Not only is it incomplete, but it is
tracking the Rust `master` branch, which means it could break at any time.

Platform Support
----------------

There are no platform-specific features used in this library, so it should
work identically on all platforms supported by the Rust toolchain.

Building
--------

This package is available as a crate on the new
[crates.io](http://crates.io) repository.  To use this package, simply
add the following line to your `[dependencies]` section in `Cargo.toml`:

    rust-tiff = "0.0.5"

Acknowledgements
----------------

My thanks to the members of the Rust community on #rust on irc.mozilla.org
for all their patient guidance while developing this project.

 - Melbourne, Autumn 2014
