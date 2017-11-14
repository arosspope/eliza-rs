# eliza-rs
[![Crates.io](https://img.shields.io/crates/v/eliza.svg)](https://crates.io/crates/eliza)
[![Documentation](https://docs.rs/eliza/badge.svg)](https://docs.rs/eliza)
[![Build Status](https://travis-ci.org/arosspope/eliza.svg?branch=master)](https://travis-ci.org/arosspope/eliza)

This rust binary is an implementation of the early natural language processing computer program **ELIZA**. The original program was developed from 1964 to 1966 at the MIT Artificial Intelligence Laboratory by Joseph Weizenbaum.

## Introduction

![convo](http://i.imgur.com/Z69mFI8.gif)

ELIZA simulates conversation by implementing _pattern matching_ and a _substitution methodology_ that gives users an illusion of understanding on the part of the program. Directives on how to process input are provided by 'scripts', (written originally in MAD-Slip, now in json) which allow ELIZA to engage in discourse by following script rules. The most famous script - [DOCTOR](scripts/doctor.json) - simulates a Rogerian psychotherapist, using rules dictated in the script to respond with non-directional questions to user inputs.

> Weizenbaum, J. (1996), _ELIZA - A computer program for the study of natural language communication between man and machine_, Communications of the ACM, vol 9, issue 1

## Installation

To install this rust binary, one can do so from source or from [crates.io](https://crates.io/crates/eliza). In either case, you need to have the rust compiler and cargo [installed](https://rustup.rs/) on your system.

### From source

//TODO

### From crates.io

//TODO

## Operation

(tree snippet of directory layout eg. script location)

(code snip of command to start)

![running](https://i.imgur.com/RUneq7b.gif)

_________

## Developers

(blurb about how eliza scripts are different to the binary)

(example simple pirate script has been provided)

(read docs for more information on how to develop your own)
