import { Block } from 'payload'

export const CodeBlock: Block = {
  slug: 'CodeBlock',
  interfaceName: 'CodeBlock',
  fields: [
    {
      name: 'language',
      type: 'text',
      required: true,
    },
    {
      name: 'code',
      type: 'textarea',
      required: true,
    },
  ],
}
