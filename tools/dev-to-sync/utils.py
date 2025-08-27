import re

BLOGS_PATH = "src/content/blog/"
DRAFT_PATH = "src/content/draft/"

def file_path_from_canonical_url(canonical_url: str, is_published: bool) -> str:
    path = canonical_url.removeprefix("https://adityais.dev/blog/").removesuffix("/")
    return f"{BLOGS_PATH}{path}.md" if is_published else f"{DRAFT_PATH}{path}.md"

def generate_file_path_from_title(title: str, is_published: bool) -> str:
    # TODO: Verify if that slug is unique and new
    slug = title.lower().replace(" ", "-")
    slug = re.sub(r'[^a-z0-9\-]', '', slug)

    return f"{BLOGS_PATH}{slug}.md" if is_published else f"{DRAFT_PATH}{slug}.md"
