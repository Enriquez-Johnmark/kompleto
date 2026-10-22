/** When a checklist document is owed, as the bookkeeper reads it. */
export const recursLabels: Record<string, string> = {
  "every-period": "Every period",
  "quarter-end": "At quarter end",
  "year-end": "In December",
}

export function recursLabel(recurs: string) {
  return recursLabels[recurs] ?? recurs
}
