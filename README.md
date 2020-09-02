# hll-util
Console utility to do a count distinct approximation using HyperLogLog (HLL). This util is analogous to calling `sort -u input.txt | wc -l`, but it requires very little memory because it doesn't need to store all unique values. The accuracy of results depend on the specified error rate.

## Examples
First, consider a reasonably large input of different values. We will generate one million values between 0 and 9999 for testing:

```bash
$ for i in `seq 1000000`; do echo $(($RANDOM % 10000)); done > input.txt
$ ls -lh input.txt
-rwxrwxrwx 1 adam adam 5.4M Aug 18 08:48 input.txt
$ wc -l input.txt
1000000 input.txt
$ head input.txt
17879
20300
19373
216
9347
14066
31476
24643
31082
30117
```

Using GNU `sort -u`, and `wc -l`, we can find how many distinct values are found in the input:

```bash
$ time sort -u input.txt | wc -l
32768

real    0m1.025s
user    0m2.813s
sys     0m0.266s
```

Using `hll`, we can do an _estimation_ of the counts:
```bash
$ time ./hll input.txt
30113

real    0m0.119s
user    0m0.094s
sys     0m0.016s
```

Because the HLL implementation uses randomly-generated numbers for hashing of input, this value can change between invocations:
```bash
$ ./hll input.txt
35679
$ ./hll input.txt
34991
$ ./hll input.txt
35662
```

In this example, HLL is about 10x faster than `sort -u`.

## Larger-scale tests
To test larger inputs, we generate two files of 100M records each, one with values up to 10k and one up to 100k:

```bash
$ for i in `seq 100000000`; do echo $(($RANDOM % 10000)); done > input2.txt
$ for i in `seq 100000000`; do echo $(($RANDOM % 100000)); done > input3.txt
```

We can then compare `sort -u | wc -l` to `hll`:

```bash
$ time sort -u input2.txt | wc -l
10001

real    1m0.775s
user    5m4.813s
sys     0m17.344s
$ time ./hll -e 0.01 input2.txt
10423

real    0m6.895s
user    0m4.578s
sys     0m2.313s
```

```bash
$ time sort -u input3.txt | wc -l
100001

real    1m8.427s
user    5m53.000s
sys     0m15.359s
$ time ./hll -e 0.01 input3.txt
103021

real    0m7.254s
user    0m5.063s
sys     0m2.188s
```

These tests show a consistent 10x wall time speedup. It's also interesting to note the CPU time for the GNU utils is almost 60x as much as `hll`.


## Error rate
You can reign-in the error by specifying the error rate to the utility: `-e {error_rate}`. For example:

```bash
$ time ./hll input.txt  -e 0.005
32358

real    0m0.107s
user    0m0.047s
sys     0m0.063s
```

The trade-off for more accurate results is a larger memory footprint to store the probabilistic data structure used by HLL. You also cannot decrease the error rate indefinitely - you will reach a point where the math breaks down and thus the underlying implementation's assertions will fail.

## CSV-formatted input
`hll` can also parse delimited (eg, comma-separated, pipe-separated) files and count column independently. First, generating a one million record, three-column CSV file:

```bash
for i in `seq 1000000`; do echo $(($RANDOM % 10000)),$(($RANDOM % 10000)),$(($RANDOM % 10000)); done > input.csv
```

We can use GNU utils or `hll` to count or estimate the number of distinct rows, as before:

```bash
$ time sort -u input.csv | wc -l
1000000

real    0m1.428s
user    0m3.406s
sys     0m0.234s

$ time ./target/release/hll input.csv
982319

real    0m0.173s
user    0m0.094s
sys     0m0.047s
```

But we can also use the `--format csv` argument to have `hll` look at columns individually:

```bash
$ time ./target/release/hll input.csv --format csv
7097,10076,9913

real    0m0.250s
user    0m0.203s
sys     0m0.047s
```

If the input has a header, provide the `-h` flag to not count the header row's values and to output the header row with its counts:

```bash
$ head -5 input.csv
Foo,Bar,Baz
517,255,5392
1864,4873,4816
8643,119,7987
3030,3432,1536

$ ./target/release/hll --format csv input.csv -h
Foo,Bar,Baz
9302,10221,9754
```

