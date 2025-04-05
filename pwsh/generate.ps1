param(
    [Parameter()]
    [String]$size = "200"
)

cargo build --release --quiet

$sw = [Diagnostics.Stopwatch]::StartNew()

cargo run --release --quiet -- $size

$sw.Stop()
$sw.Elapsed
