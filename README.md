## thaumiel
[![Build Status](https://travis-ci.org/Nu-SCPTheme/thaumiel.svg?branch=master)](https://travis-ci.org/Nu-SCPTheme/thaumiel)

A Wikidot-like web server to provide pages, forums, and other wiki services using backends such as [DEEPWELL](https://github.com/Nu-SCPTheme/deepwell).

Available under the terms of the GNU Affero General Public License. See [LICENSE.md](LICENSE).

### Compilation
This library targets the latest stable Rust. At time of writing, that is 1.40.0

```sh
$ sudo apt install libpq-dev
$ cargo build --release
```

Before running the binary, create a cookie key by using 512 bytes of random data:

```sh
$ head -c 512 /dev/urandom > thaumiel-cookie.key
```

This will create the final `thaumiel` binary, which can be executed using the following:

```sh
$ cargo run -- [arguments]
```
