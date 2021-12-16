import argparse
from PIL import Image
from io import StringIO
import numpy as np


parse = argparse.ArgumentParser(description="Parse block and output as image")
parse.add_argument("-i", "--input", help='input file')
parse.add_argument(
    "--find-matrix", action="store_true",
    help="Find the matrix with GRAPH_START and GRAPH_END code in the input"
)

args = parse.parse_args()

with open(args.input, "r") as f:
    data = list(map(lambda l: l.rstrip(",\n"), f))

if args.find_matrix:
    data = data[data.index("GRAPH_START") + 1:data.index("GRAPH_END")]

in_data = StringIO("\n".join(data))

data = np.loadtxt(in_data, delimiter=",", dtype=np.dtype("uint32"))
u32_max = np.iinfo(data.dtype).max
filtered = data[data < u32_max]
filtered = filtered.max() - filtered

data[data == u32_max] == 0

data = data / (data.max() / 255)

print(data.shape)

img = Image.fromarray(np.uint8(data))
img.save(f"{args.input}.png", "PNG")

