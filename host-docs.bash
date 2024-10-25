#!/usr/bin/env bash

function main {
    cargo d -j 4
    nix run nixpkgs#simple-http-server target/doc
}

main
