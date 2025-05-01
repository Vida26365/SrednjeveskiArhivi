#Zelo ga matrajo presledki in dejstvo da se za 1 in 0 uporabljata l in o
#Prav tako nima šumnikov
#Spušča vrstice

from paddleocr import PaddleOCR
import os


zapisi = "zapisi"
jpgji = "jpgji"
txtji = "txtji_paddle"
jpg_mapa = os.path.join(zapisi, jpgji)

ocr = PaddleOCR(use_angle_cls=True, lang='en') # need to run only once to download and load model into memory
# img_path = r'zapisi\jpgji\GZL I-1 (1243 april 13)-1\1.jpg'
# result = ocr.ocr(img_path, cls=True)
# for line in result:
#     # print(line)
#     for neki in line:
#         try:
#             print(neki[1])

#         except IndexError:
#             print(neki)


os.makedirs(os.path.join(zapisi, txtji), exist_ok=True)
for mapa in os.listdir(jpg_mapa):
    izhod_path = os.path.join(zapisi, txtji,  str(mapa)+".txt")
    # Tu je "a" da se ne prepiše datotek iz večih pdfjev
    with open(izhod_path, "a", encoding="utf-8") as f:
        for jpg in os.listdir(os.path.join(jpg_mapa, mapa)):
            img_path = os.path.join(zapisi, jpgji, mapa, jpg)
            # besedilo = pytesseract.image_to_string(Image.open(img_path), lang='lat+slv')
            result = ocr.ocr(img_path, cls=True)
            for line in result:
                for neki in line:
                    f.write(neki[1][0] + "\n")
