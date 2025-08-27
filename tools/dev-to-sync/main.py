import os
import requests
import utils

API_KEY = os.getenv("DEV_TO_API_KEY")
AUTH_HEADER = {"api-key": API_KEY}

page_number = 1

while True:
    articles = requests.get(
        "https://dev.to/api/articles/me/all",
        headers=AUTH_HEADER,
        params={
            "page": page_number,
            "per_page": 30
        }
    )

    if articles.status_code != 200:
        print(f"Got Status Code: {articles.status_code}")
        break

    articles = articles.json()


    if len(articles) == 0:
        break

    for article in articles:
        if article["user"]["username"].lower() != "as1100k":
            continue

        is_published = article["published"]
        path_already_exists = article["canonical_url"] is not None and str(article["canonical_url"]).startswith("https://adityais.dev")

        (file_path, slug) = (
            utils.file_path_from_canonical_url(article["canonical_url"], is_published)
            if path_already_exists
            # TODO: Make sure we don't overwrite anything
            else utils.generate_file_path_from_title(article["title"], is_published)
        )

        if is_published and not path_already_exists:
            res = requests.put(
                f"https://dev.to/api/articles/{article['id']}",
                headers=AUTH_HEADER,
                json={
                    "article": {
                        "canonical_url": f"https://adityais.dev/blog/{slug}"
                    }
                }
            )
            print(f"Updated Cononical URL for Post ID: {article['id']}.")

            if res.status_code != 200:
                print(f"    Error Response: {res.json()}")

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

        print(f"Generated: {file_path}")

    page_number += 1
