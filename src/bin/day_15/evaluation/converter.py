import argparse
from PIL import Image

parse = argparse.ArgumentParser(description="Parse block and output as image")
parse.add_argument("-i", "--input", help='input file')

args = parse.parse_args()


with open(args.input, "r") as f:
    data = list(map(lambda l: l.rstrip("\n"), f))

# verify we have a rectangle as an input
assert all(map(lambda l: len(data[0]) == len(l), data)), "not rectangle"

width, height = (len(data[0]), len(data))

img = Image.new(mode="1", size=(width, height))

print(f"s = ({width}, {height})")
for y, row in enumerate(data):
    for x, val in enumerate(row):
        img.putpixel((x, y), 0 if (val == " ") else 1)

img.save(f"{args.input}.png", "PNG")
