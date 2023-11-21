# Benchmarks

Some benchmarks with different files and different length options

## hello.txt

Single file containing 100 lines of text "Hello world!"

Initial file size: 1398 bytes

SHA256: 01de38027934e36fa03eb31003d581fc1311bff1b626b3d736bf33b4ef7df15e

| Length | Size (B) | Compression time (s) | Decompression time (s) |
|--------|----------|----------------------|------------------------|
| 9      | 216      | 0.010                | 0.004                  |
| 10     | 240      | 0.004                | 0.004                  |
| 11     | 264      | 0.004                | 0.040                  |
| 12     | 288      | 0.007                | 0.004                  |
| 13     | 312      | 0.004                | 0.003                  |
| 14     | 336      | 0.003                | 0.003                  |
| 15     | 360      | 0.003                | 0.004                  |
| 16     | 383      | 0.004                | 0.006                  |

## lorem.txt

100 paragraphs of lorem ipsum text generated with [loremipsum.io](https://loremipsum.io)

Initial file size: 76 798 bytes

SHA256: 170521b36c325d840068eb525c90aebe88532efc15d7ad6e69549bca909ceb58

| Length | Size (B) | Compression time (s) | Decompression time (s) |
|--------|----------|----------------------|------------------------|
| 9      | 55 542   | 0.022                | 0.022                  |
| 10     | 47 023   | 0.022                | 0.020                  |
| 11     | 40 758   | 0.016                | 0.015                  |
| 12     | 34 189   | 0.015                | 0.013                  |
| 13     | 29 624   | 0.013                | 0.011                  |
| 14     | 25 135   | 0.013                | 0.010                  |
| 15     | 26 930   | 0.013                | 0.010                  |
| 16     | 28 725   | 0.013                | 0.009                  |

## Hank-2003.pdf

A book "Introduction to Information Theory and Data Compression"

Initial size: 4 876 425 bytes

SHA256: 817c2f0d2fa989a1bc566261b3ebd6edc2f11357021b09d06498bc218daeab4b

| Length | Size (B)  | Compression time (s) | Decompression time (s) |
|--------|-----------|----------------------|------------------------|
| 9      | 4 657 343 | 1.407                | 1.383                  |
| 10     | 4 951 005 | 1.323                | 1.197                  |
| 11     | 5 284 626 | 1.297                | 1.131                  |
| 12     | 5 607 271 | 1.291                | 1.120                  |
| 13     | 5 832 107 | 1.240                | 1.070                  |
| 14     | 5 915 815 | 1.273                | 1.044                  |
| 15     | 5 821 389 | 1.337                | 1.087                  |
| 16     | 5 534 003 | 1.419                | 1.030                  |
