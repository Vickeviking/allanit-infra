import type { OrderComment } from '@/types/domain'

export default [
  {
    id: "comment-1",
    orderId: 12,
    employeeId: 2, // Alfons
    comment: "Började arbetet tidigt på morgonen som planerat. Allt går enligt schema.",
    createdAt: "2025-11-15T08:30:00Z",
    status: "in_progress"
  },
  {
    id: "comment-2", 
    orderId: 12,
    employeeId: 2, // Alfons
    comment: "Slutförde trappstädningen. Kund var mycket nöjd med resultatet.",
    createdAt: "2025-11-15T14:45:00Z",
    status: "completed"
  },
  {
    id: "comment-3",
    orderId: 14,
    employeeId: 3, // Janus
    comment: "Påbörjade säkerhetskontrollen. Hittade några mindre problem som behöver åtgärdas.",
    createdAt: "2025-11-08T10:15:00Z"
  },
  {
    id: "comment-4",
    orderId: 18,
    employeeId: 3, // Janus
    comment: "Väntar fortfarande på reservdelar från leverantör. Förväntad leverans nästa vecka.",
    createdAt: "2025-11-10T09:00:00Z",
    status: "waiting_for_materials"
  },
  {
    id: "comment-5",
    orderId: 19,
    employeeId: 2, // Alfons
    comment: "Pausade arbetet på grund av kraftigt regn. Återupptar när vädret förbättras.",
    createdAt: "2025-11-10T13:00:00Z",
    status: "paused"
  },
  {
    id: "comment-6",
    orderId: 20,
    employeeId: 2, // Alfons
    comment: "Blockerat - kommunen kräver särskilt tillstånd för återvinningsstationen.",
    createdAt: "2025-11-12T11:30:00Z",
    status: "blocked"
  },
  {
    id: "comment-7",
    orderId: 21,
    employeeId: 5, // Knut Rogerson
    comment: "Målning slutförd, väntar på kvalitetskontroll från kund.",
    createdAt: "2025-11-12T16:00:00Z",
    status: "in_review"
  },
  {
    id: "comment-8",
    orderId: 22,
    employeeId: 5, // Knut Rogerson
    comment: "Utmärkt projekt! Kund var mycket imponerad av kvaliteten.",
    createdAt: "2025-10-30T17:30:00Z",
    status: "completed"
  },
  {
    id: "comment-9",
    orderId: 23,
    employeeId: 5, // Knut Rogerson
    comment: "Började förberedelser för skyddsmålning. Kontrollerade alla säkerhetsprotokoll.",
    createdAt: "2025-12-05T07:45:00Z"
  },
  {
    id: "comment-10",
    orderId: 24,
    employeeId: 5, // Knut Rogerson
    comment: "Väntar fortfarande på godkännande från IKEA. Kontaktade projektledaren igår.",
    createdAt: "2025-11-15T14:20:00Z",
    status: "waiting_for_materials"
  }
] as OrderComment[]
