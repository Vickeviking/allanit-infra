export type Journal = {
  id: string
  employeeId: string
  title: string
  body: string
  createdAt: string
  updatedAt?: string
  author: string
  tags: string[]
  category: 'work' | 'training' | 'maintenance' | 'customer' | 'safety' | 'equipment' | 'general'
  priority: 'low' | 'medium' | 'high'
  status: 'draft' | 'published' | 'archived'
  attachments?: string[]
}

export const journals: Journal[] = [
  {
    id: "101",
    employeeId: "1", // Tobias Högberg
    title: "Ledarskapsutbildning genomförd",
    body: "Genomfört ledarskapsutbildning med fokus på digitalisering och moderna arbetsmetoder. Kursen gav värdefulla insikter om teamledning och kommunikation. Planerar att implementera nya arbetsmetoder från nästa månad.",
    createdAt: "2025-09-20T10:00:00Z",
    updatedAt: "2025-09-20T10:00:00Z",
    author: "Tobias Högberg",
    tags: ["utbildning", "ledarskap", "digitalisering", "teamledning"],
    category: "training",
    priority: "medium",
    status: "published",
    attachments: ["certifikat_ledarskap.pdf"]
  },
  {
    id: "102",
    employeeId: "2", // Alfons Högberg
    title: "Ny utrustning levererad",
    body: "Nytt städkit uthämtat, inklusive miljövänliga rengöringsmedel och förbättrade verktyg för effektivare arbete. Utrustningen kommer att förbättra vår arbetsmiljö och effektivitet avsevärt.",
    createdAt: "2025-10-01T14:30:00Z",
    updatedAt: "2025-10-01T14:30:00Z",
    author: "Alfons Högberg",
    tags: ["utrustning", "miljövänlig", "effektivitet", "städkit"],
    category: "equipment",
    priority: "high",
    status: "published",
    attachments: ["leveransnota_utrustning.pdf", "bilder_ny_utrustning.jpg"]
  },
  {
    id: "103",
    employeeId: "3", // Janus
    title: "Utmärkt feedback från kund",
    body: "Utmärkt arbete med vinterberedskap på BRF Vårby Gård. Kunden var mycket nöjd med den professionella service och snabba respons. Fick även förslag på framtida samarbeten.",
    createdAt: "2025-10-08T16:45:00Z",
    updatedAt: "2025-10-08T16:45:00Z",
    author: "Janus",
    tags: ["kundnöjdhet", "vinterberedskap", "feedback", "BRF Vårby Gård"],
    category: "customer",
    priority: "medium",
    status: "published",
    attachments: ["kundfeedback_vinterberedskap.pdf"]
  },
  {
    id: "104",
    employeeId: "4", // Johan Liljenberg
    title: "Sjukfrånvaro",
    body: "Sjukfrånvaro p.g.a. magsjuka, beräknas vara hemma och skita i 3 veckor. Skickat krya på dig-kort. Tobias tar över mina uppgifter under frånvaron.",
    createdAt: "2025-10-09T08:00:00Z",
    updatedAt: "2025-10-09T08:00:00Z",
    author: "Johan Liljenberg",
    tags: ["sjukfrånvaro", "magsjuka", "uppgiftsöverlämning"],
    category: "general",
    priority: "high",
    status: "published"
  },
  {
    id: "105",
    employeeId: "4", // Johan Liljenberg
    title: "Extra arbetspass planerat",
    body: "Planerat extra arbetspass för Industrimålning-projektet nästa vecka. Koordinerat med kund för optimal tidsplanering. Behöver ytterligare material för projektet.",
    createdAt: "2025-10-05T09:15:00Z",
    updatedAt: "2025-10-05T09:15:00Z",
    author: "Johan Liljenberg",
    tags: ["industrimålning", "projekt", "tidsplanering", "material"],
    category: "work",
    priority: "medium",
    status: "published",
    attachments: ["projektplan_industrimålning.pdf"]
  },
  {
    id: "106",
    employeeId: "1", // Tobias Högberg
    title: "Månadsrapport oktober",
    body: "Genomgång av månadens resultat och planering för november. Fokus på vinterberedskap och kundnöjdhet. Ökad omsättning med 15% jämfört med föregående månad.",
    createdAt: "2025-10-31T15:00:00Z",
    updatedAt: "2025-10-31T15:00:00Z",
    author: "Tobias Högberg",
    tags: ["månadsrapport", "oktober", "vinterberedskap", "kundnöjdhet", "omsättning"],
    category: "work",
    priority: "high",
    status: "published",
    attachments: ["månadsrapport_oktober_2024.pdf", "statistik_oktober.xlsx"]
  },
  {
    id: "107",
    employeeId: "2", // Alfons Högberg
    title: "Säkerhetsgenomgång",
    body: "Genomfört säkerhetsgenomgång av alla arbetsområden. Uppdaterat säkerhetsprotokoll och utbildat teamet. Alla säkerhetsutrustningar kontrollerade och fungerar korrekt.",
    createdAt: "2025-10-15T11:30:00Z",
    updatedAt: "2025-10-15T11:30:00Z",
    author: "Alfons Högberg",
    tags: ["säkerhet", "genomgång", "protokoll", "utbildning", "kontroll"],
    category: "safety",
    priority: "high",
    status: "published",
    attachments: ["säkerhetsprotokoll_2024.pdf", "kontrollista_säkerhet.pdf"]
  },
  {
    id: "108",
    employeeId: "3", // Janus
    title: "Kundbesök BRF Gläntan",
    body: "Besökt BRF Gläntan för genomgång av vinterberedskap. Diskuterat förbättringar och kommande arbeten. Kunden vill ha månatlig service istället för kvartalsvis.",
    createdAt: "2025-10-12T13:20:00Z",
    updatedAt: "2025-10-12T13:20:00Z",
    author: "Janus",
    tags: ["kundbesök", "BRF Gläntan", "vinterberedskap", "förbättringar", "service"],
    category: "customer",
    priority: "medium",
    status: "published",
    attachments: ["kundbesök_rapport_BRF_Gläntan.pdf"]
  },
  {
    id: "109",
    employeeId: "1", // Tobias Högberg
    title: "Ny medarbetare anställd",
    body: "Välkommen till Maria Svensson som ny städare! Hon börjar nästa måndag och kommer att arbeta med trappstädning och miljörumsservice. Utbildning planerad för första veckan.",
    createdAt: "2025-11-01T09:00:00Z",
    updatedAt: "2025-11-01T09:00:00Z",
    author: "Tobias Högberg",
    tags: ["anställning", "Maria Svensson", "utbildning", "trappstädning"],
    category: "work",
    priority: "high",
    status: "published",
    attachments: ["anställningskontrakt_Maria_Svensson.pdf"]
  },
  {
    id: "110",
    employeeId: "2", // Alfons Högberg
    title: "Underhåll av städutrustning",
    body: "Genomfört månatligt underhåll av alla städmaskiner och verktyg. Bytt filter i dammsugare och oljat alla rörliga delar. All utrustning fungerar optimalt.",
    createdAt: "2025-11-02T14:00:00Z",
    updatedAt: "2025-11-02T14:00:00Z",
    author: "Alfons Högberg",
    tags: ["underhåll", "städmaskiner", "filter", "oljning", "kontroll"],
    category: "maintenance",
    priority: "medium",
    status: "published",
    attachments: ["underhållsrapport_städutrustning.pdf"]
  },
  {
    id: "111",
    employeeId: "5", // Knut Rogerson
    title: "Gult kort för fortkörning",
    body: "Fick gult kort för fortkörning på väg till jobbet. Polisen sa att jag körde 85 km/h i 50-zon. Överväger om jag får jobba kvar eller om jag ska sparkas på snäckt sätt. Chef Tobias sa att det är första varningen.",
    createdAt: "2025-11-01T08:30:00Z",
    updatedAt: "2025-11-01T08:30:00Z",
    author: "Knut Rogerson",
    tags: ["trafik", "varning", "fortkörning", "gult kort"],
    category: "safety",
    priority: "high",
    status: "published",
    attachments: ["gult_kort_bot.pdf"]
  },
  {
    id: "112",
    employeeId: "6", // Niklas Danielsson
    title: "Dött i plogolycka - ta bort användarprofil",
    body: "RIP Niklas Danielsson. Omkom i tragisk plogolycka när han körde snöröjningsmaskinen. Han var en av våra bästa chefer på Industrimålning. Begravning nästa vecka. HR ska ta bort hans användarprofil från systemet.",
    createdAt: "2025-11-05T14:00:00Z",
    updatedAt: "2025-11-05T14:00:00Z",
    author: "Tobias Högberg",
    tags: ["olycka", "död", "plog", "begravning", "hr"],
    category: "general",
    priority: "critical",
    status: "published",
    attachments: ["dödsattest_niklas.pdf", "olycksrapport.pdf"]
  },
  {
    id: "113",
    employeeId: "7", // Zamdall Gröndal
    title: "Målade fel färg på hela fabriken",
    body: "Oj, målade hela fabriken i rosa istället för grå som kunden ville ha. Chef Knut sa att det är 'kreativt' men kunden är inte så glad. Får nog måla om hela skiten. Detta kommer att kosta oss en förmögenhet.",
    createdAt: "2025-11-08T16:20:00Z",
    updatedAt: "2025-11-08T16:20:00Z",
    author: "Zamdall Gröndal",
    tags: ["fel färg", "rosa", "fabrik", "kostnad", "omålning"],
    category: "work",
    priority: "high",
    status: "published",
    attachments: ["bilder_rosa_fabrik.jpg", "kundklagomål.pdf"]
  },
  {
    id: "114",
    employeeId: "8", // Åke Jäger
    title: "Skyddsmålning blev för skyddad",
    body: "Målade skyddsfärg så tjockt att dörrarna inte går att öppna längre. Kunden sa att det är 'överdrivet säkert' men nu kan ingen komma in eller ut från byggnaden. Behöver skrapa bort halva målningslagret.",
    createdAt: "2025-11-10T11:45:00Z",
    updatedAt: "2025-11-10T11:45:00Z",
    author: "Åke Jäger",
    tags: ["skyddsmålning", "tjockt", "dörrar", "fast", "skrapning"],
    category: "work",
    priority: "high",
    status: "published",
    attachments: ["bilder_fasta_dörrar.jpg", "reparationskostnad.pdf"]
  },
  {
    id: "115",
    employeeId: "5", // Knut Rogerson
    title: "Fortsatt problem med trafik",
    body: "Fick ännu ett gult kort, denna gång för att köra på rött ljus. Chef Tobias sa att det är andra varningen och att jag måste sluta köra som en galning. Överväger att ta bussen till jobbet istället.",
    createdAt: "2025-11-12T09:15:00Z",
    updatedAt: "2025-11-12T09:15:00Z",
    author: "Knut Rogerson",
    tags: ["trafik", "rött ljus", "andra varningen", "buss"],
    category: "safety",
    priority: "high",
    status: "published",
    attachments: ["gult_kort_rött_ljus.pdf"]
  }
]

export default {
  employee: [],
  site: [
    {
      id: 201,
      customer_id: 4,
      location: "Garageplan -2",
      type: "issue",
      note: "Oljeutsläpp, åtgärdat med absorberingsmedel.",
      created_at: "2025-10-05T11:10:00Z"
    },
    {
      id: 202,
      customer_id: 1,
      location: "Miljörum",
      type: "reminder",
      note: "Sätt upp nya skyltar för kartongvikning.",
      created_at: "2025-10-08T08:40:00Z"
    },
    {
      id: 203,
      customer_id: 2,
      location: "Trapphus A",
      type: "maintenance",
      note: "Regelbunden kontroll av hiss och brandsläckare.",
      created_at: "2025-10-07T13:20:00Z"
    },
    {
      id: 204,
      customer_id: 6,
      location: "Skyddsrum",
      type: "inspection",
      note: "Årlig säkerhetskontroll genomförd - allt OK.",
      created_at: "2025-10-06T16:00:00Z"
    },
    {
      id: 205,
      customer_id: 3,
      location: "Miljörum",
      type: "improvement",
      note: "Förbättrad ventilation efter klagomål från hyresgäster.",
      created_at: "2025-10-05T12:30:00Z"
    },
    {
      id: 206,
      customer_id: 5,
      location: "Trapphus B",
      type: "cleaning",
      note: "Extra storstädning efter renovering av lägenheter.",
      created_at: "2025-10-04T15:45:00Z"
    },
    {
      id: 207,
      customer_id: 7,
      location: "Utomhus",
      type: "winter",
      note: "Vinterberedskap - salt och skovel på plats.",
      created_at: "2025-10-03T11:00:00Z"
    },
    {
      id: 208,
      customer_id: 8,
      location: "Garageport",
      type: "repair",
      note: "Garageportreparation - elektronik kontrollerad.",
      created_at: "2025-10-02T14:15:00Z"
    }
  ]
}
