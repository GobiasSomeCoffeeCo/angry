# Angry 😠
An attempt at a Content Discovery Tool. :beer:

**Also know as the tool that inspired** [AngryOxide](https://github.com/Ragnt/AngryOxide)... 😠

![](angry.gif)

# About
This is a tool that is heavily inspired by [Feroxbuster](https://github.com/epi052/feroxbuster). It is meant to be a project that will help in better understanding Rust's :crab: features by building something similar (but way worse) from the ground up. It is a work in progress and there are plenty of things I need to clean up and optimize. If you're looking for something functional and coded by someone way smarter than me, I'd check out Feroxbuster as it's full featured and heavily optimzed. 

Note: Codes a little messy/unstructured right now, as I try to determine the best way ahead.

# Usage
```
A Content Discovery Tool written in Rust

Usage: angry [OPTIONS] --url <https://www.<target>.com> [COMMAND]

Commands:
  test
          does testing things
  help
          Print this message or the help of the given subcommand(s)

Options:
  -u, --url <https://www.<target>.com>
          Target URL

  -w, --wordlist <FILE>
          Path to the wordlist

          [default: directories.txt]

  -p, --proxy <PROXY>
          Proxy to use for requests (ex: http(s)://host:port, socks5(h)://host:port)

  -t, --threads <NUMBER>
          Number of threads

          [default: 50]

  -d, --debug...
          Turn debugging information on

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

Client Settings:
  -T, --timeout <SECONDS>
          Number of seconds before a client's request times out

          [default: 7]

  -r, --redirects
          Allow a client to follow redirects

  -H, --headers <HEADERS>
          Allow a client to specify HTTP headers

  -f, --fuzz <https://FUZZ.<target>.com>
          Enter fuzzing mode. Pass a value to FUZZ within the the URL: ("https://FUZZ.<target_url>.com" or "https://<target_url>/script.php?valid_name=FUZZ")

  -a, --user-agent <USER_AGENT>
          Allow a client to specify a User-Agent

          [default: "Mozilla/5.0 (Macintosh; Intel Mac OS X x.y; rv:42.0) Gecko/20100101 Firefox/42.0)"]

  -i, --insecure
          Disables TLS certificate validation in the client

Response filters:
  -s, --status-codes <STATUS_CODE>...
          Status Codes to include (allow list) (default: 200 204 301 302 307 308 401 403 405)

  -e, --exclude-status-codes <STATUS_CODE>...
          Status Codes to exclude (returns all status codes except the ones passed)
```

# TODO
- [ ] Add recursive scanning
- [ ] Add max depth
- [ ] Add extensions
- [x] Add headers
- [x] Add fuzzing
