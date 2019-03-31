# Python script to clean up `bindgen`-generated OMPT bindings.
# Copyright (c) 2018, Philip Conrad. All rights reserved.
import sys
import re


# Inspiration: https://stackoverflow.com/a/7060438
def titlecase(match):
    return "".join(word.title() for word in match.group(0).split("_"))


if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Error: Need refactorings TSV file.")
        exit(1)

    # Load up refactorings lists.
    lines = open(sys.argv[1], 'r').readlines()
    lines = [line.strip() for line in lines]      # Strip out newline chars.
    pairs = [line.split('\t') for line in lines]  # Split by tab delimiter.
    # Sort by length, so longest matches come first.
    pairs.sort(key=(lambda x: len(x[0])), reverse=True)

    # Cleanup code
    for line in sys.stdin.readlines():
        out = line
        # Apply all regexes in sequence:
        for pair in pairs:
            old, new = pair
            out = re.sub(old, new, out)

        # Clean up ::std namespace madness.
        if '::std' in line:
            out = re.sub(r'::std::os::raw::', '', out)
            out = re.sub(r'::std::option::', '', out)
        print(out, end='')  # Lines already have '\n' on them.
