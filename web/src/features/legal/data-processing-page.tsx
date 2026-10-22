import { DocumentLayout } from "@/components/document-layout"
import agreement from "@/features/legal/data-processing-agreement.md?raw"
import { Markdown } from "@/lib/markdown"

/** The data processing agreement between each Firm (controller) and
 *  Kompleto (processor). */
export function DataProcessingPage() {
  return (
    <DocumentLayout>
      <Markdown source={agreement} />
    </DocumentLayout>
  )
}
