import type { MachineFaultReport } from '@/types/domain'

export default [
  {
    id: "fault-1",
    machineId: 2, // Nilfisk 9000
    reportedBy: 2, // Alfons
    title: "Motorproblem",
    description: "Motorn startar inte korrekt och låter konstigt. Behöver service.",
    severity: "high",
    status: "acknowledged",
    reportedAt: "2025-11-05T09:30:00Z",
    notes: "Kontaktat servicefirma, bokad tid nästa vecka"
  },
  {
    id: "fault-2",
    machineId: 5, // Sopmaskin
    reportedBy: 2, // Alfons
    title: "Slang läcker",
    description: "Vattenslangen har en liten läcka som försämrar prestandan.",
    severity: "medium",
    status: "in_repair",
    reportedAt: "2025-11-08T14:20:00Z",
    notes: "Reservdel beställd, väntar på leverans"
  },
  {
    id: "fault-3",
    machineId: 3, // Pickup med Plog
    reportedBy: 3, // Janus
    title: "Hydraulikproblem",
    description: "Hydrauliken för plogen fungerar inte som den ska. Svag kraft.",
    severity: "critical",
    status: "reported",
    reportedAt: "2025-11-12T08:15:00Z"
  },
  {
    id: "fault-4",
    machineId: 1, // Belos Transpo
    reportedBy: 1, // Tobias
    title: "Bromsar behöver service",
    description: "Bromsarna känns mjuka och behöver kontrolleras innan vinter.",
    severity: "high",
    status: "resolved",
    reportedAt: "2025-10-25T10:00:00Z",
    resolvedAt: "2025-11-02T16:30:00Z",
    notes: "Bromsarna servade och nya bromsbelägg installerade"
  },
  {
    id: "fault-5",
    machineId: 6, // Wille 455
    reportedBy: 5, // Knut Rogerson
    title: "Filter behöver bytas",
    description: "Luftfiltret är smutsigt och påverkar maskinens prestanda.",
    severity: "low",
    status: "resolved",
    reportedAt: "2025-11-01T11:45:00Z",
    resolvedAt: "2025-11-01T12:30:00Z",
    notes: "Filter bytt, maskinen fungerar normalt igen"
  },
  {
    id: "fault-6",
    machineId: 4, // Släp
    reportedBy: 1, // Tobias
    title: "Däck slitet",
    description: "Ett av däcken är kraftigt slitet och behöver bytas snart.",
    severity: "medium",
    status: "acknowledged",
    reportedAt: "2025-11-10T13:20:00Z",
    notes: "Nytt däck beställt, kommer att bytas nästa vecka"
  },
  {
    id: "fault-7",
    machineId: 2, // Nilfisk 9000
    reportedBy: 2, // Alfons
    title: "Batteri laddar inte",
    description: "Batteriet laddar inte korrekt, maskinen går ur ström snabbt.",
    severity: "high",
    status: "in_repair",
    reportedAt: "2025-11-14T09:00:00Z",
    notes: "Batteri testat, behöver nytt batteri"
  },
  {
    id: "fault-8",
    machineId: 3, // Pickup med Plog
    reportedBy: 3, // Janus
    title: "Växellåda låter",
    description: "Växellådan låter konstigt vid växling, särskilt i låga växlar.",
    severity: "medium",
    status: "reported",
    reportedAt: "2025-11-15T07:30:00Z"
  }
] as MachineFaultReport[]
