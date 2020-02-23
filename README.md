# `shasum` for Windows

A very basic implementation of `shasum` for Windows (less than 30 lines of Rust code).

It doesn't hash directories themselves, and doesn't support any of shasum's fancier features (e.g. it only does sha1 hashing).

The output of `shasum-windows.exe foo` is equivalent to `find foo -type f -exec shasum {} +` (i.e. calling shasum on all files in the directory), except for:

* Directory junctions and symlinks are not traversed.

`shasum-windows` will also replace all backslashes in its output with forward slashes to make it easier to verify that the output is identical to that of `shasum`.

## Installation & usage

1. Install Rust from http://rustup.rs/
2. Run `cargo install --git https://github.com/caspark/shasum-windows.git`
3. Run `shasum-windows directory-of-your-choice`

## Example usage

```
c:\src\shasum-windows>shasum-windows.exe c:\temp\test\
22dedae30d54d528cdf8e81151ad26e86cd19bbb  c:/temp/test/counts-2018-10.xml
5cb92d6ba8270bd649db00ebb12873055be345f8  c:/temp/test/counts-2019-01.xml
a0d3eb93719724c76b3d43740a103eac52887dc1  c:/temp/test/subdir/counts-2018-11.xml
1ac2e5d72e99d7e237a9af2a7117a44ed1262643  c:/temp/test/subdir/counts-2018-12.xml
```

## See also

* https://github.com/idrassi/DirHash - a fancier implementation, but crashes when it encounters a directory junction.
