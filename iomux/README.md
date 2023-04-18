The `iomux` binary multiplexes the stdout, stderr, and other info about
a set of child process into stdout.

The design is unix-like, aiming to provide output that's convenient for
unix tools (`grep`, `sed`, etc…).

# Example - Aggregate stdout/stderr in a parseable format

Sometimes it's useful to aggregate stdout/stderr and then later parse them out.

```text
$ iomux find /etc/ | tee find.log
12971> spawned "find" "/etc/"
12971  /etc/
…
12971  /etc/ssl/private
12971! find: ‘/etc/ssl/private’: Permission denied
…
12981  /etc/passwd
12981  /etc/timezone
…
12981> exit 1
```

The output above is all on stdout including both the stdout and stderr of
`find` as well as metadata about the `find` process, such as its arguments
and exit status.

Each line begins with the PID of `find`, then a "tag" indicating the
source of information for that line:

- `>` metadata about the `find` process.
- ` ` stdout
- `!` stderr

## Parsing `find.log`

We can parse the log using standard unix tools to answer various queries
We can reproduce the stderr stream with standard unix text manipulation:

### Determining the PID of `find`

```text
$ grep '^[0-9]*>' find.log | head -1 | sed 's/>.*$//'
12981
```

### Reproducing stdout

```text
$ grep '^[0-9]* ' ./find.log | sed 's/^[0-9]*  //'
/etc/
…
/etc/ssl/private
…
/etc/passwd
/etc/timezone
…
```

### Reproducing stderr

```text
$ grep '^[0-9]*!' ./find.log | sed 's/^[0-9]*! //'
…
find: ‘/etc/ssl/private’: Permission denied
```

### Determining the exit status

```text
$ grep '^[0-9]*> exit' ./find.log | sed 's/^.*exit //'
1
```

# Example - Log environmental details of a build

Suppose you are automating a build process which uses `make` and want
to log the approximate launch timestamp, environment, pwd, etc… Using
`iomux` you could aggregate this info like so:

```text
$ iomux -- date --iso=s -- pwd -- env -- make | tee build.log
```

The output in `build.log` interleaves the info from those child
processes in a line-oriented format that makes unix-style text processing
convenient.

# Example - Merge logs

I like to see both my system journal and my desktop log in one place:

```text
$ iomux -- journalctl -f -- tail -F ~/.xsession-errors
```
