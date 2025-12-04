#!/usr/bin/env python3

"""AoC 2025 - day 3"""

import sys


def find_max(bank):
    """Find maximum digit and its pos"""
    max_digit = -1
    digit_pos = -1
    for c, i in zip(bank, range(len(bank))):
        c = int(c)
        if c > max_digit:
            max_digit = c
            digit_pos = i
    return (max_digit, digit_pos)


def main():
    """Shut up, pylint!"""
    with open(sys.argv[1], "r", encoding="ascii") as f:
        data = f.readlines()
        joltage_sum = 0
        for line in data:
            start = 0
            joltage = 0
            for n in range(12):
                (digit, digit_pos) = find_max(line[start : -(12 - n)])
                joltage = joltage * 10 + digit
                start += digit_pos + 1
            joltage_sum += joltage

        print(joltage_sum)


if __name__ == "__main__":
    main()
