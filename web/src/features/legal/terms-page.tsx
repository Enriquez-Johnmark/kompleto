import { DocumentLayout } from "@/components/document-layout"
import terms from "@/features/legal/terms.md?raw"
import { Markdown } from "@/lib/markdown"

/** The terms of service between Kompleto and each Firm. */
export function TermsPage() {
  return (
    <DocumentLayout>
      <Markdown source={terms} />
    </DocumentLayout>
  )
}
