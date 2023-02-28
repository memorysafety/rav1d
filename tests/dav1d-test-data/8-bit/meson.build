# Copyright © 2018-2020, VideoLAN and dav1d authors
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

subdir('data')
subdir('features')
subdir('issues')
subdir('quantizer')
subdir('size')
subdir('intra')
subdir('resize')
subdir('cdfupdate')
subdir('mv')
subdir('mfmv')

foreach test : tests
    test(test[0], dav1d, suite: 'testdata-8',
         args: dav1d_test_args + ['-i', test[1], '--verify', test[2]])
    if '@0@'.format(test[1][0]).endswith('.ivf')
        test(test[0], seek_stress, suite: 'testdata_seek-stress', timeout: 60,
            args: dav1d_test_args + ['-i', test[1], '--muxer', 'null'])
    endif
endforeach

subdir('vq_suite')

subdir('svc')

# test with film grain applied against dav1d's md5
test('av1-1-b8-23-film_grain-50', dav1d, suite: 'testdata-8',
    args: dav1d_test_args + ['-i', files('film_grain/av1-1-b8-23-film_grain-50.ivf'),
           '--filmgrain',  '1',
           '--verify', '392a4adc567fa05b210eebe15bcbb491'])
test('ccvb_film_grain-fg', dav1d, suite: 'testdata-8',
    args: dav1d_test_args + ['-i', files('features/ccvb_film_grain.ivf'),
           '--filmgrain',  '1',
           '--verify', 'a934b6263b7009746cce5f5bd33224f1'])
test('309_odd_width', dav1d, suite: 'testdata-8',
    args: dav1d_test_args + ['-i', files('issues/309_odd_width.ivf'),
           '--filmgrain',  '1',
           '--verify', '30d31f7c74575e58366898534a87841d'])

subdir('sframe')
