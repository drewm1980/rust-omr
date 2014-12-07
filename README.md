# [Optical Music Recognition System](http://en.wikipedia.org/wiki/Music_OCR) in Rust

:construction: :construction: :construction: :construction: :construction: :construction:

Given an image of some typeset sheet music, extract a semantic representation of it:
* Input format: 8-bit grayscale raster image in a format ImageMagck can read.
* Output format: To be determined.  Music XML?  Cleaned up SVG?

Goals for the code:
* Fast
* Minimal Dependencies
* Perfect performance on a well defined subset of [LilyPond](lilypond.org) output

Personal Goals:
* Learn [Rust](http://rust-lang.org)
* Maintain some of my Computer Vision chops
* Make the poor quality scans of sheet music on [todotango](http://www.todotango.com) at least print decently on my girlfriend's poor quality printer
* Maybe someday do automatic transcription to alternative music notations

Non-Goals at this time:
* Supporting handwritten music
* Ports to Mac, Windows

### Disclaimer
This code is being developed as a hobby, in a language that I am still learning, and which has not even hit 1.0. I am only sharing it on github because I develop ~everything in the open.  I have no expectaton that it will be useful any time soon, and you shouldn't either.  Run at your own risk!

### Status
This is what I have actually implemented so far:
* Loading of images directly from .pgm files using pure Rust
* Loading anything image magick can load, provided it is installed
* Currently hacking around with geometry primitives

[![Build Status](https://travis-ci.org/drewm1980/rust-omr.svg?branch=master)](https://travis-ci.org/drewm1980/rust-omr)

### Dependencies
* The Rust Compiler and Standard Library; I am tracking the Nightly versions until rust 1.0
* Runtime dependency on image magick's "convert" utility for loading images
