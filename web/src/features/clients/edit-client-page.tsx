import { useNavigate, useParams } from "react-router"

import { PageHeader } from "@/components/page-header"
import { QueryError } from "@/components/query-error"
import { Skeleton } from "@/components/ui/skeleton"
import { toast } from "@/components/ui/toast"
import { ClientForm } from "@/features/clients/client-form"
import { useClient, useUpdateClient } from "@/features/clients/queries"

export function EditClientPage() {
  const { id = "" } = useParams()
  const navigate = useNavigate()
  const client = useClient(id)
  const update = useUpdateClient(id)
  return (
    <>
      <PageHeader title="Edit client" description={client.data?.business_name} />
      {client.isPending ? <Skeleton className="h-96" /> : null}
      {client.isError ? <QueryError error={client.error} /> : null}
      {client.data ? (
        <ClientForm
          defaultValues={{
            business_name: client.data.business_name,
            contact_first_name: client.data.contact_first_name,
            email: client.data.email,
            language: client.data.language,
            forms: client.data.forms,
          }}
          submitLabel="Save changes"
          mutation={update}
          onSaved={() => {
            toast.add({ title: "Changes saved", type: "success" })
            navigate(`/clients/${id}`)
          }}
        />
      ) : null}
    </>
  )
}
