# hello-cicd
`hello-cicd` is a command-line program for showing how to deploy cross-compiled releases on GitHub using Travis CI.

[![Build Status](https://travis-ci.com/jonstites/hello-cicd.svg?branch=master)](https://travis-ci.com/jonstites/hello-cicd)

## Motivation

CI/CD is like flossing: everyone knows they should do it, but it's not very fun.

Anyway, it turns out that publishing cross-compiled binaries on GitHub for Rust projects is easy, but not super-well documented for some reason.

## Building the Travis CI configuration file

Let start with a minimal configuration that only runs our test:

```yaml
language: rust
rust: stable

script:
  - cargo build 
  - cargo test 

branches:
  only:
    # Pushes and PR to the master branch
    - master
    
notifications:
  email:
    on_success: never
```

You can see that it ran [successfully](https://travis-ci.com/jonstites/hello-cicd/jobs/178401495/config) but that the
[release](https://github.com/jonstites/hello-cicd/releases/tag/v0.1.0) has no binaries.



## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
