import json
import os
import subprocess
from glob import glob

TOTAL_KEYWORDS = [
    "AttributeBegin",
    "AttributeEnd",
    "Attribute",
    "ActiveTransform",
    "AreaLightSource",
    "Accelerator",
    "ConcatTransform",
    "CoordinateSystem",
    "CoordSysTransform",
    "ColorSpace",
    "Camera",
    "Film",
    "Integrator",
    "Include",
    "Identity",
    "LightSource",
    "LookAt",
    "MakeNamedMaterial",
    "MakeNamedMedium",
    "Material",
    "MediumInterface",
    "NamedMaterial",
    "ObjectBegin",
    "ObjectEnd",
    "ObjectInstance",
    "Option",
    "PixelFilter",
    "ReverseOrientation",
    "Rotate",
    "Shape",
    "Sampler",
    "Scale",
    "TransformBegin",
    "TransformEnd",
    "Transform",
    "Translate",
    "TransformTimes",
    "Texture",
    "WorldBegin",
]


def bash(cmd: str):
    p = subprocess.run(cmd, shell=True, capture_output=True)
    return p.returncode, p.stdout.decode("utf-8"), p.stderr.decode("utf-8")


def red(text: str) -> str:
    return "\033[91m" + text + "\033[0m"


def tokenize(content: list[str]) -> list[str]:
    tokens = []
    for line in content:
        begin = line.find("#")
        line = (line if begin == -1 else line[:begin]).strip()
        if len(line) == 0:
            continue

        tokens += line.split()

    assembled_tokens = []
    idx = 0
    while True:
        if idx >= len(tokens):
            break

        current_token = tokens[idx]
        if not current_token.startswith("\"") or current_token.endswith("\""):
            assembled_tokens.append(current_token)
            idx += 1
            continue

        # assemble tokens split by quote
        assembled_tokens.append(tokens[idx] + " " + tokens[idx + 1])
        idx += 2

    def trim_quote(token: str) -> str:
        if token[0] == "\"" and token[-1] == "\"" or token[
                0] == "\'" and token[-1] == "\'":
            return token[1:-1]
        return token

    return list(map(trim_quote, assembled_tokens))


def split_by_keyword(tokens: list[str], out_root: str,
                     in_root: str) -> list[list[str]]:
    upper_case_token = []
    semantic_tokens = []
    start_idx = 0
    for idx, current_token in enumerate(tokens):
        if idx == 0:
            continue

        if current_token in TOTAL_KEYWORDS:
            semantic_tokens.append(tokens[start_idx:idx])
            start_idx = idx
            continue

        if current_token[0].isupper():
            upper_case_token.append(current_token)

    semantic_tokens.append(tokens[start_idx:len(tokens)])

    for idx in range(len(semantic_tokens)):
        current_token = semantic_tokens[idx]
        if len(current_token) != 2:
            continue

        if current_token[0] == "Include" and current_token[1].endswith(
                ".pbrt"):
            included_pbrt_file = current_token[1]

            basename = os.path.basename(included_pbrt_file).replace(
                ".pbrt", ".json")

            included_folder = os.path.dirname(included_pbrt_file)

            out_dir = "{}/{}".format(out_root, included_folder)
            converted_json_file = "{}/{}".format(out_dir, basename)
            convert_pbrt(converted_json_file,
                         "{}/{}".format(in_root, included_pbrt_file))

            semantic_tokens[idx][1] = "{}/{}".format(included_folder, basename)

    for token in sorted(set(upper_case_token)):
        print("Did you miss parsing {} ?".format(red(token)))

    return semantic_tokens


def cancel_brackets(token_list: list[str]) -> list[str]:
    if "[" not in token_list:
        return token_list

    start_bracket_idx = token_list.index("[")
    end_bracket_idx = token_list.index("]")

    return cancel_brackets(token_list[:start_bracket_idx] +
                           [token_list[start_bracket_idx +
                                       1:end_bracket_idx]] +
                           token_list[end_bracket_idx + 1:])


def convert_pbrt(out_file: str, pbrt_file: str):
    print("parse `{}`".format(pbrt_file))

    with open(pbrt_file) as f:
        content = f.read().splitlines()

    tokens = tokenize(content)

    out_dir = os.path.dirname(out_file)
    bash("mkdir -p {}".format(out_dir))

    semantic_block = split_by_keyword(tokens, out_dir,
                                      os.path.dirname(pbrt_file))

    semantic_block = list(map(cancel_brackets, semantic_block))

    data = {}
    for idx, block in enumerate(semantic_block):
        data["token_{}".format(idx)] = block.copy()
    data["length"] = len(semantic_block)

    with open(out_file, 'w') as f:
        if os.path.getsize(pbrt_file) / 1024 < 4:
            json.dump(data, f, indent=4)
        else:
            json.dump(data, f)

    print("json file saved to `{}`".format(out_file))


if __name__ == '__main__':
    in_dir = "/home/wentao/Desktop/pbrt-v4-scenes/killeroos"
    out_dir = "/home/wentao/Desktop/pbrt-v4-scenes-json/killeroos"

    bash("mkdir -p {}".format(out_dir))
    for pbrt_file in glob("{}/*.pbrt".format(in_dir)):
        basename = os.path.basename(pbrt_file)
        if basename != "killeroo-gold.pbrt":
            print("skip `{}`".format(pbrt_file))
            continue

        out_file = "{}/{}".format(out_dir, basename.replace(".pbrt", ".json"))
        convert_pbrt(out_file, pbrt_file)
