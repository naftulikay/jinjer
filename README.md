# jinjer [![Build Status][travis.svg]][travis]

A CLI tool for rendering Jinja-esque templates using the [Tera template engine][tera].

## Usage

```
USAGE:
    jinjer [FLAGS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v               Logging verbosity. By default, it is set to ERROR, pass -v multiple times to increase verbosity.

SUBCOMMANDS:
    facts     Dump all detected facts to standard output. Useful for understanding which facts are available at
              runtime.
    help      Prints this message or the help of the given subcommand(s)
    render    Render one or more templates to standard output or a file.
```

`jinjer` exposes two subcommands at present:

 - `facts`: dumps available system facts to standard output in JSON format.
 - `render`: renders either standard input or a list of template files in order, either to standard output or to a
   specified output file.

#### `jinjer facts`

```
USAGE:
    jinjer facts

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```

Executing `jinjer facts` will simply dump a JSON dictionary of all facts discovered by `jinjer` at runtime. Use this
command to preview which facts will be available during template rendering.

#### `jinjer render`

```
USAGE:
    jinjer render [FLAGS] [OPTIONS] [template_files]...

FLAGS:
    -e, --auto-escape    Enable HTML auto-escaping of templates in the template renderer. By default, output is not safe
                         for HTML.
    -h, --help           Prints help information
    -V, --version        Prints version information

OPTIONS:
    -o, --output <output_file>    Render output to a file rather than to standard output.

ARGS:
    <template_files>...    A list of template files to render in order.
```

Use `jinjer render` to render templates. If no template files are passed, `jinjer` will attempt to read a template from
standard input. By default, rendering is done to standard output, use `-o|--output` to send rendering output to a
file on the filesystem.

Template output is unsanitized by default. If rendering HTML-safe output is required, pass `-e|--auto-escape` to
attempt to render output safely for HTML.

## Facts

`jinjer` has a fact plugin system for facts at runtime. Presently, there are two plugins:

 - `"basic"`: basic system facts such as CPU count.
 - `"env"`: environment variables.

Ultimately, it would be easy and straightforward to add support for systems like `facter` and perhaps Ansible to
leverage these as fact providers. Contributions welcome. :wave:

## License

 - [Apache Software License, Version 2.0](./LICENSE-APACHE)
 - [MIT License](./LICENSE-MIT)

 [tera]: https://tera.netlify.com/
 [travis]: https://travis-ci.org/naftulikay/jinjer
 [travis.svg]: https://travis-ci.org/naftulikay/jinjer.svg?branch=master
