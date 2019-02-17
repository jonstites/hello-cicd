# hello-cicd
`hello-cicd` is a command-line program for showing how to deploy cross-compiled releases on GitHub using Travis CI.

[![Build Status](https://travis-ci.com/jonstites/hello-cicd.svg?branch=master)](https://travis-ci.com/jonstites/hello-cicd)

## Motivation

CI/CD is like flossing: everyone knows they should do it, but it's not very fun.

Anyway, it turns out that publishing cross-compiled binaries on GitHub for Rust projects is easy, but not super-well documented for some reason.

## Building the Travis CI configuration file

Let start with a minimal `.travis.yml` that only runs our test:

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

Commit, tag and push (we'll do this every time, incrementing the version as we go):
```
git push origin master
git tag -a "v0.1.0"
git push tag "v0.1.0"
```

You can see that it ran [successfully](https://travis-ci.com/jonstites/hello-cicd/jobs/178401495/config) but that the
[release](https://github.com/jonstites/hello-cicd/releases/tag/v0.1.0) has no binaries.

Let's fix that. We'll get an [OAuth token](https://docs.travis-ci.com/user/deployment/releases/#authenticating-with-an-oauth-token) and try again with v0.1.1:

```yaml
language: rust
rust: stable

script:
  - cargo build 
  - cargo test 

before_deploy:
  - cargo build --release

deploy:
  api_key:
    # Elided for space
    secure: "tBj..."

  file: target/release/hello-cicd

  on:
    tags: true

  provider: releases
  skip_cleanup: true
  
branches:
  only:
    # Pushes and PR to the master branch
    - master
    
notifications:
  email:
    on_success: never
```

Build [passes](https://travis-ci.com/jonstites/hello-cicd/jobs/178401495/config) but still
[no binaries](https://github.com/jonstites/hello-cicd/releases/tag/v0.1.1)! What gives?

The reason is that we actually need to trigger builds on our tags.
Assuming we're using [SemVer](https://semver.org/) for our tags, this can be done with a regex for our v0.1.2:

```yaml
language: rust
rust: stable

script:
  - cargo build 
  - cargo test 

before_deploy:
  - cargo build --release

deploy:
  api_key:
    # Elided for space
    secure: "tBj..."

  file: target/release/hello-cicd

  on:
    tags: true

  provider: releases
  skip_cleanup: true
  
branches:
  only:
    # Pushes and PR to the master branch
    - master
    # Ruby regex to match tags.
    # Required, or travis won't trigger deploys when a new tag is pushed.
    - /^v\d+\.\d+(\.\d+)?(-\S*)?$/
    
notifications:
  email:
    on_success: never
```

Great! Tests [pass](https://travis-ci.com/jonstites/hello-cicd/builds/101237811) and
a binary is [uploaded](https://github.com/jonstites/hello-cicd/releases/tag/v0.1.2).

But just one binary. And it's built for whatever Travis CI is giving us by default
(currently Ubuntu Trusty 14.04).

Let's add Linux and macOS for v0.1.3:

```yaml
rust: stable
language: rust

script:
  - cargo build 
  - cargo test 

matrix:
  include:
    - os: linux
      env: TARGET=i686-unknown-linux-musl
    - os: linux
      env: TARGET=x86_64-unknown-linux-musl
    - os: osx
      env: TARGET=x86_64-apple-darwin

before_deploy:
  - cargo build --release --target $TARGET
  - cp target/${TARGET}/release/hello-cicd hello-cicd-${TRAVIS_TAG}-${TARGET}
      
deploy:
  api_key:
    secure: "tBj..."

  file: hello-cicd-${TRAVIS_TAG}-${TARGET}

  on:
    tags: true

  provider: releases
  skip_cleanup: true
  
branches:
  only:
    # Pushes and PR to the master branch
    - master
    # Ruby regex to match tags.
    # Required, or travis won't trigger deploys when a new tag is pushed.
    - /^v\d+\.\d+(\.\d+)?(-\S*)?$/
    
notifications:
  email:
    on_success: never
```


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
