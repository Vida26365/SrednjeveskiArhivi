#!/usr/bin/env bash

rm -rf ollama
mkdir -p ollama

curl -L https://ollama.com/download/ollama-linux-amd64.tgz -o ollama.tgz
tar -C ollama -xzf ollama.tgz
rm ollama.tgz
