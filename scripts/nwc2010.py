#!/usr/bin/env python3

import os
import lzma
import gzip
from glob import glob
from argparse import ArgumentParser
from natsort import natsorted

# The following command will output `1gms.sorted.gz` sorted by counts
# ```
# $ python3 scripts/nwc2010.py nwc2010-ngrams/word/over999/1gms -s
# ```
#
# The following command will output `2gms.gz` unsorted by counts
# ```
# $ python3 scripts/nwc2010.py nwc2010-ngrams/word/over999/2gms
# ```


def convert(dirname, is_sort=False):
    records = []
    dirname = dirname.rstrip('/')

    print('Loading...')
    for filename in natsorted(glob(f'{dirname}/*.xz')):
        print(filename)
        for gram in lzma.open(filename, 'rt'):
            gram = gram.rstrip()
            tokens, count = gram.split('\t')
            records.append((tokens, int(count)))

    if is_sort:
        print('Sorting...')
        records.sort(key=lambda x: x[1], reverse=True)

    outname = f'{dirname}/{os.path.basename(dirname)}' + ('.sorted.gz' if is_sort else '.gz')

    print(f'Writing {outname}...')
    with gzip.open(outname, 'wt') as fout:
        fout.write(f'{len(records)}\n')
        for rec in records:
            fout.write(f'{rec[0]}\t{rec[1]}\n')


def main():
    parser = ArgumentParser()
    parser.add_argument('input_dir')
    parser.add_argument('-s', '--sort', nargs='?', const=True, default=False)
    args = parser.parse_args()

    convert(args.input_dir, is_sort=args.sort)


if __name__ == "__main__":
    main()
