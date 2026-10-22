import type { ReactNode } from "react"
import { Link } from "react-router"

/**
 * Renders the small Markdown subset our own documents use (help, privacy,
 * terms): headings, paragraphs, lists, notes, bold, links and code. It
 * builds React elements, never HTML strings, and is not meant for text
 * anyone else wrote.
 */

export type Block =
  | { kind: "heading"; level: 1 | 2 | 3; text: string; id: string }
  | { kind: "paragraph"; text: string }
  | { kind: "list"; ordered: boolean; items: string[] }
  | { kind: "note"; text: string }

const headingLine = /^(#{1,3}) (.+)$/
const itemLine = /^(?:- |\d+\. )(.*)$/

/** Splits `source` into blocks: blank lines end a block. */
export function parseBlocks(source: string): Block[] {
  const blocks: Block[] = []
  let lines: string[] = []
  const flush = () => {
    if (lines.length > 0) blocks.push(toBlock(lines))
    lines = []
  }
  for (const line of source.split("\n")) {
    const heading = headingLine.exec(line)
    if (heading) {
      flush()
      const text = heading[2]
      blocks.push({ kind: "heading", level: heading[1].length as 1 | 2 | 3, text, id: slug(text) })
    } else if (line.trim() === "") {
      flush()
    } else {
      lines.push(line)
    }
  }
  flush()
  return blocks
}

function toBlock(lines: string[]): Block {
  if (lines.every((line) => line.startsWith(">"))) {
    return { kind: "note", text: lines.map((line) => line.replace(/^> ?/, "")).join(" ") }
  }
  if (itemLine.test(lines[0])) {
    // Lines that don't start an item continue the one before.
    const items: string[] = []
    for (const line of lines) {
      const item = itemLine.exec(line)
      if (item) items.push(item[1])
      else items[items.length - 1] += ` ${line.trim()}`
    }
    return { kind: "list", ordered: /^\d/.test(lines[0]), items }
  }
  return { kind: "paragraph", text: lines.map((line) => line.trim()).join(" ") }
}

/** A heading's anchor: "3. Who helps us" → "who-helps-us". */
export function slug(text: string): string {
  return text
    .toLowerCase()
    .replace(/^[\d.\s]+/, "")
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-|-$/g, "")
}

const inline = /\*\*(.+?)\*\*|\[([^\]]+)\]\(([^)\s]+)\)|`([^`]+)`/g

/** Bold, links and code within one block's text. */
export function renderInline(text: string): ReactNode[] {
  const nodes: ReactNode[] = []
  let last = 0
  for (const match of text.matchAll(inline)) {
    const [whole, bold, label, href, code] = match
    if (match.index > last) nodes.push(text.slice(last, match.index))
    const key = match.index
    if (bold !== undefined) nodes.push(<strong key={key}>{renderInline(bold)}</strong>)
    else if (code !== undefined)
      nodes.push(
        <code key={key} className="rounded bg-muted px-1 py-0.5 font-mono text-[0.9em]">
          {code}
        </code>
      )
    else nodes.push(<DocLink key={key} href={href} label={label} />)
    last = match.index + whole.length
  }
  if (last < text.length) nodes.push(text.slice(last))
  return nodes
}

const linkClass = "font-medium text-primary underline underline-offset-4"

function DocLink({ href, label }: { href: string; label: string }) {
  if (href.startsWith("/") || href.startsWith("#"))
    return (
      <Link to={href} className={linkClass}>
        {label}
      </Link>
    )
  // Only web and email links; anything else stays plain text.
  if (!/^(https:|mailto:)/.test(href)) return label
  return (
    <a href={href} className={linkClass} target="_blank" rel="noreferrer">
      {label}
    </a>
  )
}

export function Markdown({ source }: { source: string }) {
  return (
    <div className="flex flex-col gap-4 leading-7">
      {parseBlocks(source).map((block, index) => (
        <BlockView key={index} block={block} />
      ))}
    </div>
  )
}

function BlockView({ block }: { block: Block }) {
  switch (block.kind) {
    case "heading": {
      const content = renderInline(block.text)
      if (block.level === 1)
        return (
          <h1 id={block.id} className="text-3xl font-semibold tracking-tight">
            {content}
          </h1>
        )
      if (block.level === 2)
        return (
          <h2 id={block.id} className="mt-4 scroll-mt-6 text-xl font-semibold tracking-tight">
            {content}
          </h2>
        )
      return (
        <h3 id={block.id} className="mt-2 scroll-mt-6 font-semibold">
          {content}
        </h3>
      )
    }
    case "paragraph":
      return <p>{renderInline(block.text)}</p>
    case "note":
      return (
        <p className="rounded-lg border border-warning/40 bg-warning/10 px-4 py-3 text-sm">
          {renderInline(block.text)}
        </p>
      )
    case "list": {
      const items = block.items.map((item, index) => <li key={index}>{renderInline(item)}</li>)
      return block.ordered ? (
        <ol className="flex list-decimal flex-col gap-1 pl-6">{items}</ol>
      ) : (
        <ul className="flex list-disc flex-col gap-1 pl-6">{items}</ul>
      )
    }
  }
}
