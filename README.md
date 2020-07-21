# reprow-rs
Row repeater by as many times as the value in specific column from CSV-formatted files

## Usage

```
USAGE:
    reprow-rs [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --col <STRING>
            Sets column which specifies times to repeat rows.
            Each row in the column must be non-negative integer.
        --dst <PATH>
            Sets destination file path. If not set, destination sets to stdout. (default: stdout)

        --log <off|error|warn|info|debug|trace>    Sets log level. (default: off)
        --src <PATH>
            Sets source file path. If not set, source sets to stdin. (default: stdin)
```

## Example

in.csv

```in.csv
letter,count
a,3
b,0
c,4
d,1
```

```
$ reprow-rs --src in.csv --col count --dst out.csv 
```

out.csv
```out.csv
letter,count
a,3
a,3
a,3
c,4
c,4
c,4
c,4
d,1
```
