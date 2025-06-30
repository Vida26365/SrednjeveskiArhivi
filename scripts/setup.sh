#!/usr/bin/env bash

## SET UP OLLAMA ##

echo "[SETUP] Setting up Ollama"

rm -rf ollama
mkdir -p ollama

curl -L https://github.com/ollama/ollama/releases/download/v0.9.3/ollama-linux-amd64.tgz -o ollama.tgz
tar -C ollama -xzf ollama.tgz
rm ollama.tgz

## SET UP PYTHON ##

echo "[SETUP] Setting up Python"

module load Python

rm -rf venv

python3 -m venv venv
source venv/bin/activate

pip3 install --upgrade pip
pip3 install -r requirements.txt
