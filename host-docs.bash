#!/usr/bin/env bash

function main {
    cargo d --no-deps -p peacock -p peacock-crest -p peacock-pinion
    nix run nixpkgs#simple-http-server -- target/doc --port 8001
}

main
