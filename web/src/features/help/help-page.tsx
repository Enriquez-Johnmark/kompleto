import { DocumentLayout } from "@/components/document-layout"
import help from "@/features/help/help.md?raw"
import { type Block, Markdown, parseBlocks } from "@/lib/markdown"

type Heading = Extract<Block, { kind: "heading" }>

// The title and introduction, then the contents, then the sections.
const firstSection = help.indexOf("\n## ")
const intro = help.slice(0, firstSection)
const body = help.slice(firstSection)
const sections = parseBlocks(body).filter(
  (block): block is Heading => block.kind === "heading" && block.level === 2
)

/** How Kompleto works, for bookkeepers; open to anyone. */
export function HelpPage() {
  return (
    <DocumentLayout>
      <Markdown source={intro} />
      <nav aria-label="On this page" className="my-8 rounded-lg border bg-muted/40 p-4 text-sm">
        <p className="mb-2 font-medium">On this page</p>
        <ul className="grid gap-1 sm:grid-cols-2">
          {sections.map((section) => (
            <li key={section.id}>
              <a href={`#${section.id}`} className="text-muted-foreground hover:text-foreground">
                {section.text}
              </a>
            </li>
          ))}
        </ul>
      </nav>
      <Markdown source={body} />
    </DocumentLayout>
  )
}
