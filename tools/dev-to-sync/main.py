import os
import requests
from requests.exceptions import RequestException
import utils
import sys

# These articles are used for testing
BLACKLIST_ARTICLE_ID = ["2802437"]

API_KEY = os.getenv("DEV_TO_API_KEY")
if not API_KEY:
    print("Error: DEV_TO_API_KEY environment variable is not set. Please set it to your dev.to API key.")
    sys.exit(1)

AUTH_HEADER = {"api-key": API_KEY}

page_number = 1

while True:
    try:
        articles = requests.get(
            "https://dev.to/api/articles/me/all",
            headers=AUTH_HEADER,
            params={
                "page": page_number,
                "per_page": 30
            }
        )
    except RequestException as e:
        print(f"Network error occurred while fetching articles: {e}")
        break

    if articles.status_code != 200:
        print(f"Got Status Code: {articles.status_code}")
        break

    articles = articles.json()


    if len(articles) == 0:
        break

    for article in articles:
        if str(article["id"]) in BLACKLIST_ARTICLE_ID:
            continue

        if article["user"]["username"].lower() != "as1100k":
            continue

        is_published = article["published"]
        path_already_exists = article["canonical_url"] is not None and str(article["canonical_url"]).startswith("https://adityais.dev")

        (file_path, slug) = (
            utils.file_path_from_canonical_url(article["canonical_url"], is_published)
            if path_already_exists
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
            print(f"Updated Canonical URL for Post ID: {article['id']}.")

            if res.status_code != 200:
                print(f"    Error Response: {res.json()}")

        os.makedirs(os.path.dirname(file_path), exist_ok=True)
        with open(file_path, "w") as f:
            frontmatter_lines = [
                "---",
                'layout: "@layout/blog.astro"',
                f'title: "{article["title"]}"',
                f'description: "{article["description"]}"',
                f'pubDate: {article["published_at"] if is_published else "2025-08-27"}',
                "v1Data:",
                f'    devtoUrl: "{article["url"]}"',
                f'    readingTimeMinutes: {article["reading_time_minutes"]}'
            ]

            if article['cover_image'] is not None:
                            frontmatter_lines.append(f'    coverImage: {article["cover_image"]}')

            frontmatter_lines.append("---")
            frontmatter = "\n".join(frontmatter_lines)

            f.write(f"{frontmatter}\n\n{article['body_markdown']}\n")

        print(f"Generated: {file_path}")

    page_number += 1
