import os
import requests
import utils

API_KEY = os.getenv("DEV_TO_API_KEY")
AUTH_HEADER = {"api-key": API_KEY}

# TODO: Support for multiple pages
articles = requests.get(
    "https://dev.to/api/articles/me/published",
    headers=AUTH_HEADER
).json()

for article in articles:
    if article["user"]["username"].lower() != "as1100k":
        continue

    file_path = (
        utils.file_path_from_canonical_url(article["canonical_url"])
        if article["canonical_url"] is not None
        # TODO: Make sure we don't overwrite anything
        else utils.generate_slug_from_title(article["title"])
    )

    with open(file_path, "w") as f:
        f.write(f"""---
layout: "@layout/blog.astro"
title: "{article["title"]}"
description: "{article["description"]}"
pubDate: {article["published_at"]}
v1Data:
    devtoUrl: "{article["url"]}"
    readingTimeMinutes: {article["reading_time_minutes"]}
    {f"coverImage: {article['cover_image']}" if article['cover_image'] is not None else "" }
---

{article["body_markdown"]}
            """)
