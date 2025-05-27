import { BlocksFeature, FixedToolbarFeature, lexicalEditor } from '@payloadcms/richtext-lexical'
import type { CollectionConfig } from 'payload'
import { CodeBlock } from '../blocks/CodeBlock'
import { authenticated, authenticatedOrPublished } from '@/utilities/auth'

export const Blogs: CollectionConfig = {
  slug: 'blogs',
  access: {
    create: authenticated,
    read: authenticatedOrPublished,
    delete: authenticatedOrPublished,
    readVersions: authenticated,
    update: authenticated,
  },
  fields: [
    {
      name: 'title',
      type: 'text',
    },
    {
      type: 'tabs',
      tabs: [
        {
          label: 'Content',
          fields: [
            {
              name: 'heroImage',
              type: 'upload',
              relationTo: 'media',
            },
            {
              name: 'content',
              type: 'richText',
              editor: lexicalEditor({
                features: ({ defaultFeatures, rootFeatures }) => [
                  ...defaultFeatures,
                  ...rootFeatures,
                  FixedToolbarFeature(),
                  BlocksFeature({
                    blocks: [CodeBlock],
                  }),
                ],
              }),
            },
          ],
        },
        {
          label: 'Meta',
          fields: [],
        },
      ],
    },
  ],
  admin: {
    preview: ({ slug }) => `http://localhost:3000/blogs-draft/${slug}`, // TODO: Support the real current domain
    useAsTitle: 'title',
    enableRichTextLink: true,
    listSearchableFields: ['title'],
  },
}
