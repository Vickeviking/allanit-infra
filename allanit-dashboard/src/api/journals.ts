import { journals } from "@/mock/journals"
import type { Journal } from "@/mock/journals"

export async function fetchJournals(): Promise<Journal[]> {
  return [...journals].sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime())
}