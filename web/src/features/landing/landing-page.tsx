import { Link, ScrollRestoration } from "react-router"

import dashboard from "../../../../docs/showcase/desktop-1-dashboard.png"
import payroll from "../../../../docs/showcase/desktop-10-payroll-chasing.png"
import payorImport from "../../../../docs/showcase/desktop-11-payors-from-a-sales-export.png"
import review from "../../../../docs/showcase/desktop-3-review-before-approving.png"
import uploadPage from "../../../../docs/showcase/phone-4-client-upload-page-english.png"
import walkthrough from "../../../../docs/showcase/walkthrough.mp4"
import { Logo } from "@/components/logo"
import { Button } from "@/components/ui/button"
import { ChecklistSample } from "@/features/landing/checklist-sample"

const sections = [
  { href: "#how", label: "How it works" },
  { href: "#demo", label: "Demo" },
  { href: "#features", label: "Features" },
  { href: "#pricing", label: "Pricing" },
  { href: "#questions", label: "Questions" },
]

const steps = [
  {
    title: "Add your clients",
    text: "Type them in, or import a spreadsheet, a Xero contact export or a QuickBooks customer list. Pick the BIR forms each one files.",
  },
  {
    title: "Approve the morning's drafts",
    text: "At 7:00 each morning, Kompleto drafts the reminders that are due. Read them, edit if you like, and approve them all at once.",
  },
  {
    title: "Clients upload from their phone",
    text: "Each email carries the client's own private link. No app, no password. They pick what each photo or PDF is and send it.",
  },
  {
    title: "Download and mark received",
    text: "Files land on the client's page. Reminders stop once everything is in, or as soon as the client replies.",
  },
]

const features = [
  {
    title: "BIR calendar built in",
    text: "Deadlines for 2551Q, 1601-C, 0619-E, 1701Q and more, moved off weekends and holidays. Enter a BIR extension for any client.",
  },
  {
    title: "Taglish or English",
    text: "Each client gets reminders in their language: a first request after the period ends, then reminders 7, 3 and 1 day before your cut-off.",
  },
  {
    title: "You approve everything",
    text: "No email leaves without your approval, and each approved email is sent once. Drafts you don't approve expire at the cut-off.",
  },
  {
    title: "Copy for Viber",
    text: "Some clients only answer on Viber. Copy the same message, upload link included, into your own chat.",
  },
  {
    title: "Stops on its own",
    text: "Reminders stop once a client uploads everything or replies, and you get an email when they reply. Bounced addresses show on the dashboard.",
  },
  {
    title: "Staff logins",
    text: "Invite up to 5 staff, each with their own login. Only the owner can export data or close the account, and the activity log shows who did what.",
  },
]

const limits = [
  {
    title: "No books, no tax, no filing",
    text: "Kompleto doesn't keep books, compute tax, prepare returns or produce BIR files. Your work and your filings stay with you.",
  },
  {
    title: "Files protected, then deleted",
    text: "Every upload is scanned for viruses and encrypted. Files are deleted 30 days after you mark them received, and after 90 days at most.",
  },
  {
    title: "AI never sees documents",
    text: "If AI wording is on, the model sees only names, the documents owed and dates: never uploaded files, TINs or amounts. Without it, reminders use fixed templates.",
  },
]

const questions = [
  {
    question: "Do my clients need to install anything?",
    answer:
      "No. Each reminder has a private link that opens in their phone's browser. They choose what each file is and upload photos or PDFs of up to 10 MB. The link works only for that client and that period.",
  },
  {
    question: "Will Kompleto send something without me knowing?",
    answer:
      "No. Every reminder waits on your Drafts page until you or your staff approve it. Login emails and the notice that a client replied go only to your firm.",
  },
  {
    question: "Can my clients see each other's files?",
    answer:
      "No. Clients can upload but can't see, download or delete files. Only your firm can download them.",
  },
  {
    question: "Does it work with Xero or QuickBooks?",
    answer:
      "You can import your client list from a Xero contact export or a QuickBooks customer list, and fill a client's Payors from a sales export. Kompleto reads only names and emails, and ignores other columns such as TINs.",
  },
  {
    question: "What if the BIR extends a deadline?",
    answer:
      "Enter the extended date on the client's Deadlines tab. Kompleto moves deadlines off weekends and holidays, but always check against the BIR's own announcements.",
  },
]

function Eyebrow({ children }: { children: string }) {
  return (
    <p className="font-mono text-xs font-semibold tracking-widest text-primary uppercase">
      {children}
    </p>
  )
}

function SectionHead({ eyebrow, title, text }: { eyebrow: string; title: string; text?: string }) {
  return (
    <div className="mb-10 grid max-w-2xl gap-3">
      <Eyebrow>{eyebrow}</Eyebrow>
      <h2 className="font-heading text-3xl font-bold tracking-tight text-balance md:text-4xl">
        {title}
      </h2>
      {text ? <p className="text-lg text-muted-foreground">{text}</p> : null}
    </div>
  )
}

function Screenshot({ src, alt }: { src: string; alt: string }) {
  return <img src={src} alt={alt} loading="lazy" className="w-full rounded-lg border bg-card" />
}

function RequestAccount({ size = "lg" }: { size?: "default" | "lg" }) {
  return (
    <Button size={size} render={<Link to="/signup" />} nativeButton={false}>
      Request an account
    </Button>
  )
}

/** What Kompleto is and why a bookkeeper would want it; open to anyone, and
 *  what a visitor who isn't logged in sees first. */
export function LandingPage() {
  return (
    <div className="min-h-svh bg-background">
      <ScrollRestoration />
      <header className="sticky top-0 z-10 border-b bg-background/90 backdrop-blur">
        <div className="mx-auto flex max-w-6xl items-center gap-6 px-4 py-3">
          <Link to="/welcome" aria-label="Kompleto home">
            <Logo />
          </Link>
          <nav aria-label="Sections" className="hidden gap-5 text-sm md:flex">
            {sections.map((section) => (
              <a
                key={section.href}
                href={section.href}
                className="text-muted-foreground hover:text-foreground"
              >
                {section.label}
              </a>
            ))}
          </nav>
          <div className="ml-auto flex items-center gap-2">
            <Button variant="ghost" render={<Link to="/login" />} nativeButton={false}>
              Log in
            </Button>
            <RequestAccount size="default" />
          </div>
        </div>
      </header>

      <main>
        <section className="mx-auto grid max-w-6xl items-center gap-12 px-4 py-14 md:grid-cols-[1.15fr_0.85fr] md:py-20">
          <div className="min-w-0">
            <Eyebrow>For bookkeepers in the Philippines</Eyebrow>
            <h1 className="mt-4 font-heading text-4xl leading-[1.05] font-extrabold tracking-tight text-balance md:text-6xl">
              Your clients send their documents.{" "}
              <span className="text-primary">You stop chasing.</span>
            </h1>
            <p className="mt-5 max-w-xl text-lg text-muted-foreground">
              Kompleto drafts the reminder emails for every client's BIR deadlines, in Taglish or
              English. You approve them. Your clients upload receipts and statements from their
              phone, and the reminders stop once everything is in.
            </p>
            <div className="mt-8 flex flex-wrap gap-3">
              <RequestAccount />
              <Button size="lg" variant="outline" render={<a href="#demo" />} nativeButton={false}>
                Watch the 2½-minute demo
              </Button>
            </div>
            <p className="mt-4 text-sm text-muted-foreground">
              Free for 4 weeks. Nothing is sent without your approval.
            </p>
          </div>
          <ChecklistSample />
        </section>

        <section aria-label="The problem" className="bg-foreground text-background">
          <div className="mx-auto flex max-w-6xl flex-wrap items-baseline gap-x-8 gap-y-2 px-4 py-8">
            <p className="max-w-3xl font-heading text-xl font-medium">
              Every month it's the same: Viber threads, one spreadsheet per client, and the same
              message sent dozens of times.
            </p>
          </div>
        </section>

        <section id="how" className="mx-auto max-w-6xl scroll-mt-16 px-4 py-16 md:py-20">
          <SectionHead
            eyebrow="How it works"
            title="Four steps, and most of them happen without you."
          />
          <ol className="grid gap-8 sm:grid-cols-2 lg:grid-cols-4">
            {steps.map((step, index) => (
              <li key={step.title} className="grid content-start gap-2">
                <span className="font-heading text-4xl font-extrabold text-primary">
                  {index + 1}
                </span>
                <h3 className="font-heading text-lg font-semibold">{step.title}</h3>
                <p className="text-muted-foreground">{step.text}</p>
              </li>
            ))}
          </ol>
        </section>

        <section id="demo" className="scroll-mt-16 border-t">
          <div className="mx-auto max-w-6xl px-4 py-16 md:py-20">
            <SectionHead
              eyebrow="See it working"
              title="A bookkeeper's week, in two and a half minutes."
              text="A demo firm with 15 made-up clients, three days before the December cut-off."
            />
            <video
              controls
              playsInline
              preload="metadata"
              poster={dashboard}
              src={walkthrough}
              className="aspect-video w-full rounded-xl border bg-card"
            >
              <track kind="captions" />
            </video>
            <div className="mt-8 grid gap-6 md:grid-cols-3">
              <figure className="grid content-start gap-2">
                <Screenshot
                  src={dashboard}
                  alt="The dashboard: each client's missing documents, nearest cut-off first"
                />
                <figcaption className="text-sm text-muted-foreground">
                  The dashboard: what each client still owes, nearest cut-off first.
                </figcaption>
              </figure>
              <figure className="grid content-start gap-2">
                <Screenshot
                  src={review}
                  alt="Reviewing several reminder drafts in full before approving them"
                />
                <figcaption className="text-sm text-muted-foreground">
                  Review every reminder exactly as it will go out, then approve.
                </figcaption>
              </figure>
              <figure className="grid content-start gap-2">
                <img
                  src={uploadPage}
                  alt="The client's upload page on a phone"
                  loading="lazy"
                  className="mx-auto w-full max-w-56 rounded-lg border bg-card"
                />
                <figcaption className="text-sm text-muted-foreground">
                  What your client sees: what's still missing, and an upload button.
                </figcaption>
              </figure>
            </div>
          </div>
        </section>

        <section id="features" className="scroll-mt-16 border-t">
          <div className="mx-auto max-w-6xl px-4 py-16 md:py-20">
            <SectionHead
              eyebrow="What's included"
              title="Built around how Philippine bookkeepers already work."
            />
            <div className="grid gap-x-12 md:grid-cols-2">
              {features.map((feature) => (
                <div key={feature.title} className="grid gap-1.5 border-t py-5">
                  <h3 className="font-heading text-lg font-semibold">{feature.title}</h3>
                  <p className="text-muted-foreground">{feature.text}</p>
                </div>
              ))}
            </div>

            <div className="mt-16 grid items-center gap-8 md:grid-cols-[0.9fr_1.1fr]">
              <div className="grid gap-3">
                <Eyebrow>2307 tracker</Eyebrow>
                <h3 className="font-heading text-2xl font-bold text-balance">
                  Know which 2307s are still missing, Payor by Payor.
                </h3>
                <p className="text-muted-foreground">
                  List the businesses that withhold tax from each client, or pick them from a Xero
                  or QuickBooks sales export: Kompleto keeps only the names of customers who
                  withheld tax. Reminders then ask for each missing 2307 by name.
                </p>
              </div>
              <Screenshot
                src={payorImport}
                alt="Picking Payors from the customers who withheld tax in a sales export"
              />
            </div>
            <div className="mt-14 grid items-center gap-8 md:grid-cols-[1.1fr_0.9fr]">
              <Screenshot
                src={payroll}
                alt="A client's payroll chasing settings: paydays and contribution due days"
              />
              <div className="grid gap-3 max-md:row-start-1">
                <Eyebrow>Payroll chasing</Eyebrow>
                <h3 className="font-heading text-2xl font-bold text-balance">
                  Timesheets before payday. Receipts after the due date.
                </h3>
                <p className="text-muted-foreground">
                  For clients who run payroll, Kompleto asks for timesheets or DTRs, overtime and
                  leave approvals, and new hires and resignations before each payday. After each due
                  date, it asks for the SSS, PhilHealth and Pag-IBIG payment receipts. It never
                  computes pay.
                </p>
              </div>
            </div>
          </div>
        </section>

        <section className="border-t">
          <div className="mx-auto max-w-6xl px-4 py-16 md:py-20">
            <SectionHead eyebrow="Safe by design" title="It chases documents. Nothing more." />
            <div className="grid gap-8 md:grid-cols-3">
              {limits.map((limit) => (
                <div key={limit.title} className="grid content-start gap-2">
                  <h3 className="font-heading text-lg font-semibold">{limit.title}</h3>
                  <p className="text-muted-foreground">{limit.text}</p>
                </div>
              ))}
            </div>
          </div>
        </section>

        <section id="pricing" className="scroll-mt-16 border-t">
          <div className="mx-auto max-w-6xl px-4 py-16 md:py-20">
            <SectionHead eyebrow="Pricing" title="Try it free with your real clients." />
            <div className="grid overflow-hidden rounded-xl border bg-card md:grid-cols-2">
              <div className="grid content-start gap-3 p-8">
                <h3 className="font-heading text-lg font-semibold">First 4 weeks</h3>
                <p className="font-heading text-4xl font-extrabold">
                  ₱0 <span className="text-base font-medium text-muted-foreground">to try</span>
                </p>
                <ul className="grid list-disc gap-1.5 pl-5 text-muted-foreground">
                  <li>All features, all your clients</li>
                  <li>No payment details asked</li>
                  <li>Staff logins included</li>
                </ul>
              </div>
              <div className="grid content-start gap-3 border-t p-8 md:border-t-0 md:border-l">
                <h3 className="font-heading text-lg font-semibold">After that</h3>
                <p className="font-heading text-4xl font-extrabold">
                  ₱1,500{" "}
                  <span className="text-base font-medium text-muted-foreground">
                    a month per firm
                  </span>
                </p>
                <ul className="grid list-disc gap-1.5 pl-5 text-muted-foreground">
                  <li>Paid by GCash, month by month</li>
                  <li>Download all your data any time</li>
                  <li>Close your account whenever you like</li>
                </ul>
              </div>
            </div>
          </div>
        </section>

        <section className="bg-primary/10">
          <div className="mx-auto flex max-w-6xl flex-wrap items-center justify-between gap-6 px-4 py-14">
            <div className="grid max-w-xl gap-2">
              <h2 className="font-heading text-3xl font-bold tracking-tight text-balance">
                Ready to stop chasing?
              </h2>
              <p className="text-lg text-muted-foreground">
                Request an account with your firm's name and email. Once it's approved, a login link
                arrives by email, and your first reminders are drafted the next morning.
              </p>
            </div>
            <RequestAccount />
          </div>
        </section>

        <section id="questions" className="mx-auto max-w-6xl scroll-mt-16 px-4 py-16 md:py-20">
          <SectionHead eyebrow="Questions" title="What bookkeepers ask first." />
          <div className="max-w-3xl">
            {questions.map((item) => (
              <details key={item.question} className="group border-t py-4">
                <summary className="flex cursor-pointer list-none justify-between gap-4 font-heading text-lg font-semibold [&::-webkit-details-marker]:hidden">
                  {item.question}
                  <span aria-hidden="true" className="font-mono text-primary group-open:rotate-45">
                    +
                  </span>
                </summary>
                <p className="mt-2 text-muted-foreground">{item.answer}</p>
              </details>
            ))}
          </div>
        </section>
      </main>

      <footer className="border-t">
        <div className="mx-auto flex max-w-6xl flex-wrap justify-between gap-4 px-4 py-8 text-sm text-muted-foreground">
          <span>Screenshots and video show a demo firm with made-up clients.</span>
          <nav aria-label="Documents" className="flex flex-wrap gap-4">
            <Link to="/help" className="hover:text-foreground">
              Help
            </Link>
            <Link to="/privacy" className="hover:text-foreground">
              Privacy
            </Link>
            <Link to="/terms" className="hover:text-foreground">
              Terms
            </Link>
          </nav>
        </div>
      </footer>
    </div>
  )
}
