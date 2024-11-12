# Http Redirect

Redirect http requests to their https counterpart.

## How to use

### Install

Bash the following commands:

```sh
git clone https://github.com/herebythere/http_redirect
cargo install --path http_redirect
```

### Run 

Bash the following command to redirect requests `127.0.0.1:80`:

```sh
http_redirect 127.0.0.1:80
```

### In action

Any request is sent a redirect response.

For example:
- `super-cool.com` is redirected to `https://super-cool.com`
- `http://bummer-drag.com` is redirected to `https://bummer-drag`

`Http redirect` cannot forward `https` requests.

## Licence

BSD 3-Clause License
