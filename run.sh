#!/bin/bash
set -eux

export PATH=$PATH:$PWD/../rage/target/debug
(cd ../rage && cargo build)
cargo build

export PATH=$PATH:$PWD/target/debug
age-plugin-trezor > trezor.id
R=`grep "recipient:" trezor.id | cut -f 3 -d " "`

date | rage -r $R -a | rage -d -i trezor.id
