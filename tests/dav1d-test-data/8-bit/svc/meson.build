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

test('av1-1-b8-22-svc-L1T2', dav1d, suite: 'testdata-8',
    args: dav1d_test_args + ['-i', files('av1-1-b8-22-svc-L1T2.ivf'),
           '--verify', '3987b4852874d3fba06999ee9b4d117c'])
# test --oppoint 0
test('av1-1-b8-22-svc-L2T1', dav1d, suite: 'testdata-8',
    args: dav1d_test_args + ['-i', files('av1-1-b8-22-svc-L2T1.ivf'),
           '--alllayers', '0', '--oppoint', '0',
           '--verify', 'f8305c4b796b620388fee614df3ebac6'])
# test --oppoint 1
test('av1-1-b8-22-svc-L2T1', dav1d, suite: 'testdata-8',
    args: dav1d_test_args + ['-i', files('av1-1-b8-22-svc-L2T1.ivf'),
           '--alllayers', '0', '--oppoint', '1',
           '--verify', '1e595312e1cebf280a4dc65e84f07544'])
# test --alllayers 1
test('av1-1-b8-22-svc-L2T1', dav1d, suite: 'testdata-8',
    args: dav1d_test_args + ['-i', files('av1-1-b8-22-svc-L2T1.ivf'),
           '--alllayers', '1', '--oppoint', '0',
           '--verify', '47df59c53e601d4addb9ac9c5fd6431e'])
# test --oppoint 0
test('av1-1-b8-22-svc-L2T2', dav1d, suite: 'testdata-8',
    args: dav1d_test_args + ['-i', files('av1-1-b8-22-svc-L2T2.ivf'),
           '--alllayers', '0', '--oppoint', '0',
           '--verify', '4d86bc8f55bae0ac0e688fb7fa2fe741'])
# test --oppoint 1
test('av1-1-b8-22-svc-L2T2', dav1d, suite: 'testdata-8',
    args: dav1d_test_args + ['-i', files('av1-1-b8-22-svc-L2T2.ivf'),
           '--alllayers', '0', '--oppoint', '1',
           '--verify', '30f5b24f8aa7a8be78a9366ddb2b18d3'])
# test --oppoint 2
test('av1-1-b8-22-svc-L2T2', dav1d, suite: 'testdata-8',
    args: dav1d_test_args + ['-i', files('av1-1-b8-22-svc-L2T2.ivf'),
           '--alllayers', '0', '--oppoint', '2',
           '--verify', '8443d3cacdcee6b7ee3d699013845525'])
# test --oppoint 3
test('av1-1-b8-22-svc-L2T2', dav1d, suite: 'testdata-8',
    args: dav1d_test_args + ['-i', files('av1-1-b8-22-svc-L2T2.ivf'),
           '--alllayers', '0', '--oppoint', '3',
           '--verify', 'c1b34078caaef0689157151890f5a656'])
# test --alllayers 1
test('av1-1-b8-22-svc-L2T2', dav1d, suite: 'testdata-8',
    args: dav1d_test_args + ['-i', files('av1-1-b8-22-svc-L2T2.ivf'),
           '--alllayers', '1', '--oppoint', '0',
           '--verify', '19a96386a3a061f64d52fea9f3906258'])
