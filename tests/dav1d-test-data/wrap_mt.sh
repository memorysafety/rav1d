#!/bin/sh

# if the tests uses the dav1d CLI tool append tile / frame thread options
if [ -z ${1%%*/dav1d} ]; then
    $@ --threads ${THREADS:=2} --framedelay ${FRAMEDELAY:=2}
else
    $@
fi
