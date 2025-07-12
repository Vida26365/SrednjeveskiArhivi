OPENAI_BASE_URL = "http://localhost:8000/v1"
OPENAI_API_KEY = "SOME-API-KEY"

MODEL_OCR = "Qwen/Qwen2.5-VL-72B-Instruct-AWQ"
MODEL_SEGMENTATION = "Qwen/Qwen2.5-VL-72B-Instruct-AWQ"

PARAMS_OCR = {"temperature": 0, "top_k": 1, "top_p": 0.1, "min_p": 1.0, "max_tokens": 1024}
PARAMS_SEGMENTATION = {"temperature": 0, "top_k": 1, "top_p": 0.1, "min_p": 1.0, "max_tokens": 2048}

DIRECTORY_SOURCE = "source"
DIRECTORY_OUTPUT_RAW = "output/raw"
DIRECTORY_OUTPUT_SEGMENTED = "output/segmented"
