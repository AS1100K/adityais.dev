import { lexicalEditor } from '@payloadcms/richtext-lexical'
import type { CollectionConfig } from 'payload'

export const Blogs: CollectionConfig = {
  slug: 'blogs',
  access: {},
  fields: [
    {
      name: 'title',
      type: 'text',
    },
    {
      name: 'content',
      type: 'richText',
      editor: lexicalEditor({}),
    },
  ],
  admin: {
    preview: ({ slug }) => `http://localhost:3000/blogs-draft/${slug}`, // TODO: Support the real current domain
    useAsTitle: 'title',
    enableRichTextLink: true,
    listSearchableFields: ['title'],
  },
}
