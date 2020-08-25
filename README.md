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

In this example, HLL is about 10x faster than `sort -u`; for even larger inputs, the time difference is likely to be even greater.

## Error Rate
You can reign-in the error by specifying the error rate to the utility: `-e {error_rate}`. For example:

```bash
$ time ./hll input.txt  -e 0.005
32358

real    0m0.107s
user    0m0.047s
sys     0m0.063s
```

The trade-off for more accurate results is a larger memory footprint to store the probabilistic data structure used by HLL. You also cannot decrease the error rate indefinitely - you will reach a point where the math breaks down and thus the underlying implementation's assertions will fail.
