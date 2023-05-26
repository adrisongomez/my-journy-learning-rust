# Mini-GREP

It's just learning purpose project of building simple terminal utilities similar to `grep`.

## PoC

```Bash
cargo run -- searchString <file_name> <flags>?
```

## Arguments

```
-i To set ignore case or use env var MINIGREP_IGNORE_CASE
```

## Commands
```
cargo run -- to ./example_file.txt -i


cargo run -- to ./example_file.txt -i > output.txt
```