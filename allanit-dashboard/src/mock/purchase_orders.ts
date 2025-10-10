export default [
  // Allanit Service AB Orders
  {
    id: 11,
    external_id: "PO-201",
    customer_id: 2,
    status: "not_planned",
    description: "Vinterunderhåll 24/7 beredskap – BRF Vårby Gård",
    amount: 12500,
    assigned_employee_id: null,
    supervisor_id: 1, // Tobias
    scheduled_date: null,
    completed_date: null,
    notes: "Prioritera detta uppdrag - kund väntar på svar",
    company: "allanit",
    priority: "high",
    dueDate: "2025-11-15T00:00:00Z",
    created_at: "2025-10-02T09:00:00Z",
    updated_at: "2025-10-04T12:00:00Z"
  },
  {
    id: 12,
    external_id: "PO-202",
    customer_id: 3,
    status: "planned",
    description: "Trappstädning månadsavtal – BRF Gläntan",
    amount: 6200,
    assigned_employee_id: 2, // Alfons
    supervisor_id: 1, // Tobias
    scheduled_date: "2025-11-15T08:00:00Z",
    completed_date: null,
    notes: "Månadsvis service, börja tidigt på morgonen",
    company: "allanit",
    priority: "medium",
    dueDate: "2025-11-20T00:00:00Z",
    created_at: "2025-10-03T10:00:00Z",
    updated_at: "2025-10-10T14:30:00Z"
  },
  {
    id: 13,
    external_id: "PO-203",
    customer_id: 1,
    status: "completed",
    description: "Miljörumsservice kvartalsvis – BRF Akvarellen",
    amount: 3200,
    assigned_employee_id: 2, // Alfons
    supervisor_id: 1, // Tobias
    scheduled_date: "2025-09-30T09:00:00Z",
    completed_date: "2025-09-30T16:00:00Z",
    notes: "Utmärkt arbete, kund mycket nöjd",
    company: "allanit",
    priority: "low",
    dueDate: "2025-10-05T00:00:00Z",
    created_at: "2025-09-15T08:30:00Z",
    updated_at: "2025-09-30T16:00:00Z"
  },
  {
    id: 14,
    external_id: "PO-204",
    customer_id: 4,
    status: "in_progress",
    description: "Garageunderhåll och säkerhetskontroll – P-Garage Norden",
    amount: 8500,
    assigned_employee_id: 3, // Janus
    supervisor_id: 1, // Tobias
    scheduled_date: "2025-11-08T10:00:00Z",
    completed_date: null,
    notes: "Pågående arbete, kontrollera alla säkerhetssystem",
    company: "allanit",
    priority: "high",
    dueDate: "2025-11-12T00:00:00Z",
    created_at: "2025-10-01T11:15:00Z",
    updated_at: "2025-11-08T10:00:00Z"
  },
  {
    id: 15,
    external_id: "PO-205",
    customer_id: 5,
    status: "completed",
    description: "Storstädning trapphus A-D – BRF Solbacken",
    amount: 4800,
    assigned_employee_id: 2, // Alfons
    supervisor_id: 1, // Tobias
    scheduled_date: "2025-10-06T09:20:00Z",
    completed_date: "2025-10-06T15:30:00Z",
    notes: "Arbete slutfört enligt specifikation",
    company: "allanit",
    priority: "medium",
    dueDate: "2025-10-10T00:00:00Z",
    created_at: "2025-09-28T13:45:00Z",
    updated_at: "2025-10-06T15:30:00Z"
  },
  {
    id: 16,
    external_id: "PO-206",
    customer_id: 6,
    status: "planned",
    description: "Skyddsrumsunderhåll årlig kontroll – Fastighets AB Ekholmen",
    amount: 15000,
    assigned_employee_id: 3, // Janus
    supervisor_id: 1, // Tobias
    scheduled_date: "2025-12-01T08:00:00Z",
    completed_date: null,
    notes: "Årlig säkerhetskontroll, viktigt uppdrag",
    company: "allanit",
    priority: "high",
    dueDate: "2025-12-05T00:00:00Z",
    created_at: "2025-10-07T10:00:00Z",
    updated_at: "2025-10-15T09:00:00Z"
  },
  {
    id: 17,
    external_id: "PO-207",
    customer_id: 7,
    status: "completed",
    description: "Vinterberedskap och saltning – BRF Rosenlund",
    amount: 7200,
    assigned_employee_id: 3, // Janus
    supervisor_id: 1, // Tobias
    scheduled_date: "2025-10-01T17:00:00Z",
    completed_date: "2025-10-01T17:00:00Z",
    notes: "Vinterberedskap slutförd, salt och skovel på plats",
    company: "allanit",
    priority: "medium",
    dueDate: "2025-10-05T00:00:00Z",
    created_at: "2025-09-20T09:30:00Z",
    updated_at: "2025-10-01T17:00:00Z"
  },
  {
    id: 18,
    external_id: "PO-208",
    customer_id: 8,
    status: "waiting_for_materials",
    description: "Garageportreparation och underhåll – Garage Syd AB",
    amount: 11200,
    assigned_employee_id: 3, // Janus
    supervisor_id: 1, // Tobias
    scheduled_date: null,
    completed_date: null,
    notes: "Väntar på reservdelar från leverantör",
    company: "allanit",
    priority: "medium",
    dueDate: "2025-11-25T00:00:00Z",
    created_at: "2025-10-06T14:20:00Z",
    updated_at: "2025-10-08T11:45:00Z"
  },
  {
    id: 19,
    external_id: "PO-209",
    customer_id: 2,
    status: "paused",
    description: "Extra storstädning efter renovering – BRF Vårby Gård",
    amount: 3800,
    assigned_employee_id: 2, // Alfons
    supervisor_id: 1, // Tobias
    scheduled_date: "2025-11-10T12:30:00Z",
    completed_date: null,
    notes: "Pausat på grund av väderförhållanden",
    company: "allanit",
    priority: "low",
    dueDate: "2025-11-20T00:00:00Z",
    created_at: "2025-10-05T16:00:00Z",
    updated_at: "2025-11-10T12:30:00Z"
  },
  {
    id: 20,
    external_id: "PO-210",
    customer_id: 3,
    status: "blocked",
    description: "Miljörumsservice och återvinningsstation – BRF Gläntan",
    amount: 2900,
    assigned_employee_id: 2, // Alfons
    supervisor_id: 1, // Tobias
    scheduled_date: "2025-11-20T10:00:00Z",
    completed_date: null,
    notes: "Blockerat - väntar på tillstånd från kommunen",
    company: "allanit",
    priority: "low",
    dueDate: "2025-11-30T00:00:00Z",
    created_at: "2025-10-08T08:15:00Z",
    updated_at: "2025-10-12T16:00:00Z"
  },

  // Industrimålning Stockholm AB Orders
  {
    id: 21,
    external_id: "IM-301",
    customer_id: 9,
    status: "in_review",
    description: "Industrimålning av produktionshall – Volvo AB",
    amount: 45000,
    assigned_employee_id: 5, // Knut Rogerson
    supervisor_id: 5, // Knut Rogerson (self-supervised)
    scheduled_date: "2025-11-12T07:00:00Z",
    completed_date: null,
    notes: "Under granskning av kvalitetskontroll",
    company: "industrimålning",
    priority: "high",
    dueDate: "2025-11-15T00:00:00Z",
    created_at: "2025-10-10T09:00:00Z",
    updated_at: "2025-11-12T07:00:00Z"
  },
  {
    id: 22,
    external_id: "IM-302",
    customer_id: 10,
    status: "completed",
    description: "Målning av kontorsbyggnad – Ericsson AB",
    amount: 28000,
    assigned_employee_id: 4, // Johan
    supervisor_id: 4, // Johan
    scheduled_date: "2025-10-25T08:00:00Z",
    completed_date: "2025-10-30T17:00:00Z",
    notes: "Projekt slutfört i tid, kund mycket nöjd",
    company: "industrimålning",
    created_at: "2025-09-15T10:30:00Z",
    updated_at: "2025-10-30T17:00:00Z"
  },
  {
    id: 23,
    external_id: "IM-303",
    customer_id: 11,
    status: "planned",
    description: "Skyddsmålning av fabrikshall – Scania AB",
    amount: 35000,
    assigned_employee_id: 4, // Johan
    supervisor_id: 4, // Johan
    scheduled_date: "2025-12-05T07:30:00Z",
    completed_date: null,
    notes: "Specialistarbete, kräver erfarenhet",
    company: "industrimålning",
    created_at: "2025-10-12T11:00:00Z",
    updated_at: "2025-10-20T14:00:00Z"
  },
  {
    id: 24,
    external_id: "IM-304",
    customer_id: 12,
    status: "not_planned",
    description: "Målning av lagerbyggnad – IKEA AB",
    amount: 22000,
    assigned_employee_id: null,
    supervisor_id: 4, // Johan
    scheduled_date: null,
    completed_date: null,
    notes: "Väntar på godkännande från kund",
    company: "industrimålning",
    created_at: "2025-10-15T13:45:00Z",
    updated_at: "2025-10-15T13:45:00Z"
  },
  {
    id: 25,
    external_id: "IM-305",
    customer_id: 13,
    status: "completed",
    description: "Målning av produktionslinje – Electrolux AB",
    amount: 18000,
    assigned_employee_id: 4, // Johan
    supervisor_id: 4, // Johan
    scheduled_date: "2025-10-20T08:00:00Z",
    completed_date: "2025-10-25T16:00:00Z",
    notes: "Arbete slutfört enligt tidsplan",
    company: "industrimålning",
    created_at: "2025-09-25T14:20:00Z",
    updated_at: "2025-10-25T16:00:00Z"
  },
  {
    id: 26,
    external_id: "IM-306",
    customer_id: 14,
    status: "in_progress",
    description: "Skyddsmålning av kemisk fabrik – AkzoNobel AB",
    amount: 52000,
    assigned_employee_id: 4, // Johan
    supervisor_id: 4, // Johan
    scheduled_date: "2025-11-05T06:00:00Z",
    completed_date: null,
    notes: "Kritiskt säkerhetsarbete, följ alla protokoll",
    company: "industrimålning",
    created_at: "2025-10-01T12:00:00Z",
    updated_at: "2025-11-05T06:00:00Z"
  },
  {
    id: 27,
    external_id: "IM-307",
    customer_id: 15,
    status: "planned",
    description: "Målning av kontorsbyggnad – H&M AB",
    amount: 15000,
    assigned_employee_id: 4, // Johan
    supervisor_id: 4, // Johan
    scheduled_date: "2025-11-25T09:00:00Z",
    completed_date: null,
    notes: "Kontorsmiljö, arbeta utanför arbetstid",
    company: "industrimålning",
    created_at: "2025-10-18T10:15:00Z",
    updated_at: "2025-10-25T11:30:00Z"
  },
  {
    id: 28,
    external_id: "IM-308",
    customer_id: 16,
    status: "cancelled",
    description: "Målning av lagerbyggnad – ICA AB",
    amount: 19000,
    assigned_employee_id: 4, // Johan
    supervisor_id: 4, // Johan
    scheduled_date: "2025-11-01T08:00:00Z",
    completed_date: null,
    notes: "Projekt avbrutet av kund",
    company: "industrimålning",
    created_at: "2025-09-30T15:30:00Z",
    updated_at: "2025-10-28T10:00:00Z"
  },
  {
    id: 29,
    external_id: "IM-309",
    customer_id: 17,
    status: "not_planned",
    description: "Skyddsmålning av kemisk fabrik – Perstorp AB",
    amount: 41000,
    assigned_employee_id: null,
    supervisor_id: 4, // Johan
    scheduled_date: null,
    completed_date: null,
    notes: "Komplex miljö, kräver specialutrustning",
    company: "industrimålning",
    created_at: "2025-10-22T09:45:00Z",
    updated_at: "2025-10-22T09:45:00Z"
  },
  {
    id: 30,
    external_id: "IM-310",
    customer_id: 18,
    status: "completed",
    description: "Målning av produktionshall – Atlas Copco AB",
    amount: 33000,
    assigned_employee_id: 4, // Johan
    supervisor_id: 4, // Johan
    scheduled_date: "2025-10-15T07:00:00Z",
    completed_date: "2025-10-22T18:00:00Z",
    notes: "Utmärkt kvalitet, kund rekommenderar oss",
    company: "industrimålning",
    created_at: "2025-09-20T11:20:00Z",
    updated_at: "2025-10-22T18:00:00Z"
  },

  // Additional Allanit orders for variety
  {
    id: 31,
    external_id: "PO-211",
    customer_id: 19,
    status: "planned",
    description: "Trappstädning veckovis – BRF Solen",
    amount: 1800,
    assigned_employee_id: 2, // Alfons
    supervisor_id: 1, // Tobias
    scheduled_date: "2025-11-18T08:00:00Z",
    completed_date: null,
    notes: "Veckovis service, börja tidigt",
    company: "allanit",
    created_at: "2025-10-20T12:00:00Z",
    updated_at: "2025-10-25T16:00:00Z"
  },
  {
    id: 32,
    external_id: "PO-212",
    customer_id: 20,
    status: "in_progress",
    description: "Miljörumsservice månadsvis – BRF Månen",
    amount: 2400,
    assigned_employee_id: 3, // Janus
    supervisor_id: 1, // Tobias
    scheduled_date: "2025-11-14T10:00:00Z",
    completed_date: null,
    notes: "Månadsvis service, kontrollera alla system",
    company: "allanit",
    created_at: "2025-10-25T14:30:00Z",
    updated_at: "2025-11-14T10:00:00Z"
  },
  {
    id: 33,
    external_id: "PO-213",
    customer_id: 21,
    status: "not_planned",
    description: "Garageunderhåll kvartalsvis – Garage Centrum AB",
    amount: 5600,
    assigned_employee_id: null,
    supervisor_id: 1, // Tobias
    scheduled_date: null,
    completed_date: null,
    notes: "Kvartalsvis service, planera för december",
    company: "allanit",
    created_at: "2025-10-28T11:15:00Z",
    updated_at: "2025-10-28T11:15:00Z"
  },
  {
    id: 34,
    external_id: "PO-214",
    customer_id: 22,
    status: "completed",
    description: "Vinterberedskap och saltning – BRF Stjärnan",
    amount: 4200,
    assigned_employee_id: 3, // Janus
    supervisor_id: 1, // Tobias
    scheduled_date: "2025-11-01T09:00:00Z",
    completed_date: "2025-11-01T15:00:00Z",
    notes: "Vinterberedskap slutförd, allt på plats",
    company: "allanit",
    created_at: "2025-10-30T08:00:00Z",
    updated_at: "2025-11-01T15:00:00Z"
  },
  {
    id: 35,
    external_id: "PO-215",
    customer_id: 23,
    status: "planned",
    description: "Storstädning efter renovering – BRF Kompassen",
    amount: 3200,
    assigned_employee_id: 2, // Alfons
    supervisor_id: 1, // Tobias
    scheduled_date: "2025-11-22T13:00:00Z",
    completed_date: null,
    notes: "Extra storstädning efter renovering",
    company: "allanit",
    created_at: "2025-11-01T10:30:00Z",
    updated_at: "2025-11-05T14:00:00Z"
  }
]