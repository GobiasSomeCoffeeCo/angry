# Angry 😠
An attempt at a Content Discovery Tool. :beer:

![](angry.gif)

# About
This is a tool that is heavily inspired by [Feroxbuster](https://github.com/epi052/feroxbuster). It is meant to be a project that will help me better understand Rust's :crab: features by building something similar from the ground up. It is a work in progress and there are plenty of things I need to clean up and optimize. If you're looking for something functional and coded by someone way smarter than me, I'd check out Feroxbuster as it's full featured and heavily optimzed. 

Note: Codes a little messy/unstructured right now, as I try to determine the best way ahead.

# Usage
```
A Content Discovery Tool written in Rust

Usage: angry [OPTIONS] --url <https://www.<target>.com> [COMMAND]

Commands:
  test
          does testing things  ##Dont worry about this guy##
  help
          Print this message or the help of the given subcommand(s)

Options:
  -u, --url <https://www.<target>.com>
          Target URL

  -w, --wordlist <FILE>
          Path to the wordlist

          [default: directories.txt]

  -t, --threads <NUMBER> **Work in Progress**
          Number of threads

          [default: 50]

  -s, --status-codes <STATUS_CODE>...
          Status Codes to include (allow list) (default: 200 204 301 302 307 308 401 403 405)

  -e, --exclude-status-codes <STATUS_CODE>...
          Status Codes to exclude aka inverse of --status-codes (returns all status codes except the ones passed)

  -d, --debug...
          Turn debugging information on ##Work in Progress##

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

# TODO
- [ ] Add recursive scanning
- [ ] Add max depth
- [ ] Add extensions
- [ ] Add headers
