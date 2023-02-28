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

# skip fuzz tests when fuzzing target are not built
if not is_variable('dav1d_fuzzer_sources')
    subdir_done()
endif

NAME = 'clusterfuzz-testcase-minimized-dav1d_fuzzer'

asan_tests = [
    '4554338031239168',
    '5634614945644544',
    '5634740825096192',
    '5642297870385152',
    '5642709818146816',
    '5643152501768192',
    '5646679710105600',
    '5648093022781440',
    '5649420100567040',
    '5660318848516096',
    '5661648644210688',
    '5663492208590848',
    '5665734852608000',
    '5667445105229824',
    '5668166089310208',
    '5671715242246144',
    '5673101040287744',
    '5674065629544448',
    '5674936551604224',
    '5678810226229248',
    '5679083342528512',
    '5679944668020736',
    '5681086273683456',
    '5692903256227840',
    '5693196152078336',
    '5697181166600192',
    '5705886085939200',
    '5707479116152832',
    '5707663195766784',
    '5708205552828416',
    '5708722911838208',
    '5710678279585792',
    '5712199973076992',
    '5712491695308800',
    '5713998356414464',
    '5719070108811264',
    '5721265675960320',
    '5727148069879808',
    '5727844055908352',
    '5731418676658176',
    '5731769337249792',
    '5733641993322496',
    '5736270563639296',
    '5739751889436672',
    '5740590025670656',
    '5740611047522304',
    '5740963536830464',
    '5742383057076224',
    '5744823991009280',
    '5744917725315072',
    '5757842691194880',
    '5758202998685696',
    '5758322939002880',
    '5760991657197568',
    '5762640203218944',
    '5762861192708096',
    '5765823210192896',
]
foreach tc : asan_tests
    test(tc, dav1d_fuzzer, suite: 'oss-fuzz-asan',
         args: [files('asan/' + NAME + '-' + tc)])
endforeach

msan_tests = [
    '5098868442660864',
    '5634939618328576',
    '5645355501223936',
    '5647965146841088',
    '5657755306688512',
    '5658693757042688',
    '5662161355931648',
    '5670100066107392',
    '5678406532857856',
    '5678778181746688',
    '5681207354851328',
    '5682084585144320',
    '5684823666982912',
    '5692962210578432',
    '5697034242228224',
    '5704953876709376',
    '5710557533962240',
    '5724911197224960',
    '5734861545930752',
    '5737125446680576',
    '5737476191158272',
    '5751594251976704',
    '5769427090407424',
]
foreach tc : msan_tests
    test(tc, dav1d_fuzzer, suite: 'oss-fuzz-msan',
         args: [files('msan/' + NAME + '-' + tc)])
endforeach

ubsan_tests = [
    '5205936726081536',
    '5637959701757952',
    '5640043364876288',
    '5640864307609600',
    '5649066722066432',
    '5651110220529664',
    '5652082695077888',
    '5669943870226432',
    '5670328320131072',
    '5685383740784640',
    '5691087507685376',
    '5691270664552448',
    '5693769798647808',
    '5702505208479744',
    '5717082881130496',
    '5719316180238336',
    '5724731362246656',
    '5733716937146368',
    '5737466644922368',
    '5746175664193536',
    '5766054584778752',
]
foreach tc : ubsan_tests
    test(tc, dav1d_fuzzer, suite: 'oss-fuzz-ubsan',
         args: [files('ubsan/' + NAME + '-' + tc)])
endforeach

asan_mt_tests = [
    '5089954858795008',
    '5387199655313408',
    '5550258780766208',
    '5640098964570112',
    '5646860283281408',
    '5652139771166720',
    '5655593017147392',
    '5674491150073856',
    '5700224002031616',
    '5702455078158336',
    '5728508249112576',
    '5734069753610240',
    '5743306491822080',
]
foreach tc : asan_mt_tests
    test(tc, dav1d_fuzzer_mt, suite: 'oss-fuzz-asan',
         args: [files('asan/' + NAME + '_mt-' + tc)])
endforeach

msan_mt_tests = [
    '5636143299690496',
    '5652400153559040',
    '5662659052044288',
    '5674585370918912',
    '5710047858917376',
    '5716498572640256',
    '5719098537803776',
    '5726708213219328',
    '5741861168218112',
    '5746740678885376',
    '5753369222709248',
    '5757117496033280',
    '5767052292259840',
]
foreach tc : msan_mt_tests
    test(tc, dav1d_fuzzer_mt, suite: 'oss-fuzz-msan',
         args: [files('msan/' + NAME + '_mt-' + tc)])
endforeach

ubsan_mt_tests = [
    '5636065151418368',
    '5647147911872512',
    '5648677054447616',
    '5649526686220288',
    '5684725352497152',
    '5684846249115648',
    '5687624891629568',
    '5697786113163264',
    '5700248035393536',
    '5076736684851200',
    '5720731439071232',
    '5732654503165952',
    '5735640277188608',
    '5736621216890880',
    '5748471626203136',
    '6586873632391168',
]
foreach tc : ubsan_mt_tests
    test(tc, dav1d_fuzzer_mt, suite: 'oss-fuzz-ubsan',
         args: [files('ubsan/' + NAME + '_mt-' + tc)])
endforeach

test('issue_200', dav1d_fuzzer, suite: 'oss-fuzz-msan',
     args: [files('msan/200_poc-716bd48336e172bb613b14102a34853aaa4f5cbd93175d1265ee087fdf7c5930')])

test('issue_210', dav1d_fuzzer, suite: 'oss-fuzz-asan',
     args: [files('asan/210_poc-fd54f42a4b812d4159f88c0e165a0acf1a3b5604a4926fdd7445c29db207e6ea__1_')])

test('issue_220', dav1d_fuzzer, suite: 'oss-fuzz-ubsan',
     args: [files('ubsan/220_fuzz-input-72836')])

test('issue_266', dav1d_fuzzer_mt, suite: 'oss-fuzz-asan',
     args: [files('asan/fuzz-input-4f968')])

test('5634807321591808', dav1d_fuzzer, suite: 'oss-fuzz-ubsan',
     args: files('ubsan/clusterfuzz-testcase-dav1d_fuzzer-5634807321591808'))

test('5761827623927808', dav1d_fuzzer, suite: 'oss-fuzz-ubsan',
     args: files('ubsan/clusterfuzz-testcase-dav1d_fuzzer-5761827623927808'))
