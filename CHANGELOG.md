# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/)
and this project adheres to [Semantic Versioning](https://semver.org/).

## [0.1.3] - 2026-11-19

### CI

- pass the automation App secrets to the release (#44)

### Chores

- cargo update

### Features

- add title_case, and roll memchr back in Cargo.lock (#49)

## [0.1.2] - 2026-11-18

### Chores

- cargo update

### Documentation

- describe the Rust gate as lines and regions, with the release build (#31)

### Features

- add letter_count, and roll itoa back in Cargo.lock (#35)

## [0.1.1] - 2026-11-17

### Build

- raise the rust-version floor to 1.98 (#17)

### CI

- move the nightly ops cron to the 01:xx UTC hour (#16)

### Chores

- sync the managed files with the released templates (#14)
- sync the managed files with the released templates (#26)

### Documentation

- list the doctests, the formatting check and the README step for a new function (#19)

### Features

- add longest_word, and roll memchr back in Cargo.lock (#25)

## [0.1.0] - 2026-11-14

### CI

- run CodeQL on Rust so PRs can merge (#4)

### Chores

- step 3 - vergil.toml
- step 4 - config files
- step 6 - CI/CD workflows
- step 7 - docs site

### Features

- add reverse_words, which reverses a text's word order (#8)
