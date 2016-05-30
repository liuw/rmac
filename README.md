Random mac address generator written in Rust.

Distributed under MIT license.

Before you consider using this program, I have something better for you:

$ openssl rand -hex 6 | sed 's/\*\(..\)/\1:/g; s/.$//'
