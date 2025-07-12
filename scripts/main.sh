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

# Unset incorrect GPU environment variable to prevent issues with GPU detection
# See: https://github.com/ollama/ollama/issues/11220
unset HIP_VISIBLE_DEVICES
unset ROCR_VISIBLE_DEVICES

# Note: Add support for different models in the future

# Start the vLLM server in the background
echo "[HELPER] Starting vLLM server"
apptainer run --nv docker://vllm/vllm-openai --model "$MODEL_OCR" --max-model-len 50000 >logs/vllm.log 2>&1 &
trap 'kill $(jobs -p) 2>/dev/null' EXIT

# Wait for the vLLM server to start
echo "[HELPER] Waiting for vLLM server"
until curl --output /dev/null --silent --fail http://localhost:8000/ping; do
  printf '.'
  sleep 5
done
echo ""
echo "[HELPER] vLLM server is ready"

# Run the processing script
echo "[HELPER] Running the processing script"
srun ./venv/bin/python3 main.py >logs/script.log 2>&1

echo "[HELPER] Job completed successfully"
