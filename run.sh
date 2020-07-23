#!/bin/bash

if cargo build; then
    [[ ! -a wtrack_base.so ]] && ln -s target/debug/libwtrack_base.so wtrack_base.so
    python3 ./run.py
fi

exit

