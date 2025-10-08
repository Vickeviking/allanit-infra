export default [
  {
    id: 1,
    level: "info",
    module: "sync",
    message: "Synkronisering slutförd - 5 kunder importerade",
    timestamp: "2025-10-08T10:30:00Z",
    entity_type: "customer",
    entity_id: 5
  },
  {
    id: 2,
    level: "warning",
    module: "email",
    message: "E-postutskick misslyckades för 1 mottagare",
    timestamp: "2025-10-08T09:15:00Z",
    entity_type: "campaign",
    entity_id: "cmp_weekly_brf"
  },
  {
    id: 3,
    level: "error",
    module: "api",
    message: "API-anrop till Seventime misslyckades",
    timestamp: "2025-10-08T08:45:00Z",
    entity_type: "sync",
    entity_id: null
  },
  {
    id: 4,
    level: "info",
    module: "order",
    message: "Ny order skapad: PO-210",
    timestamp: "2025-10-08T08:15:00Z",
    entity_type: "order",
    entity_id: 20
  },
  {
    id: 5,
    level: "success",
    module: "export",
    message: "Export till BL Administration slutförd",
    timestamp: "2025-10-07T16:30:00Z",
    entity_type: "export",
    entity_id: null
  }
]
