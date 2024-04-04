# Wordle-analyzer


![Project badge](https://img.shields.io/badge/language-Rust-blue.svg)
![Crates.io License](https://img.shields.io/crates/l/wordle-analyzer)
![Gitea Release](https://img.shields.io/gitea/v/release/PlexSheep/wordle-analyzer?gitea_url=https%3A%2F%2Fgit.cscherr.de)
![Gitea language count](https://img.shields.io/gitea/languages/count/PlexSheep/wordle-analyzer?gitea_url=https%3A%2F%2Fgit.cscherr.de)
[![cargo checks and tests](https://github.com/PlexSheep/wordle-analyzer/actions/workflows/cargo.yaml/badge.svg)](https://github.com/PlexSheep/wordle-analyzer/actions/workflows/cargo.yaml)

* [Original Repository](https://git.cscherr.de/PlexSheep/wordle-analyzer)
* [GitHub Mirror](https://github.com/PlexSheep/wordle-analyzer)
* [crates.io](https://crates.io/crates/wordle-analyzer)

[Wordle](https://en.wikipedia.org/wiki/Wordle) is a popular game in which you
have to guess words by slowly guessing the letters contained in it.

`Wordle-analyzer` aims to offer a few things:

* A basic implementation of the Wordle game
* An Interface for solvers for the Wordle game, which can make use of math and
  information theory
* A benchmark for Wordle solvers

## Wordlists

Included in this repository are the following wordlists:

* [3Blue1Brown Top English words -- `./data/wordlists/en_US_3b1b_freq_map.json`](https://github.com/3b1b/videos/tree/master/_2022/wordle/data)
