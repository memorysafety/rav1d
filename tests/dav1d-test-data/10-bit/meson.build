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

tests = []

subdir('argon')
subdir('data')
subdir('features')
subdir('issues')
subdir('quantizer')

foreach test : tests
    test(test[0], dav1d, suite: 'testdata-10',
         args: dav1d_test_args + ['-i', test[1], '--verify', test[2]])
    if '@0@'.format(test[1][0]).endswith('.ivf')
        test(test[0], seek_stress, suite: 'testdata_seek-stress', timeout: 60,
             args: dav1d_test_args + ['-i', test[1], '--muxer', 'null'])
    endif
endforeach

# test with film grain applied against dav1d's md5
test('av1-1-b10-23-film_grain-50', dav1d, suite: 'testdata-10',
    args: dav1d_test_args + ['-i', files('film_grain/av1-1-b10-23-film_grain-50.ivf'),
           '--filmgrain',  '1',
           '--verify', 'be596f5921854b9a9a5be81c302a5327'])
