import { DocumentLayout } from "@/components/document-layout"
import { Markdown } from "@/lib/markdown"

import notice from "@/features/legal/privacy-notice.md?raw"

/** The privacy notice, for Firms and for their clients (linked from the
 *  upload page). */
export function PrivacyPage() {
  return (
    <DocumentLayout>
      <Markdown source={notice} />
    </DocumentLayout>
  )
}
