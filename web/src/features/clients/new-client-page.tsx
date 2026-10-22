import { useNavigate } from "react-router"

import { PageHeader } from "@/components/page-header"
import { toast } from "@/components/ui/toast"
import { ClientForm, emptyClient } from "@/features/clients/client-form"
import { useCreateClient } from "@/features/clients/queries"

export function NewClientPage() {
  const navigate = useNavigate()
  const create = useCreateClient()
  return (
    <>
      <PageHeader
        title="Add a client"
        description="Reminders start from the next period with something due."
      />
      <ClientForm
        defaultValues={emptyClient}
        submitLabel="Add client"
        mutation={create}
        onSaved={({ id }) => {
          toast.add({ title: "Client added", type: "success" })
          navigate(`/clients/${id}`)
        }}
      />
    </>
  )
}
