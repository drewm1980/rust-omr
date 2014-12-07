# Rust Optical Music Recognition System

Given an image of some typeset sheet music, extract a semantic representation of it.

Input format: 8-bit grayscale raster image in a format ImageMagck can read.
Output format: To be determined.  Music XML?  Cleaned up SVG?

Goals for the code:
* Fast
* Minimal Dependencies
* Usable as a command-line tool
* Make the poor quality scans of sheet music on [todotango](http://www.todotango.com) print decently on my girlfriend's poor quality printer.
* Maybe someday do automatic transcription to alternative music notations

Personal Goals:
* Learn [Rust](http://rust-lang.org)
* Maintain my Computer Vision chops

### Disclaimer
This is some throw-away code that I am hacking on as a way to learn rust.  
It is being written before rust has even hit 1.0, so it is probably NOT worth
your time to even look at as an example.  I am only sharing it on github
becaues I develop ~everything in the open.

### Status
This is what I have actually implemented so far:
* Loading of images directly from .pgm files using pure Rust
* Loading anything image magick can load, provided it is installed
* Currently hacking around with geometry primitives

[![Build Status](https://travis-ci.org/drewm1980/rust-omr.svg?branch=master)](https://travis-ci.org/drewm1980/rust-omr)

### Dependencies
* The Rust Compiler and Standard Library; I am tracking the Nightly versions until rust 1.0
* Runtime dependency on image magick's "convert" utility for loading images
