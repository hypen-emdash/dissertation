#!/bin/sh
./target/release/score ./target/release/hill-solve ./weddings/ring-suite
./target/release/score ./target/release/hill-solve ./weddings/rand-suite
./target/release/score ./target/release/hill-solve ./weddings/complete-suite
./target/release/score ./target/release/hill-solve ./weddings/tense-suite
