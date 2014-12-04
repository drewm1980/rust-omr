This is some throw-away code that I am hacking on as a way to learn rust.  

The intention is to gradually build a toy Optical Music Recognition sytem in Rust.

This was inspired by: 
* Wanting to learn Rust
* I have a background and interest in both music and Computer Vision
* The main open source OMR app crashes on launch for me

It is being written before rust has even hit 1.0, so it is probably NOT worth
your time to even look at as an example.  I am only sharing it on github
becaues I develop ~everything in the open.

What has been implemented so far:
nothing; I am still bogged down on image file parsing and learning the toolchain

[![Build Status](https://travis-ci.org/drewm1980/rust-omr.svg?branch=master)](https://travis-ci.org/drewm1980/rust-omr)

Dependencies:
* Rust
* Runtime dependency on image magick's "convert" utility,
	assuming you want to load anything besides an 8 bit pgm file.
