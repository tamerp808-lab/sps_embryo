#!/bin/bash
cargo build --release
mkdir -p deb/usr/local/bin
cp target/release/sps_embryo deb/usr/local/bin/sps
dpkg-deb --build deb sps-embryo.deb
