import base64
import json
import logging
import os
import re
import sys
import time
from io import BytesIO

import pdfplumber
from openai import OpenAI
from openai.types.chat.chat_completion_content_part_image_param import ChatCompletionContentPartImageParam
from openai.types.chat.chat_completion_content_part_image_param import ImageURL
from openai.types.chat.chat_completion_system_message_param import ChatCompletionSystemMessageParam
from openai.types.chat.chat_completion_user_message_param import ChatCompletionUserMessageParam

from config.config import (
    DIRECTORY_OUTPUT_RAW,
    DIRECTORY_OUTPUT_SEGMENTED,
    DIRECTORY_SOURCE,
    MODEL_OCR,
    MODEL_SEGMENTATION,
    OPENAI_API_KEY,
    OPENAI_BASE_URL,
    PARAMS_OCR,
    PARAMS_SEGMENTATION,
)
from config.models import Document
from config.prompts import PROMPT_OCR, PROMPT_SEGMENTATION


CLIENT = OpenAI(base_url=OPENAI_BASE_URL, api_key=OPENAI_API_KEY)


def extract_pages(path):
    images = []
    with pdfplumber.open(path) as pdf:
        for page in pdf.pages:
            image = page.to_image(resolution=300, antialias=True).original
            images.append(image)
    return images


def encode_image(image):
    buffered = BytesIO()
    image.save(buffered, format="jpeg")
    b64 = base64.b64encode(buffered.getvalue()).decode("utf-8")
    return f"data:image/jpeg;base64,{b64}"


def escape_linebreaks(text):
    def replacer(match):
        return match.group(0).replace("\n", "\\n").replace("\r", "\\r")

    return re.sub(r"\"(.*?)(?<!\\)\"", replacer, text, flags=re.DOTALL)


def ocr_image(image):
    messages = [
        ChatCompletionUserMessageParam(
            role="user",
            content=[
                ChatCompletionContentPartImageParam(
                    type="image_url",
                    image_url=ImageURL(url=encode_image(image)),
                )
            ],
        ),
        ChatCompletionSystemMessageParam(
            role="system",
            content=PROMPT_OCR,
        ),
    ]

    completion = CLIENT.chat.completions.create(
        model=MODEL_OCR,
        messages=messages,
        extra_body=PARAMS_OCR,
    )

    if completion.choices[0].finish_reason != "stop":
        raise ValueError(f"Invalid finish reason ({completion.choices[0].finish_reason}), skipping document")

    return completion.choices[0].message.content


def segment_text(text):
    messages = [
        ChatCompletionSystemMessageParam(role="system", content=PROMPT_SEGMENTATION),
        ChatCompletionUserMessageParam(role="user", content=text),
    ]

    completion = CLIENT.beta.chat.completions.parse(
        model=MODEL_SEGMENTATION,
        messages=messages,
        response_format=Document,
        extra_body=PARAMS_SEGMENTATION,
    )

    if completion.choices[0].finish_reason != "stop":
        raise ValueError(f"Invalid finish reason ({completion.choices[0].finish_reason}), skipping document")

    message = completion.choices[0].message

    if message.parsed:
        return message.parsed.model_dump_json(indent=2) + "\n"

    logging.error("  No parsed content found, parsing manually")

    # Ensure Unix line endings
    content = message.content.replace("\r\n", "\n").replace("\\r\\n", "\\n")

    # Remove code block wrappers if present
    if content.strip().startswith("```"):
        content = "\n".join(content.split("\n")[1:])
    if content.strip().endswith("```"):
        content = "\n".join(content.split("\n")[:-1])

    # Ensure trailing newline
    if not content.endswith("\n"):
        content += "\n"

    # Escape unescaped line breaks inside strings
    content = escape_linebreaks(content)

    # Validate and format JSON
    try:
        obj = json.loads(content)
        return json.dumps(obj, indent=2, ensure_ascii=False) + "\n"
    except ValueError:
        logging.error("  Invalid JSON, using raw text")
        return content


def process_document(filename):
    # Skip non-PDF files
    if not filename.lower().endswith(".pdf"):
        logging.info("  Skipping non-PDF document")
        return

    source = os.path.join(DIRECTORY_SOURCE, filename)
    output_raw = os.path.join(DIRECTORY_OUTPUT_RAW, filename.rsplit(".", 1)[0] + ".txt")
    output_segmented = os.path.join(DIRECTORY_OUTPUT_SEGMENTED, filename.rsplit(".", 1)[0] + ".json")

    # Skip already processed files
    if os.path.exists(output_raw) and os.path.exists(output_segmented):
        logging.info("  Skipping already processed document")
        return

    start = time.time()

    # Extract pages from the file
    logging.info("  Extracting pages from document")
    images = extract_pages(source)

    # Extract text from each page
    texts = []
    for idy, image in enumerate(images):
        logging.info(f"  Extracting text from page {idy + 1}/{len(images)}")
        text = ocr_image(image)
        texts.append(text)

    # Combine the extracted text
    combined = "\n".join(texts)

    # Save the raw text
    logging.info("  Saving raw content")
    with open(output_raw, "w", encoding="utf-8") as file:
        file.write(combined)

    # Segment the raw text
    logging.info("  Segmenting raw content")
    segmented = segment_text(combined)

    # Save the segmented text
    logging.info("  Saving segmented content")
    with open(output_segmented, "w", encoding="utf-8") as file:
        file.write(segmented)

    logging.info(f"  Took {time.time() - start:.2f}s")


def main():
    logging.basicConfig(
        stream=sys.stdout,
        level=logging.INFO,
        format="[%(name)s] [%(asctime)s] [%(levelname)s] %(message)s",
        datefmt="%Y-%m-%d %H:%M:%S",
    )

    logging.getLogger('openai').setLevel(logging.INFO)
    logging.getLogger('httpx').setLevel(logging.WARNING)

    os.makedirs(DIRECTORY_SOURCE, exist_ok=True)
    os.makedirs(DIRECTORY_OUTPUT_RAW, exist_ok=True)
    os.makedirs(DIRECTORY_OUTPUT_SEGMENTED, exist_ok=True)

    documents = os.listdir(DIRECTORY_SOURCE)

    for idx, filename in enumerate(documents):
        logging.info(f'Processing document "{filename}" ({idx + 1}/{len(documents)})')

        try:
            process_document(filename)
        except Exception as error:
            logging.exception(f"  {error}")


if __name__ == "__main__":
    main()
