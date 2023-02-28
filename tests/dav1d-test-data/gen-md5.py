#!/usr/bin/env python3

# Copyright © 2018, VideoLAN and dav1d authors
# All rights reserved.
#
# Redistribution and use in source and binary forms, with or without
# modification, are permitted provided that the following conditions are met:
#
# 1. Redistributions of source code must retain the above copyright notice, this
#    list of conditions and the following disclaimer.
#
# 2. Redistributions in binary form must reproduce the above copyright notice,
#    this list of conditions and the following disclaimer in the documentation
#    and/or other materials provided with the distribution.
#
# THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
# ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
# WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
# DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
# ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
# (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
# LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
# ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
# (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
# SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

import os, argparse, subprocess
from pathlib import Path

parser = argparse.ArgumentParser()

# Input file
parser.add_argument("files", metavar='FILE', nargs='+', help='input file')

# Test line output file
parser.add_argument("--mesonfile",
    type=argparse.FileType('a', encoding='UTF-8'),
    default='meson.build',
    help='meson build file to which to append test calls')

# Test program
parser.add_argument("--aomdec",
    default='aomdec',
    help='full path to aomdec')

# Output bit depth
parser.add_argument("--bitdepth",
    type=int,
    default='8',
    help='output bit depth')

args = parser.parse_args()

for filepath in args.files:
    # Call aomdec to obtain the md5
    res = subprocess.run([
        args.aomdec,
        "--output-bit-depth={}".format(args.bitdepth),
        "--md5", "--rawvideo", "--skip-film-grain",
        filepath
        ],
        stdout=subprocess.PIPE, check=True)

    md5, _ = res.stdout.decode('UTF-8').split(' ', 1)

    if args.mesonfile:
        test_name = Path(filepath).stem
        args.mesonfile.write(
                (
                "\t['{name}', files('{filepath}'), '{md5}'],\n"
                ).format(name=test_name, filepath=filepath, md5=md5)
            )

if args.mesonfile:
    args.mesonfile.close()
