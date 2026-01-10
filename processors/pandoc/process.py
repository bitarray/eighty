import sys
import json
import os
import re
import glob
import subprocess

SOURCE = sys.argv[1]

TOC_TEMPLATE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "toc-template.html")

def parse_meta(pandoc_raw, name):
    if not name in pandoc_raw["meta"]:
        return None

    value_raw = pandoc_raw["meta"][name]
    value_raw["t"] = "Para"
    value_wrapped = json.dumps({
        "blocks": [value_raw],
        "pandoc-api-version": pandoc_raw["pandoc-api-version"],
        "meta": {}
    })
    value = subprocess.run("pandoc -f json -t plain", shell=True, check=True, input=value_wrapped.encode(), capture_output=True).stdout.decode("utf-8")

    return value.strip()

file_path = SOURCE
with open(file_path, "rb") as f:
    content = f.read()

if os.path.splitext(file_path)[1] == ".md":
    pandoc_raw = json.loads(subprocess.run("pandoc -f markdown -t json {}".format(file_path), shell=True, check=True, capture_output=True).stdout)

    title = parse_meta(pandoc_raw, "title")
    sitemap_title = parse_meta(pandoc_raw, "sitemap")
    document_id = parse_meta(pandoc_raw, "id")
    description = parse_meta(pandoc_raw, "subtitle")
    order = parse_meta(pandoc_raw, "order")

    if not order is None:
        order = int(order)

    html = subprocess.run("pandoc -f markdown -t html {}".format(file_path), shell=True, check=True, capture_output=True).stdout.decode("utf-8")
    toc = subprocess.run("pandoc --toc -f markdown -t html --template {} {}".format(TOC_TEMPLATE, file_path), shell=True, check=True, capture_output=True).stdout.decode("utf-8")

    content = json.dumps({
        "title": title,
        "description": description,
        "descriptionContent": description,
        "sitemapTitle": sitemap_title,
        "id": document_id,
        "order": order,
        "content": html,
        "toc": toc,
    }, sort_keys=True, indent=4)

elif os.path.splitext(file_path)[1] == ".org":
    pandoc_raw = json.loads(subprocess.run("pandoc -f org -t json {}".format(file_path), shell=True, check=True, capture_output=True).stdout)

    title = parse_meta(pandoc_raw, "title")
    description = parse_meta(pandoc_raw, "subtitle")
    order = parse_meta(pandoc_raw, "order")
    document_id = parse_meta(pandoc_raw, "id")
    sitemap_title = parse_meta(pandoc_raw, "sitemap")

    if not order is None:
        order = int(order)

    html = subprocess.run("pandoc -f org -t html {}".format(file_path), shell=True, check=True, capture_output=True).stdout.decode("utf-8")
    toc = subprocess.run("pandoc --toc -f org -t html --template {} {}".format(TOC_TEMPLATE, file_path), shell=True, check=True, capture_output=True).stdout.decode("utf-8")

    content = json.dumps({
        "title": title,
        "description": description,
        "descriptionContent": description,
        "order": order,
        "content": html,
        "toc": toc,
        "id": document_id,
        "sitemapTitle": sitemap_title,
    }, sort_keys=True, indent=4)
else:
    raise "Unknown file extension"

print(content)
