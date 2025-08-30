import { visit } from "unist-util-visit";

/**
 * Remark plugin to detect consecutive images and convert them to carousels
 */
export function remarkCarousel() {
  return (tree) => {
    const nodesToProcess = [];

    // First pass: find all nodes and identify consecutive image patterns
    visit(tree, (node, index, parent) => {
      if (!parent || index === undefined) return;

      // Look for consecutive paragraph nodes containing only images
      if (
        node.type === "paragraph" &&
        node.children.length === 1 &&
        node.children[0].type === "image"
      ) {
        // Check if this starts a group of consecutive images
        const consecutiveImages = [node];
        let nextIndex = index + 1;

        while (nextIndex < parent.children.length) {
          const nextNode = parent.children[nextIndex];
          if (
            nextNode.type === "paragraph" &&
            nextNode.children.length === 1 &&
            nextNode.children[0].type === "image"
          ) {
            consecutiveImages.push(nextNode);
            nextIndex++;
          } else {
            break;
          }
        }

        // If we found 2 or more consecutive images, mark for processing
        if (consecutiveImages.length > 1) {
          nodesToProcess.push({
            parent,
            startIndex: index,
            nodes: consecutiveImages,
            count: consecutiveImages.length,
          });
        }
      }
    });

    // Process in reverse order to maintain correct indices
    nodesToProcess.reverse().forEach(({ parent, startIndex, nodes, count }) => {
      // Create carousel HTML structure
      const carouselHtml = createCarouselHtml(nodes);

      // Create a raw HTML node for the carousel
      const carouselNode = {
        type: "html",
        value: carouselHtml,
      };

      // Replace the consecutive image nodes with the carousel
      parent.children.splice(startIndex, count, carouselNode);
    });
  };
}

function createCarouselHtml(imageNodes) {
  const images = imageNodes.map((node) => node.children[0]);

  // Generate unique ID for this carousel
  const carouselId = `carousel-${Math.random().toString(36).substring(2, 9)}`;

  // Create slides HTML
  const slidesHtml = images
    .map(
      (img) => `
    <div class="carousel-slide min-w-full flex-shrink-0">
      <img 
        src="${img.url}" 
        alt="${img.alt || ""}" 
        class="w-full h-auto object-contain max-h-[70vh] mx-auto block rounded-lg shadow-lg"
        style="margin: 0 auto;"
      />
    </div>
  `,
    )
    .join("");

  // Create indicators HTML
  const indicatorsHtml = images
    .map(
      (_, index) => `
    <div class="carousel-indicator w-2 h-2 rounded-full bg-gray-300 cursor-pointer transition-colors hover:bg-secondary ${index === 0 ? "active bg-primary" : ""}" 
         data-slide="${index}">
    </div>
  `,
    )
    .join("");

  // Get first image alt text for initial caption
  const firstCaption = images[0]?.alt || "";

  return `
<div class="image-carousel not-prose my-6" data-carousel-id="${carouselId}">
  <div class="carousel-container relative">
    <div class="carousel-track flex transition-transform duration-300 ease-in-out overflow-hidden">
      ${slidesHtml}
    </div>
    
    <button class="carousel-prev absolute left-2 top-1/2 transform -translate-y-1/2 bg-black bg-opacity-50 text-white rounded-full w-10 h-10 flex items-center justify-center hover:bg-opacity-75 transition-opacity z-10" 
            aria-label="Previous image">
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
      </svg>
    </button>
    
    <button class="carousel-next absolute right-2 top-1/2 transform -translate-y-1/2 bg-black bg-opacity-50 text-white rounded-full w-10 h-10 flex items-center justify-center hover:bg-opacity-75 transition-opacity z-10" 
            aria-label="Next image">
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
      </svg>
    </button>
  </div>
  
  <div class="carousel-indicators flex justify-center mt-4 space-x-2">
    ${indicatorsHtml}
  </div>
  
  <div class="carousel-caption text-center mt-2">
    <p class="text-xs text-gray-600 caption-text" style="${firstCaption ? "display: block;" : "display: none;"}">
      ${firstCaption}
    </p>
  </div>
  
  <script type="application/json" class="carousel-data">
    ${JSON.stringify(images.map((img) => ({ alt: img.alt || "", url: img.url })))}
  </script>
</div>`;
}
