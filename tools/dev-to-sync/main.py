import os
import requests
import utils

API_KEY = os.getenv("DEV_TO_API_KEY")
AUTH_HEADER = {"api-key": API_KEY}

# TODO: Support for multiple pages
articles = requests.get(
    "https://dev.to/api/articles/me/all",
    headers=AUTH_HEADER
).json()

for article in articles:
    if article["user"]["username"].lower() != "as1100k":
        continue

    is_published = article["published"]

    file_path = (
        utils.file_path_from_canonical_url(article["canonical_url"], is_published)
        if article["canonical_url"] is not None and article["canonical_url"] != article["url"]
        # TODO: Make sure we don't overwrite anything
        else utils.generate_file_path_from_title(article["title"], is_published)
    )

    print(f"Generated: {file_path}")

    os.makedirs(os.path.dirname(file_path), exist_ok=True)
    with open(file_path, "w") as f:
        f.write(f"""---
layout: "@layout/blog.astro"
title: "{article["title"]}"
description: "{article["description"]}"
pubDate: {article["published_at"] if is_published else "2025-08-27"}
v1Data:
    devtoUrl: "{article["url"]}"
    readingTimeMinutes: {article["reading_time_minutes"]}
    {f"coverImage: {article['cover_image']}" if article['cover_image'] is not None else "" }
---

{article["body_markdown"]}
            """)
