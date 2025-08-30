---
layout: "@layout/blog.astro"
title: Testing Image Carousels
description: "A demonstration of the automatic carousel functionality for consecutive images in blog posts."
pubDate: 2024-12-20
---

# Testing Image Carousels

This blog post demonstrates the automatic carousel functionality for consecutive images.

## Single Images

Single images continue to render normally:

![A beautiful landscape](https://picsum.photos/800/400?random=1)

Some text content between images.

## Carousel Example

When multiple images appear consecutively without text between them, they automatically form an interactive carousel:

![First image in carousel](https://picsum.photos/800/400?random=2)

![Second image in carousel](https://picsum.photos/800/400?random=3)

![Third image in carousel](https://picsum.photos/800/400?random=4)

The carousel includes:
- Navigation arrows on hover
- Dot indicators for direct navigation  
- Keyboard arrow key support
- Touch/swipe support on mobile
- Caption display from image alt text

## Mixed Content

Another single image:

![Another standalone image](https://picsum.photos/800/400?random=5)

And here's another carousel with just two images:

![Two-image carousel first](https://picsum.photos/800/400?random=6)

![Two-image carousel second](https://picsum.photos/800/400?random=7)

The implementation uses build-time static generation via a custom remark plugin, ensuring optimal performance and SEO.