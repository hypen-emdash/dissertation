#!/bin/sh
./target/release/score ./target/release/lahc-solve ./weddings/ring-suite
./target/release/score ./target/release/lahc-solve ./weddings/rand-suite
./target/release/score ./target/release/lahc-solve ./weddings/complete-suite
./target/release/score ./target/release/lahc-solve ./weddings/tense-suite
