#!/usr/bin/env bash

#SBATCH --job-name=srednjeveski-arhivi
#SBATCH --partition=gpu
#SBATCH --gpus=1
#SBATCH --mem=200G
#SBATCH --output=logs/stdout.log
#SBATCH --error=logs/stderr.log
#SBATCH --time=00:30:00

echo "[HELPER] Starting job $SLURM_JOB_ID on $(hostname)"

# Load necessary modules
module load Python

# Get the model configuration
MODEL_OCR=$(./venv/bin/python3 -c "from config.config import MODEL_OCR; print(MODEL_OCR)")
MODEL_SEGMENTATION=$(./venv/bin/python3 -c "from config.config import MODEL_SEGMENTATION; print(MODEL_SEGMENTATION)")

# Unset incorrect GPU environment variable to prevent issues with Ollama
# See: https://github.com/ollama/ollama/issues/11220
unset HIP_VISIBLE_DEVICES
unset ROCR_VISIBLE_DEVICES

# Configure the Ollama variables
export OLLAMA_MODELS="$WORKDIR/models"
export OLLAMA_FLASH_ATTENTION=1
export OLLAMA_KV_CACHE_TYPE=f16
export OLLAMA_KEEP_ALIVE=1h
export OLLAMA_DEBUG=2

# Start the Ollama server in the background
echo "[HELPER] Starting Ollama"
./ollama/bin/ollama serve > logs/ollama.log 2>&1 &
trap 'kill $(jobs -p) 2>/dev/null' EXIT
sleep 5

# Pull the models from Ollama registry
echo "[HELPER] Pulling models"
./ollama/bin/ollama pull "$MODEL_OCR"
./ollama/bin/ollama pull "$MODEL_SEGMENTATION"
sleep 5

# Run the processing script
echo "[HELPER] Running the processing script"
srun ./venv/bin/python3 main.py > logs/script.log 2>&1

echo "[HELPER] Job completed successfully"
