# hello-cicd
`hello-cicd` is a command-line program for showing how to deploy cross-compiled releases on GitHub using Travis CI.

[![Build Status](https://travis-ci.com/jonstites/hello-cicd.svg?branch=master)](https://travis-ci.com/jonstites/hello-cicd)

## Motivation

CI/CD is like flossing: everyone knows they should do it, but it's not very fun.

Anyway, it turns out that publishing cross-compiled binaries on GitHub for Rust projects is easy, but not super-well documented for some reason.

## Lessons

Rust can be cross-compiled, but it's nowhere near as easy as it is in Go.

The tools [cross](https://github.com/japaric/rust-cross) and [trust](https://github.com/japaric/trust) can be used together
for cross-compilation on Travis CI and AppVeyor. Cross is awesome - targets include FreeBSD, Dragonfly, iOS, and Android.
Trust makes it easier to release the binaries using Travis CI and AppVeyor.

The downside is that it adds a lot of complexity. I chose not to use Cross/Trust because I wanted a simpler system that I
understood completely and could maintain indefinitely. I only wanted Linux/macOS/Windows support anyway, so I didn't
really lose any functionality.

Rust builds on Windows on Travis CI are agonizingly slow. This is because of some interaction between `rust-docs` and antivirus
software. This is a known issue. Hopefully this is [fixed](https://github.com/rust-lang/rustup.rs/issues/1540) from the Rust side eventually.

A lot of guides use AppVeyor for Windows builds, but very recently, Travis CI started supporting Windows builds too (it's in early release now).
This makes it simpler and easier than learning and maintaining two different ci/cd systems.

## Building the Travis CI configuration file

### v0.1.0

Let start with a minimal configuration file that only runs our test. After all, we do want Travis CI to
make sure our tests all pass on the `master` branch whenever it changes.

`.travis.yml`
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

The `master` branch tests were [successful](https://travis-ci.com/jonstites/hello-cicd/jobs/178401495/config).
There were no tests for the tag and no [release](https://github.com/jonstites/hello-cicd/releases/tag/v0.1.0) with binaries.

Let's fix that.

### v0.1.1

We need to given Travis CI write-access to our repository and tell it which files to deploy.

We'll get an [OAuth token](https://docs.travis-ci.com/user/deployment/releases/#authenticating-with-an-oauth-token) for access.

And we will use the `before_deploy` and `deploy` steps.

`.travis.yml`
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

Tests for the `master` branch [pass](https://travis-ci.com/jonstites/hello-cicd/jobs/178401495/config).
There is still no test for our tag, specifically, though, and [no binaries](https://github.com/jonstites/hello-cicd/releases/tag/v0.1.1).

The reason is that we need to trigger builds on our tags. We have told Travis CI that we only
want a build to be triggered against the `master` branch. Tags, even based off the `master` branch,
won't trigger a new build.

### v0.1.2

Assuming we're using [SemVer](https://semver.org/) for our tags, we can just use a regex to match tags only (not branches).

`travis.yml`
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

Great! Tests for the tag [pass](https://travis-ci.com/jonstites/hello-cicd/builds/101237811) and
a binary is [uploaded](https://github.com/jonstites/hello-cicd/releases/tag/v0.1.2).

But just one binary. And it's built for whatever Travis CI is giving us by default
(currently Ubuntu Trusty 14.04).

### v0.1.3

Let's add 32-bit and 64-bit Linux binaries, and also macOS for v0.1.3 of `hello-cicd`.

`.travis.yml`
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

install:
  - rustup target add $TARGET

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

Tests [pass](https://travis-ci.com/jonstites/hello-cicd/builds/101238539) and
binaries get [deployed](https://github.com/jonstites/hello-cicd/releases/tag/v0.1.3).

That change was easy.

### v0.1.4

Finally, let's add Windows support, too. Be warned that because of an interaction between
rust-docs and antivirus software, Windows Rust builds are super slow on Travis CI (like 10 minutes).

`.travis.yml`
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
    - os: windows
      env: TARGET=x86_64-pc-windows-msvc
    - os: windows
      env: TARGET=i686-pc-windows-msvc

install:
  - rustup target add $TARGET
      
before_deploy:
  - |
    (
    cargo build --release --target $TARGET    
    if [ "$TRAVIS_OS_NAME" = 'windows' ]; then
      cp target/${TARGET}/release/hello-cicd.exe hello-cicd-${TRAVIS_TAG}-${TARGET}.exe
    else
      cp target/${TARGET}/release/hello-cicd hello-cicd-${TRAVIS_TAG}-${TARGET}
    fi
    )
    
deploy:
  api_key:
    secure: "tBj..."

  file: hello-cicd-${TRAVIS_TAG}-${TARGET}*
  file_glob: true

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

`.cargo/config`
```toml
[target.x86_64-pc-windows-msvc]
rustflags = ["-Ctarget-feature=+crt-static"]
[target.i686-pc-windows-msvc]
rustflags = ["-Ctarget-feature=+crt-static"]
```

Tests [pass](https://travis-ci.com/jonstites/hello-cicd/builds/101383535)
and binaries are [added](https://github.com/jonstites/hello-cicd/releases/tag/v0.1.4).

We have added Windows support now!

Also, the magic in `.cargo/config` enables static builds for Windows binaries. Our
Windows users won't need to install MSVC (Microsoft Visual C++ (MSVC) Redistributable for Visual Studio),
or anything else. They can run our executable and it will just work.

### Closing thoughts

If we wanted to get fancy, we could create an archive that included a README and License, along with the
binary. We would probably want to use `tar.gz` for Linux and macOS and `.zip` for Windows.

If we added `cache: cargo` to our `.travis.yml`, it would speed up our builds.

We could use the `strip` command to reduce the size of our binaries.

Finally, we could run tests against beta and nightly Rust, in addition to stable.

Implementing those ideas will be left as an exercise for the reader.

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
