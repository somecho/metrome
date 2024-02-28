Get-ChildItem examples/valid | ForEach-Object { cargo run -- -p $_.FullName }
