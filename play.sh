#!/bin/bash
cd carmen/src
cargo build
cd ..
cd target/debug
./run.sh
