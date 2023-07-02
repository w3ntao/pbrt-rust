import json
import os
import yaml


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


def split_by_keyword(tokens: list[str]) -> list[list[str]]:
    total_keywords = [
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

    upper_case_token = []
    semantic_tokens = []
    start_idx = 0
    for idx, current_token in enumerate(tokens):
        if idx == 0:
            continue

        if current_token in total_keywords:
            semantic_tokens.append(tokens[start_idx:idx])
            start_idx = idx
            continue

        if current_token[0].isupper():
            upper_case_token.append(current_token)

    semantic_tokens.append(tokens[start_idx:len(tokens)])

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


def convert_root_pbrt(pbrt_file: str):
    print("parse `{}`".format(pbrt_file))

    with open(pbrt_file) as f:
        content = f.read().splitlines()

    tokens = tokenize(content)

    semantic_block = split_by_keyword(tokens)
    semantic_block = list(map(cancel_brackets, semantic_block))

    data = {}
    for idx, block in enumerate(semantic_block):
        data["token_{}".format(idx)] = block.copy()
    data["length"] = len(semantic_block)

    return data


if __name__ == '__main__':
    pbrt_file = "/home/wentao/Desktop/pbrt-v4-scenes/killeroos/killeroo-floor.pbrt"
    data = convert_root_pbrt(pbrt_file)

    out_file = os.path.basename(pbrt_file)
    out_file = out_file[:out_file.rfind(".")] + ".json"
    with open(out_file, 'w') as f:
        json.dump(data, f, indent=4)
    print("file saved to `{}`".format(out_file))
