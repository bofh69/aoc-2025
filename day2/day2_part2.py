#!/usr/bin/env python3

"""Solution to Advent of Code 2025 Day 2 Part 2"""

import sys


def count_invalids(start, end):
    """Count numbers in the range that are invalid."""

    invalid_sum = 0
    for number in range(start, end + 1):
        decimals = len(str(number))
        for n_digits in range(1, decimals // 2 + 1):
            if decimals % n_digits != 0:
                continue
            base = 10**n_digits
            bv = number % base
            if bv == 0:
                continue
            v = bv
            i = n_digits
            while i < decimals:
                v = (v * base) + bv
                i += n_digits
            if v == number:
                invalid_sum += number
                break
    return invalid_sum


def main():
    """Comment to make pylint shut up."""
    with open(sys.argv[1], "r", encoding="ascii") as file:
        lines = file.readlines()
        line = lines[0]
        ranges = line.split(",")
        invalid_sum = 0
        for r in ranges:
            bounds = r.split("-")
            start = int(bounds[0])
            end = int(bounds[1])
            invalid_sum += count_invalids(start, end)
        print(invalid_sum)


main()
