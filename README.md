# elm-rust

[![Build Status](https://travis-ci.com/xuyanwen2012/elm-rust.svg?branch=master)](https://travis-ci.com/xuyanwen2012/elm-rust)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# Modified Syntax of the language

```
e ::= () | n | x | \x: η | e1 e2 | e1 ⨁ e2
      | if e1 then e2 else e3 | let x = e1 in e2 | i
      | liftn e: e1 ... en | foldp e1 e2 e3
      | async e
```


# Reference

```
@article{czaplicki2013asynchronous,
  title={Asynchronous functional reactive programming for GUIs},
  author={Czaplicki, Evan and Chong, Stephen},
  journal={ACM SIGPLAN Notices},
  volume={48},
  number={6},
  pages={411--422},
  year={2013},
  publisher={ACM New York, NY, USA}
}
```
