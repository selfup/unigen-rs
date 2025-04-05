param(
    [Parameter()]
    [String]$size = "30"
)

cargo build --quiet

cargo run --quiet $size
