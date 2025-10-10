export type MachineJournal = {
  id: string
  machineId: string
  title: string
  body: string
  createdAt: string
  updatedAt?: string
  author: string
  tags: string[]
  category: 'maintenance' | 'repair' | 'inspection' | 'cleaning' | 'upgrade' | 'general'
  priority: 'low' | 'medium' | 'high'
  status: 'draft' | 'published' | 'archived'
  attachments?: string[]
}

export const machineJournals: MachineJournal[] = [
  {
    id: "m101",
    machineId: "1", // Belos Transpo
    title: "Månatlig service genomförd",
    body: "Genomfört månatlig service på Belos Transpo. Bytt olja, kontrollerat bromsar och däck. Alla system fungerar optimalt. Maskinen är redo för nästa månads användning.",
    createdAt: "2024-10-15T10:00:00Z",
    updatedAt: "2024-10-15T10:00:00Z",
    author: "Tobias Högberg",
    tags: ["service", "oljbyte", "bromsar", "däck", "månatlig"],
    category: "maintenance",
    priority: "medium",
    status: "published",
    attachments: ["service_rapport_belos_oktober.pdf"]
  },
  {
    id: "m102",
    machineId: "2", // Nilfisk 9000
    title: "Filterbyte och rengöring",
    body: "Bytt filter i Nilfisk 9000 och genomfört grundlig rengöring av alla komponenter. Maskinen presterar nu som ny. Rekommenderar nästa service om 3 månader.",
    createdAt: "2024-10-20T14:30:00Z",
    updatedAt: "2024-10-20T14:30:00Z",
    author: "Alfons Högberg",
    tags: ["filterbyte", "rengöring", "service", "prestanda"],
    category: "maintenance",
    priority: "high",
    status: "published",
    attachments: ["filterbyte_nilfisk_oktober.pdf"]
  },
  {
    id: "m103",
    machineId: "3", // Pickup med Plog
    title: "Vinterberedskap kontrollerad",
    body: "Kontrollerat vinterberedskap på Pickup med plog. Alla system fungerar perfekt. Plogen är justerad och redo för snöfall. Salt och skovel på plats.",
    createdAt: "2024-10-25T16:45:00Z",
    updatedAt: "2024-10-25T16:45:00Z",
    author: "Janus",
    tags: ["vinterberedskap", "plog", "kontroll", "snöfall"],
    category: "inspection",
    priority: "high",
    status: "published",
    attachments: ["vinterberedskap_kontroll_2024.pdf"]
  },
  {
    id: "m104",
    machineId: "4", // Släp
    title: "Däckbyte och bromsinspektion",
    body: "Bytt däck på släpet och genomfört bromsinspektion. Alla bromsar fungerar korrekt. Släpet är säkert att använda för tunga transporter.",
    createdAt: "2024-10-10T08:00:00Z",
    updatedAt: "2024-10-10T08:00:00Z",
    author: "Tobias Högberg",
    tags: ["däckbyte", "bromsinspektion", "säkerhet", "transport"],
    category: "repair",
    priority: "high",
    status: "published",
    attachments: ["däckbyte_släp_oktober.pdf"]
  },
  {
    id: "m105",
    machineId: "5", // Sopmaskin
    title: "Motorfel upptäckt",
    body: "Upptäckt motorfel på sopmaskinen under rutinkontroll. Motor startar inte konsekvent. Behöver teknisk service. Maskinen är tillfälligt ur drift.",
    createdAt: "2024-11-05T09:15:00Z",
    updatedAt: "2024-11-05T09:15:00Z",
    author: "Alfons Högberg",
    tags: ["motorfel", "reparation", "ur_drift", "teknisk_service"],
    category: "repair",
    priority: "high",
    status: "published",
    attachments: ["motorfel_diagnos_sopmaskin.pdf"]
  },
  {
    id: "m106",
    machineId: "6", // Wille 455
    title: "Rutinservice genomförd",
    body: "Genomfört rutinservice på Wille 455. Bytt filter, kontrollerat alla system och smort rörliga delar. Maskinen fungerar utmärkt och är redo för användning.",
    createdAt: "2024-10-10T14:00:00Z",
    updatedAt: "2024-10-10T14:00:00Z",
    author: "Johan Liljenberg",
    tags: ["rutinservice", "filter", "smörjning", "kontroll"],
    category: "maintenance",
    priority: "medium",
    status: "published",
    attachments: ["rutinservice_wille_oktober.pdf"]
  },
  {
    id: "m107",
    machineId: "1", // Belos Transpo
    title: "Kollisionsskada rapporterad",
    body: "Rapporterad mindre kollisionsskada på Belos Transpo. Skadan är kosmetisk och påverkar inte funktionaliteten. Planerar reparation nästa vecka.",
    createdAt: "2024-10-28T11:30:00Z",
    updatedAt: "2024-10-28T11:30:00Z",
    author: "Tobias Högberg",
    tags: ["kollisionsskada", "reparation", "kosmetisk", "funktionalitet"],
    category: "repair",
    priority: "medium",
    status: "published",
    attachments: ["kollisionsskada_bilder.jpg"]
  },
  {
    id: "m108",
    machineId: "2", // Nilfisk 9000
    title: "Förbättrad prestanda efter uppgradering",
    body: "Uppgraderat Nilfisk 9000 med nytt filter och förbättrad sugkraft. Prestanda har ökat med 25%. Kunder är mycket nöjda med resultatet.",
    createdAt: "2024-09-15T13:20:00Z",
    updatedAt: "2024-09-15T13:20:00Z",
    author: "Alfons Högberg",
    tags: ["uppgradering", "prestanda", "sugkraft", "kundnöjdhet"],
    category: "upgrade",
    priority: "medium",
    status: "published",
    attachments: ["uppgradering_nilfisk_september.pdf"]
  },
  {
    id: "m109",
    machineId: "3", // Pickup med Plog
    title: "Årlig säkerhetskontroll",
    body: "Genomfört årlig säkerhetskontroll på Pickup med plog. Alla säkerhetssystem fungerar korrekt. Fordonet är godkänt för användning under vintersäsongen.",
    createdAt: "2024-09-20T16:00:00Z",
    updatedAt: "2024-09-20T16:00:00Z",
    author: "Janus",
    tags: ["säkerhetskontroll", "årsvis", "vintersäsong", "godkänd"],
    category: "inspection",
    priority: "high",
    status: "published",
    attachments: ["säkerhetskontroll_pickup_2024.pdf"]
  },
  {
    id: "m110",
    machineId: "6", // Wille 455
    title: "Nytt batteri installerat",
    body: "Installerat nytt batteri i Wille 455. Batteriet håller nu laddning mycket längre och maskinen kan arbeta kontinuerligt utan avbrott. Mycket förbättrad användarupplevelse.",
    createdAt: "2024-10-05T12:30:00Z",
    updatedAt: "2024-10-05T12:30:00Z",
    author: "Johan Liljenberg",
    tags: ["batteri", "installation", "laddning", "kontinuerlig_arbete"],
    category: "upgrade",
    priority: "medium",
    status: "published",
    attachments: ["batteri_installation_wille.pdf"]
  }
]

export default {
  machine: machineJournals
}
