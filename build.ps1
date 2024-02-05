param (
    [string]$BuildOption = "--speed"
)

switch ($BuildOption) {
    "--speed" {
        & cargo +nightly build -Z unstable-options -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-pc-windows-msvc --profile=release-speed
    }
    "--min" {
        & cargo +nightly build -Z unstable-options -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-pc-windows-msvc --profile=release-min-size
    }
    default {
        Write-Host "Invalid argument. Please use --speed or --min."
    }
}
