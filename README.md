# xenv

A cross-shell env sourcing command.

I use [fish](https://fishshell.com) a lot, but I also like to do

```bash
source .env
```

when I want to source specific environment variables for a shell session.

As far as I know `fish` doesn't have this capability, and I figured I would write something that could work across different shells, just in case I switch to `zsh` or something else.

## installation and usage

For now to install it, you need `cargo` installed. Once installed run the following command:

```fish
cargo install xenv
```

```fish
# this will look for a .xenv file in your current working directory
xenv echo hello this is my command

# you can also override this with the XENV_PATH environment variable

env XPATH_ENV=xenv.env xenv env | rg foo

# for bash/zsh/...
# XPATH_ENV=xenv.env xenv env | rg foo
```

## .xenv file format

It'll follow the format for any `.env` file, but for reference, here's the [docker-compose .env rules](https://docs.docker.com/compose/env-file/#syntax-rules)

```bash
# here is an example
FOO=bar

# it's case insensitive
foo=bar

# empty keys will be skipped
=qux

# empty values are allowed
BANG=
```
