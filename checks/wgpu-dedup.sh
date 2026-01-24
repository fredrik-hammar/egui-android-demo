#!/bin/sh

# Check that wgpu dependency is deduplicated so that necessary
# features can be enabled as describen in README.md.

case "$1" in
    "-v" | "--verbose")
	out=/dev/stdout
	err=/dev/stderr
	;;
    *)
	out=/dev/null
	err=/dev/null
	;;
esac

check() {
    cargo tree --target=aarch64-linux-android \
	       --edges features \
	       --invert wgpu \
	       --offline \
	> $out 2> $err
}

if check; then
    echo "OK: wgpu is deduplicated"
else
    echo "FAIL: wgpu is duplicated"
    exit 1
fi
