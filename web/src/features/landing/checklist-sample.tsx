/** One client's documents for a month, set like a printed receipt. Example
 *  data only. */
const items = [
  { document: "Sales invoices issued", uploaded: true },
  { document: "Purchase receipts", uploaded: true },
  { document: "Bank and GCash statements", uploaded: false },
  { document: "Payroll register, SSS, PhilHealth, Pag-IBIG", uploaded: false },
  { document: "2307 from Jollibee Foods", uploaded: false },
]

export function ChecklistSample() {
  return (
    <figure
      aria-label="Example: Dela Cruz Pharmacy's December 2026 documents, two uploaded and three not yet, cut-off Sunday 10 January 2027"
      className="relative min-w-0 rotate-1 rounded-sm border bg-card px-6 pt-6 pb-7 font-mono text-[0.8rem] leading-relaxed shadow-[0_18px_40px_-24px_rgb(0_0_0/0.35)] transition-transform duration-300 hover:rotate-0 motion-reduce:transition-none max-md:rotate-0"
    >
      <div className="border-b border-dashed pb-3 text-center">
        <p className="font-semibold tracking-wider">DELA CRUZ PHARMACY</p>
        <p className="text-muted-foreground">December 2026 · 1601-C, 2550Q</p>
      </div>
      <ul className="grid gap-2 py-3">
        {items.map((item) => (
          <li key={item.document} className="flex items-baseline gap-3">
            <span className="min-w-0 flex-1">{item.document}</span>
            <span
              className={
                item.uploaded
                  ? "shrink-0 rounded-full bg-primary/10 px-2 text-[0.7rem] font-semibold text-primary"
                  : "shrink-0 rounded-full bg-amber-100 px-2 text-[0.7rem] font-semibold text-amber-800 dark:bg-amber-950 dark:text-amber-300"
              }
            >
              {item.uploaded ? "Uploaded" : "Not yet"}
            </span>
          </li>
        ))}
      </ul>
      <figcaption className="border-t border-dashed pt-3 text-muted-foreground">
        Cut-off <b className="font-semibold text-foreground">Sun 10 Jan 2027</b>
        <br />
        Next reminder <b className="font-semibold text-foreground">Thu 7 Jan</b>, waiting for your
        approval
      </figcaption>
    </figure>
  )
}
