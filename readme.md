# A faster str::from_utf8

Via a Unicode DFA.

```text
test from_utf8_2_bytes_fast      ... bench:         304 ns/iter (+/- 65) = 1315 MB/s
test from_utf8_2_bytes_regular   ... bench:         346 ns/iter (+/- 17) = 1156 MB/s

test from_utf8_3_bytes_fast      ... bench:         847 ns/iter (+/- 17) = 1416 MB/s
test from_utf8_3_bytes_regular   ... bench:       1,148 ns/iter (+/- 106) = 1045 MB/s

test from_utf8_4_bytes_fast      ... bench:       1,554 ns/iter (+/- 105) = 1544 MB/s
test from_utf8_4_bytes_regular   ... bench:       2,072 ns/iter (+/- 41) = 1158 MB/s

test from_utf8_all_bytes_fast    ... bench:       2,731 ns/iter (+/- 135) = 1464 MB/s
test from_utf8_all_bytes_regular ... bench:       3,982 ns/iter (+/- 371) = 1004 MB/s

test from_utf8_ascii_fast        ... bench:       1,253 ns/iter (+/- 106) = 2035 MB/s
test from_utf8_ascii_regular     ... bench:       1,935 ns/iter (+/- 16) = 1318 MB/s

test from_utf8_cyr_fast          ... bench:       5,755 ns/iter (+/- 326) = 891 MB/s
test from_utf8_cyr_regular       ... bench:       6,433 ns/iter (+/- 510) = 797 MB/s

test from_utf8_enwik8_fast       ... bench:   5,112,086 ns/iter (+/- 157,224) = 1956 MB/s
test from_utf8_enwik8_regular    ... bench:   4,781,329 ns/iter (+/- 139,057) = 2091 MB/s

test from_utf8_jawik10_fast      ... bench:  10,222,942 ns/iter (+/- 438,919) = 978 MB/s
test from_utf8_jawik10_regular   ... bench:  11,677,255 ns/iter (+/- 510,663) = 856 MB/s

test from_utf8_mixed_fast        ... bench:       3,264 ns/iter (+/- 94) = 1480 MB/s
test from_utf8_mixed_regular     ... bench:       3,564 ns/iter (+/- 168) = 1355 MB/s

test from_utf8_mostlyasc_fast    ... bench:       1,858 ns/iter (+/- 43) = 1965 MB/s
test from_utf8_mostlyasc_regular ... bench:       2,287 ns/iter (+/- 222) = 1596 MB/s
```