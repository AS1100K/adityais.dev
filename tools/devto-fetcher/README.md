# Dev.to Blog Fetcher

A Python tool to fetch blog posts from dev.to API and generate JSON data for the website.

## Features

- Fetches latest articles from dev.to API for a given username
- Combines dev.to articles with existing local blog posts
- Generates unified JSON data file for the website
- Handles API rate limits and errors gracefully
- Includes fallback data for existing local blogs

## Usage

### Basic Usage

```bash
cd tools/devto-fetcher
python3 fetch_blogs.py
```

### Environment Variables

- `DEVTO_USERNAME`: Dev.to username (default: "as1100k")
- `DEVTO_API_KEY`: Optional API key for higher rate limits

### Example

```bash
export DEVTO_USERNAME=as1100k
export DEVTO_API_KEY=your_api_key_here
python3 fetch_blogs.py
```

## Output

The tool generates `devto-blogs.json` with the following structure:

```json
{
  "updated_at": "2024-01-01T00:00:00.000000",
  "total_articles": 5,
  "devto_articles": 1,
  "local_articles": 4,
  "articles": [
    {
      "id": "article-id",
      "title": "Article Title",
      "description": "Article description",
      "url": "https://dev.to/username/article-slug",
      "published_at": "2024-01-01T00:00:00Z",
      "cover_image": "https://image-url.com/image.jpg",
      "tag_list": ["tag1", "tag2"],
      "reading_time_minutes": 5,
      "public_reactions_count": 10,
      "comments_count": 2,
      "source": "devto"
    }
  ]
}
```

## Integration

The tool is integrated into the build process via `package.json`:

```json
{
  "scripts": {
    "generate:blogs": "cd tools/devto-fetcher && python3 fetch_blogs.py && mv devto-blogs.json ../../",
    "build": "npm run generate:blogs && astro check && astro build"
  }
}
```

## Dependencies

- Python 3.6+
- requests library

Install dependencies:

```bash
pip install -r requirements.txt
```

## Error Handling

- If dev.to API is unreachable, the tool will use only local blog data
- Invalid API responses are logged and ignored
- Missing cover images will use placeholder images on the website
- The tool always generates valid JSON output

## Local Blog Data

The tool includes hardcoded data for existing local blog posts:

- 2024 Year in Review
- GSoC 2025 - My Journey Begins
- Introducing Pastey - A Modern Pastebin
- Introducing Release Butler

These are merged with dev.to articles and sorted by publish date (newest first).
