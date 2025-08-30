import re
import os
import random
import string

BLOGS_PATH = "src/content/blog/"
DRAFT_PATH = "src/content/draft/"

def file_path_from_canonical_url(canonical_url: str) -> tuple[str, str]:
    path = canonical_url.removeprefix("https://adityais.dev/blog/").removesuffix("/")
    return (f"{BLOGS_PATH}{path}.md", path)

def generate_file_path_from_title(title: str, is_published: bool) -> tuple[str, str]:
    slug = title.lower().replace(" ", "-")
    slug = re.sub(r'[^a-z0-9\-]', '', slug)

    base_path = BLOGS_PATH if is_published else DRAFT_PATH
    file_path = f"{base_path}{slug}.md"

    suffix = None

    # Check if file exists, if yes, add random suffix
    while os.path.exists(file_path):
        suffix = ''.join(random.choices(string.ascii_lowercase + string.digits, k=6))
        file_path = f"{base_path}{slug}-{suffix}.md"

    if suffix:
        return (file_path, f"{slug}-{suffix}")

    return (file_path, slug)
