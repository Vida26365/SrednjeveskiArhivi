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
    DIRECTORY_OUTPUT,
    DIRECTORY_SOURCE,
    MODEL_OCR,
    MODEL_SEGMENTATION,
    OPENAI_API_KEY,
    OPENAI_BASE_URL,
    TEMPERATURE_OCR,
    TEMPERATURE_SEGMENTATION,
)
from config.models import Document
from config.prompts import PROMPT_OCR, PROMPT_SEGMENTATION


CLIENT = OpenAI(base_url=OPENAI_BASE_URL, api_key=OPENAI_API_KEY)


def extract_pages(path):
    images = []
    with pdfplumber.open(path) as pdf:
        for page in pdf.pages:
            image = page.to_image(resolution=150, antialias=True).original
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
        temperature=TEMPERATURE_OCR,
        messages=messages,
    )

    return completion.choices[0].message.content


def segment_text(text):
    messages = [
        ChatCompletionSystemMessageParam(role="system", content=PROMPT_SEGMENTATION),
        ChatCompletionUserMessageParam(role="user", content=text),
    ]

    completion = CLIENT.beta.chat.completions.parse(
        model=MODEL_SEGMENTATION,
        temperature=TEMPERATURE_SEGMENTATION,
        messages=messages,
        response_format=Document,
    )

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


def main():
    logging.basicConfig(
        stream=sys.stdout,
        level=logging.INFO,
        format="[%(name)s] [%(asctime)s] [%(levelname)s] %(message)s",
        datefmt="%Y-%m-%d %H:%M:%S",
    )

    os.makedirs(DIRECTORY_SOURCE, exist_ok=True)
    os.makedirs(DIRECTORY_OUTPUT, exist_ok=True)

    documents = os.listdir(DIRECTORY_SOURCE)

    for idx, filename in enumerate(documents):
        logging.info(f'Processing document "{filename}" ({idx + 1}/{len(documents)})')

        # Skip non-PDF files
        if not filename.lower().endswith(".pdf"):
            logging.info("  Skipping non-PDF document")
            continue

        source = os.path.join(DIRECTORY_SOURCE, filename)
        output = os.path.join(DIRECTORY_OUTPUT, filename.rsplit(".", 1)[0] + ".json")

        # Skip already processed files
        if os.path.exists(output):
            logging.info("  Skipping already processed document")
            continue

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

        # Segment the combined text
        logging.info("  Segmenting content")
        combined = "\n".join(texts)
        segmented = segment_text(combined)

        # Save the output
        logging.info("  Saving output")
        with open(output, "w", encoding="utf-8") as file:
            file.write(segmented)

        logging.info(f"  Took {time.time() - start:.2f}s")


if __name__ == "__main__":
    main()
