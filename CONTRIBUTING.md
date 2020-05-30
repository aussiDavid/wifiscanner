# Contributing to wifiscanner

## How to contribute

If you'd like to contribute, start by searching through the [issues](https://github.com/github/opensource.guide/issues) and [pull requests](https://github.com/github/opensource.guide/pulls) to see whether someone else has raised a similar idea or question.

If you don't see your idea listed, and you think it fits into the goals of this guide, do one of the following:

* **If your contribution is minor,** such as a typo fix, open a pull request.
* **If your contribution is major,** such as a new feature or bug fix, start by opening an issue first. That way, other people can weigh in on the discussion before you do any work.

If you want mentoring on an issue please ask, we'll assign someone to work with you.

## Submitting your contribution

- Work on the feature or bug (with tests if possible)
- Run `make checks` if there's no problems raise the PR and reference the issue that initated the work carried out
- Once the PR passes checks and review we'll merge the PR

## Setting up your environment

- Install Rust and friends using [rustup](https://rustup.rs) 
- If you don't have a preferred editor, [VS Code](https://code.visualstudio.com) with the [RLS](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) extension are an awesome combo

### Using Docker

Install Docker and Docker Compose.

To run the tests and watch for changes:

```sh
$ docker-compose run --rm app

[Running 'cargo test']
  Downloaded itertools v0.8.2
  Downloaded either v1.5.3
    Finished test [unoptimized + debuginfo] target(s) in 2.18s
     Running target/debug/deps/wifiscanner-c5e51ced9dea0c24

running 4 tests
test sys::linux::tests::should_parse_iw_dev ... ok
test sys::linux::tests::when_there_is_a_security ... ok
test sys::linux::tests::when_it_parses_the_first_device ... ok
test sys::linux::tests::when_there_is_no_security ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests wifiscanner

running 2 tests
test src/lib.rs -  (line 25) ... ok
test src/lib.rs -  (line 31) ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

[Finished running. Exit status: 0]
```

To open a shell inside the container:

```sh
$ docker-compose run --rm app bash

I have no name!@578c7faa0bf7:/usr/src/app
```

To manually build the image:

```sh
$ docker-compose build

Building app
Step 1/7 : FROM rust
 ---> 84ba1f55dee3
Step 2/7 : RUN rustup component add rustfmt clippy --toolchain 1.43.1-x86_64-unknown-linux-gnu
 ---> Using cache
 ---> 6369442688c6
Step 3/7 : USER 1000:1000
 ---> Using cache
 ---> 8832d988e5d8
Step 4/7 : WORKDIR /usr/src/app
 ---> Using cache
 ---> ae5bc1343dcc
Step 5/7 : COPY . .
 ---> Using cache
 ---> cab58cab3dd7
Step 6/7 : RUN cargo install cargo-watch
 ---> Using cache
 ---> f58dbc61a6e7
Step 7/7 : CMD ["make test-watch"]
 ---> Using cache
 ---> a7327164755d
Successfully built a7327164755d
Successfully tagged wifiscannner:latest
```

