#!/usr/bin/env bash

## SET UP VLLM ##

echo "[SETUP] Setting up vLLM"

apptainer pull /tmp/vllm.sif docker://vllm/vllm-openai
rm /tmp/vllm.sif

## SET UP PYTHON ##

echo "[SETUP] Setting up Python"

module load Python

rm -rf venv

python3 -m venv venv
source venv/bin/activate

pip3 install --upgrade pip
pip3 install -r requirements.txt
