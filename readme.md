# A faster str::from_utf8

Using modulo arithmetic to calculate pointer alignment

```text
test from_utf8_2_bytes_fast      ... bench:         310 ns/iter (+/- 42) = 1290 MB/s
test from_utf8_2_bytes_regular   ... bench:         309 ns/iter (+/- 24) = 1294 MB/s

test from_utf8_3_bytes_fast      ... bench:       1,027 ns/iter (+/- 62) = 1168 MB/s
test from_utf8_3_bytes_regular   ... bench:       1,513 ns/iter (+/- 611) = 793 MB/s

test from_utf8_4_bytes_fast      ... bench:       1,788 ns/iter (+/- 26) = 1342 MB/s
test from_utf8_4_bytes_regular   ... bench:       1,907 ns/iter (+/- 181) = 1258 MB/s

test from_utf8_all_bytes_fast    ... bench:       3,463 ns/iter (+/- 97) = 1155 MB/s
test from_utf8_all_bytes_regular ... bench:       4,083 ns/iter (+/- 89) = 979 MB/s

test from_utf8_ascii_fast        ... bench:          88 ns/iter (+/- 4) = 28988 MB/s
test from_utf8_ascii_regular     ... bench:          88 ns/iter (+/- 8) = 28988 MB/s

test from_utf8_cyr_fast          ... bench:       7,707 ns/iter (+/- 531) = 665 MB/s
test from_utf8_cyr_regular       ... bench:       8,202 ns/iter (+/- 135) = 625 MB/s

test from_utf8_enwik8_fast       ... bench:   1,135,756 ns/iter (+/- 84,450) = 8804 MB/s
test from_utf8_enwik8_regular    ... bench:   1,145,468 ns/iter (+/- 79,601) = 8730 MB/s

test from_utf8_jawik10_fast      ... bench:  12,723,844 ns/iter (+/- 473,247) = 785 MB/s
test from_utf8_jawik10_regular   ... bench:  13,384,596 ns/iter (+/- 666,997) = 747 MB/s

test from_utf8_mixed_fast        ... bench:       2,321 ns/iter (+/- 123) = 2081 MB/s
test from_utf8_mixed_regular     ... bench:       2,702 ns/iter (+/- 408) = 1788 MB/s

test from_utf8_mostlyasc_fast    ... bench:         249 ns/iter (+/- 10) = 14666 MB/s
test from_utf8_mostlyasc_regular ... bench:         276 ns/iter (+/- 5) = 13231 MB/s
```