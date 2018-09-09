# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## v0.2.0 - 2018-09-09

* Removed `#[panic_handler]` as it is no longer needed in latest nightly.
* Removed unnecessary old attribute to access intrinsics


## v0.1.3 - 2018-09-07

Now the panic handler is `#[inline(never)]` to have a symbol to place
breakpoints on, thanks to @japaric for bringing it to attention.

## v0.1.2 - 2018-08-29

* Implementation caused code bloat of about 1kB, now fixed

## v0.1.1 - 2018-08-28

* Changed to the new `panic_handler`
* Updated to include `README.md` in `Cargo.toml`

## v0.1.0 - 2018-08-10

Initial release
