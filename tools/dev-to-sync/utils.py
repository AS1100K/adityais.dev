import re

BLOGS_PATH = "src/content/blog/"

def file_path_from_canonical_url(canonical_url: str) -> str:
    path = canonical_url.removeprefix("https://adityais.dev/blog/").removesuffix("/")
    return f"{BLOGS_PATH}{path}.md"

def generate_slug_from_title(title: str) -> str:
    # TODO: Verify if that slug is unique and new
    slug = title.lower().replace(" ", "-")
    return re.sub(r'[^a-zA-Z0-9]', '', slug)
