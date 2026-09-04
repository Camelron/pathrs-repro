# pathrs MSHV kernel-version reproducer

This minimal program exercises the `pathrs 0.2.4` operation sequence used by
Kata Agent's `CopyFile` implementation:

1. Open a confined directory with `Root::open`.
2. Create a nested parent with `Root::mkdir_all`.
3. Reopen the returned `O_PATH` handle as a directory.

The reopen path constructs a safe procfs handle. `pathrs` then checks whether
the guest kernel has an unbroken mount API and lazily parses the kernel release.

## Run

```bash
RUST_BACKTRACE=1 cargo run
```

An explicit scratch directory can be supplied if needed:

```bash
RUST_BACKTRACE=1 cargo run -- /tmp/pathrs-repro-root
```

On a conventional kernel release such as `5.15.0-190-generic`, all operations
should succeed.

On an MSHV kernel release such as `6.6.137.mshv2-2.azl3`, unpatched
`pathrs 0.2.4` extracts the numeric prefix `6.6.137.` and rejects its empty
final component. The expected result is a panic containing:

```text
uname kernel release must be a valid KernelVersion string
```

The process should exit with status 101. The host directory may remain after a
panic and can be removed with:

```bash
rm -rf /tmp/pathrs-repro-*
```