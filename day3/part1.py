#!/usr/bin/env python3

"""AoC 2025 - day 3"""

import sys


def main():
    """Shut up, pylint!"""
    with open(sys.argv[1], "r", encoding="ascii") as f:
        data = f.readlines()
        joltage_sum = 0
        for line in data:
            line = line.strip()
            max_first = -1
            max_pos = -1
            for c, i in zip(line, range(len(line) - 1)):
                c = int(c)
                if c > max_first:
                    max_first = c
                    max_pos = i
            max_second = -1
            for c in line[max_pos + 1 :]:
                c = int(c)
                max_second = max(max_second, c)
            joltage_sum += max_first * 10 + max_second

        print(joltage_sum)


if __name__ == "__main__":
    main()
