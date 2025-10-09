export type Subsidiary = {
  id: number
  name: string
  description: string
  employees: number
  customers: number
  active_orders: number
  monthly_revenue: number
  color: string
  established: string
  location: string
  services: string[]
  contact: {
    email: string
    phone: string
    address: string
  }
  performance: {
    customer_satisfaction: number
    completion_rate: number
    growth_rate: number
  }
  recent_activities: {
    id: string
    type: 'order' | 'customer' | 'employee' | 'maintenance'
    description: string
    date: string
  }[]
}

export const subsidiaries: Subsidiary[] = [
  {
    id: 1,
    name: "Allanit Service AB",
    description: "Trappstädning, garage/skyddsrum, miljörum, vinterunderhåll",
    employees: 3,
    customers: 5,
    active_orders: 8,
    monthly_revenue: 45000,
    color: "#3B82F6",
    established: "2020-03-15",
    location: "Stockholm, Sverige",
    services: [
      "Trappstädning",
      "Garageunderhåll", 
      "Skyddsrumsunderhåll",
      "Miljörumsservice",
      "Vinterberedskap",
      "Snöröjning",
      "Saltning"
    ],
    contact: {
      email: "info@allanit.se",
      phone: "08-123 456 78",
      address: "Storgatan 123, 123 45 Stockholm"
    },
    performance: {
      customer_satisfaction: 4.8,
      completion_rate: 96,
      growth_rate: 15
    },
    recent_activities: [
      {
        id: "act_001",
        type: "order",
        description: "Ny vinterberedskap beställning från BRF Vårby Gård",
        date: "2024-11-08T10:30:00Z"
      },
      {
        id: "act_002", 
        type: "customer",
        description: "Kundbesök hos BRF Gläntan för servicegenomgång",
        date: "2024-11-07T14:15:00Z"
      },
      {
        id: "act_003",
        type: "employee",
        description: "Maria Svensson anställd som ny städare",
        date: "2024-11-01T09:00:00Z"
      },
      {
        id: "act_004",
        type: "maintenance",
        description: "Månatligt underhåll av städutrustning genomfört",
        date: "2024-11-02T14:00:00Z"
      }
    ]
  },
  {
    id: 2, 
    name: "Industrimålning Stockholm AB",
    description: "Industrimålning, fasadrenovering, specialmålning",
    employees: 1,
    customers: 3,
    active_orders: 2,
    monthly_revenue: 25000,
    color: "#10B981",
    established: "2021-06-01",
    location: "Stockholm, Sverige",
    services: [
      "Industrimålning",
      "Fasadrenovering", 
      "Specialmålning",
      "Korrosionsskydd",
      "Ytbehandling",
      "Kvalitetskontroll"
    ],
    contact: {
      email: "info@industrimålning.se",
      phone: "08-987 654 32",
      address: "Industrivägen 45, 123 45 Stockholm"
    },
    performance: {
      customer_satisfaction: 4.6,
      completion_rate: 94,
      growth_rate: 8
    },
    recent_activities: [
      {
        id: "act_005",
        type: "order",
        description: "Industrimålning-projekt planerat för nästa vecka",
        date: "2024-11-05T09:15:00Z"
      },
      {
        id: "act_006",
        type: "customer", 
        description: "Kundbesök för offert på fasadrenovering",
        date: "2024-11-03T11:30:00Z"
      },
      {
        id: "act_007",
        type: "maintenance",
        description: "Kontroll av målningutrustning och material",
        date: "2024-11-04T08:45:00Z"
      }
    ]
  }
]

export default subsidiaries
