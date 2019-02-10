"""
benchmark.py
author: Ian Brault <ian.brault@engineering.ucla.edu>
"""

import collections
import fileinput
import sys

def main():
    bm_dict = collections.defaultdict(float)
    for line in fileinput.input():
        name, time_str = line.split(',')
        bm_dict[name] += float(time_str)

    total = bm_dict["total"]
    max_name_len = len(max(bm_dict.keys(), key=len)) + 2
    for func, time in bm_dict.items():
        print("{: <{w}}{: >10.4f}s{: >10.3f}%".format(func, time, (time/total)*100, w=max_name_len))

if __name__ == "__main__":
    sys.exit(main())
