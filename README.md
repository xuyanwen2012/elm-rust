# elm-rust

[![Build Status](https://travis-ci.com/xuyanwen2012/elm-rust.svg?branch=master)](https://travis-ci.com/xuyanwen2012/elm-rust)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# Get Started

If you have Rust [(link)](https://www.rust-lang.org/tools/install) installed, then it will be extremely easy do test the code. 

```
git clone https://github.com/xuyanwen2012/elm-rust

cd elm-rust

cargo test --all

```

Or if you do not want to install Rust, then you can still play around with the pre-compiled binary executable (Windows) under the `bin` directory. Expect you won't be able to see all test cases passed.

```
./bin/elm_rust.exe
```

If you don't want to do any of the above, you can also check the *Build Status* bandage at the top of this README file, the repo is checked using Travis CI which will run the test cases for you instead. 


# Modified Syntax of the language

```
e ::= () | n | x | \x:η. e | e1 e2 | e1 ⨁ e2
      | if e1 then e2 else e3 | let x = e1 in e2 | i
      | liftn e: e1 ... en | foldp e1 e2 e3
      | async e

t ::= unit | int | t -> t'
o ::= signal t. | t -> o | o -> o'
η = t | o

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
