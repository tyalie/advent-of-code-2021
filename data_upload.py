#!/usr/bin/env python3
"""Upload a specific file to the teensy
"""

import argparse
from pathlib import Path

START_CODE = b"\xC0\xDEGO"


def p_file(v: str) -> Path:
    assert (p := Path(v)).is_file(), f"{v} is not a known file"
    return p


def p_char_device(v: str) -> Path:
    assert (p := Path(v)).is_char_device(), f"{v} is not a known character device"
    return p


parser = argparse.ArgumentParser(
    description=__doc__,
    formatter_class=argparse.RawDescriptionHelpFormatter
)
parser.add_argument(
    "-i", "--input", type=p_file, required=True,
    help="Input file to upload"
)
parser.add_argument(
    "-d", "--device", type=p_char_device, required=True,
    help="The serial port referencing the teensy"
)


def main(in_file: Path, device: Path):
    with open(in_file, "rb") as f:
        data = f.read()

    print(f"writing {len(data)} bytes to {device}")

    with open(device, "wb") as d:
        d.write(b"R")  # Reset code
        d.write(START_CODE)
        d.write(len(data).to_bytes(4, byteorder='big'))
        d.flush()
        d.write(data)
        d.flush()


if __name__ == "__main__":
    args = parser.parse_args()
    main(args.input, args.device)

