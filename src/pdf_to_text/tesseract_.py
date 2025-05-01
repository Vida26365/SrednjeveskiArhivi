# občasno zgreši del teksta
# 3 prebere kot 5

from PIL import Image
import pytesseract
import os

zapisi = "zapisi"
jpgji = "jpgji"
txtji = "txtji_tesseract"
jpg_mapa = os.path.join(zapisi, jpgji)

# Koristne stvari na: https://pypi.org/project/pytesseract/

pytesseract.pytesseract.tesseract_cmd = r"C:\Program Files\Tesseract-OCR\tesseract.exe"

os.makedirs(os.path.join(zapisi, txtji), exist_ok=True)
for mapa in os.listdir(jpg_mapa):
    izhod_path = os.path.join(zapisi, txtji,  str(mapa)+".txt")
    # Tu je "a" da se ne prepiše datotek iz večih pdfjev
    with open(izhod_path, "a", encoding="utf-8") as f:
        for jpg in os.listdir(os.path.join(jpg_mapa, mapa)):
            img_path = os.path.join(zapisi, jpgji, mapa, jpg)
            besedilo = pytesseract.image_to_string(Image.open(img_path), lang='lat+slv')
            f.write(besedilo)



# List of available languages
# print(pytesseract.get_languages(config=''))

# French text image to string
# print(pytesseract.image_to_string(Image.open('zapisi/jpgji/GZL I-1 (1243 april 13)-1/1.jpg'), lang='lat+slv'))



# Get verbose data including boxes, confidences, line and page numbers
# print(pytesseract.image_to_data(Image.open('zapisi/jpgji/GZL I-1 (1243 april 13)-1/1.jpg')))

