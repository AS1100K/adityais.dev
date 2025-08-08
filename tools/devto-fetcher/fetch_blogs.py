#!/usr/bin/env python3
"""
Dev.to Blog Fetcher Tool

Fetches blog posts from dev.to API and generates a JSON file for the website.
"""

import json
import os
import sys
from datetime import datetime
from typing import List, Dict, Any

try:
    import requests
except ImportError:
    print("Error: requests library not found. Run: pip install -r requirements.txt")
    sys.exit(1)


def fetch_devto_articles(username: str, api_key: str = None) -> List[Dict[str, Any]]:
    """
    Fetch articles from dev.to API for a given username.
    
    Args:
        username: Dev.to username
        api_key: Optional API key for higher rate limits
    
    Returns:
        List of article dictionaries
    """
    url = f"https://dev.to/api/articles?username={username}"
    headers = {"User-Agent": "BlogFetcher/1.0"}
    
    if api_key:
        headers["api-key"] = api_key
    
    try:
        response = requests.get(url, headers=headers, timeout=10)
        response.raise_for_status()
        return response.json()
    except requests.RequestException as e:
        print(f"Error fetching articles from dev.to: {e}")
        return []


def get_existing_blogs() -> List[Dict[str, Any]]:
    """
    Get existing blog posts from local markdown files.
    
    Returns:
        List of blog post dictionaries
    """
    existing_blogs = [
        {
            "id": "2024-year-in-review",
            "title": "2024 Year in Review",
            "description": "A comprehensive review of my journey through 2024",
            "url": "/blog/2024-year-in-review.md",
            "published_at": "2024-12-31T00:00:00Z",
            "cover_image": "https://images.unsplash.com/photo-1493612276216-ee3925520721?w=800&h=400&fit=crop",
            "tag_list": ["year-review", "personal", "reflection"],
            "reading_time_minutes": 5,
            "public_reactions_count": 0,
            "comments_count": 0,
            "source": "local"
        },
        {
            "id": "gsoc-2025",
            "title": "GSoC 2025 - My Journey Begins",
            "description": "Getting accepted into Google Summer of Code 2025",
            "url": "/blog/gsoc-2025.md",
            "published_at": "2024-11-15T00:00:00Z",
            "cover_image": "https://images.unsplash.com/photo-1517077304055-6e89abbf09b0?w=800&h=400&fit=crop",
            "tag_list": ["gsoc", "google", "open-source"],
            "reading_time_minutes": 8,
            "public_reactions_count": 0,
            "comments_count": 0,
            "source": "local"
        },
        {
            "id": "introducing-pastey",
            "title": "Introducing Pastey - A Modern Pastebin",
            "description": "A new pastebin service built with modern web technologies",
            "url": "/blog/introducing-pastey.md",
            "published_at": "2024-10-20T00:00:00Z",
            "cover_image": "https://images.unsplash.com/photo-1551288049-bebda4e38f71?w=800&h=400&fit=crop",
            "tag_list": ["project", "web-development", "pastebin"],
            "reading_time_minutes": 6,
            "public_reactions_count": 0,
            "comments_count": 0,
            "source": "local"
        },
        {
            "id": "introducing-release-butler",
            "title": "Introducing Release Butler",
            "description": "Automated release management for your projects",
            "url": "/blog/introducing-release-butler.md",
            "published_at": "2024-09-15T00:00:00Z",
            "cover_image": "https://images.unsplash.com/photo-1516382799247-87df95d790b7?w=800&h=400&fit=crop",
            "tag_list": ["automation", "devops", "tools"],
            "reading_time_minutes": 7,
            "public_reactions_count": 0,
            "comments_count": 0,
            "source": "local"
        }
    ]
    return existing_blogs


def normalize_article(article: Dict[str, Any], source: str = "devto") -> Dict[str, Any]:
    """
    Normalize article data to a consistent format.
    
    Args:
        article: Raw article data
        source: Source of the article ("devto" or "local")
    
    Returns:
        Normalized article dictionary
    """
    if source == "local":
        return article
    
    # Normalize dev.to article format
    return {
        "id": str(article.get("id", "")),
        "title": article.get("title", ""),
        "description": article.get("description", ""),
        "url": article.get("url", ""),
        "published_at": article.get("published_at", ""),
        "cover_image": article.get("cover_image") or article.get("social_image"),
        "tag_list": article.get("tag_list", []),
        "reading_time_minutes": article.get("reading_time_minutes", 5),
        "public_reactions_count": article.get("public_reactions_count", 0),
        "comments_count": article.get("comments_count", 0),
        "source": "devto"
    }


def generate_blog_data(username: str, api_key: str = None) -> Dict[str, Any]:
    """
    Generate complete blog data combining dev.to articles and existing blogs.
    
    Args:
        username: Dev.to username
        api_key: Optional API key
    
    Returns:
        Complete blog data dictionary
    """
    # Fetch dev.to articles
    devto_articles = fetch_devto_articles(username, api_key)
    normalized_devto = [normalize_article(article, "devto") for article in devto_articles]
    
    # Get existing blogs
    existing_blogs = get_existing_blogs()
    
    # Combine all articles
    all_articles = normalized_devto + existing_blogs
    
    # Sort by published date (newest first)
    all_articles.sort(key=lambda x: x.get("published_at", ""), reverse=True)
    
    return {
        "updated_at": datetime.now().isoformat(),
        "total_articles": len(all_articles),
        "devto_articles": len(normalized_devto),
        "local_articles": len(existing_blogs),
        "articles": all_articles
    }


def main():
    """Main function to generate blog data."""
    username = os.getenv("DEVTO_USERNAME", "as1100k")
    api_key = os.getenv("DEVTO_API_KEY")  # Optional
    
    print(f"Fetching blog data for username: {username}")
    
    # Generate blog data
    blog_data = generate_blog_data(username, api_key)
    
    # Write to JSON file
    output_file = "devto-blogs.json"
    with open(output_file, "w", encoding="utf-8") as f:
        json.dump(blog_data, f, indent=2, ensure_ascii=False)
    
    print(f"Generated {output_file} with {blog_data['total_articles']} articles")
    print(f"  - {blog_data['devto_articles']} from dev.to")
    print(f"  - {blog_data['local_articles']} existing local blogs")


if __name__ == "__main__":
    main()