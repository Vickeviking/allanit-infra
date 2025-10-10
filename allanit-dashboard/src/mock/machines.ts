export interface Machine {
  id: number
  name: string
  model: string
  category: 'transport' | 'cleaning' | 'winter'
  image: string
  nextServiceDate: string
  responsibleEmployeeId: number
  status: 'operational' | 'maintenance' | 'out_of_service'
  purchaseDate: string
  hours?: number
  kilometers?: number
  location: string
  description?: string
  created_at: string
  updated_at: string
}

export const machines: Machine[] = [
  {
    id: 1,
    name: "Belos Transpo",
    model: "Transpo 500",
    category: "transport",
    image: "/maskiner/belos-transpo500px.jpg",
    nextServiceDate: "2024-12-15T00:00:00Z",
    responsibleEmployeeId: 1, // Tobias Högberg
    status: "operational",
    purchaseDate: "2023-03-15T00:00:00Z",
    hours: 1250,
    location: "Huvudkontor",
    description: "Transportfordon för material och utrustning",
    created_at: "2023-03-15T00:00:00Z",
    updated_at: "2024-11-01T00:00:00Z"
  },
  {
    id: 2,
    name: "Nilfisk 9000",
    model: "910C",
    category: "cleaning",
    image: "/maskiner/Nilfisk_9000_910C.jpg",
    nextServiceDate: "2024-11-20T00:00:00Z",
    responsibleEmployeeId: 2, // Alfons Högberg
    status: "operational",
    purchaseDate: "2022-08-10T00:00:00Z",
    hours: 2100,
    location: "Huvudkontor",
    description: "Högprestanda städmaskin för stora ytor",
    created_at: "2022-08-10T00:00:00Z",
    updated_at: "2024-10-15T00:00:00Z"
  },
  {
    id: 3,
    name: "Pickup med Plog",
    model: "Ford Ranger",
    category: "winter",
    image: "/maskiner/Pickup_med_plog.jpg",
    nextServiceDate: "2024-12-01T00:00:00Z",
    responsibleEmployeeId: 3, // Janus
    status: "operational",
    purchaseDate: "2023-11-01T00:00:00Z",
    kilometers: 15000,
    location: "Huvudkontor",
    description: "Vinterberedskap med snöröjningsplog",
    created_at: "2023-11-01T00:00:00Z",
    updated_at: "2024-10-20T00:00:00Z"
  },
  {
    id: 4,
    name: "Släp",
    model: "Transport Släp 2.5t",
    category: "transport",
    image: "/maskiner/Släp.jpg",
    nextServiceDate: "2024-11-30T00:00:00Z",
    responsibleEmployeeId: 1, // Tobias Högberg
    status: "operational",
    purchaseDate: "2022-05-20T00:00:00Z",
    location: "Huvudkontor",
    description: "Släp för transport av tunga material",
    created_at: "2022-05-20T00:00:00Z",
    updated_at: "2024-09-15T00:00:00Z"
  },
  {
    id: 5,
    name: "Sopmaskin",
    model: "Professional 500",
    category: "cleaning",
    image: "/maskiner/sopmaskin-500px.jpg",
    nextServiceDate: "2024-12-10T00:00:00Z",
    responsibleEmployeeId: 2, // Alfons Högberg
    status: "maintenance",
    purchaseDate: "2021-09-12T00:00:00Z",
    hours: 3200,
    location: "Huvudkontor",
    description: "Automatisk sopmaskin för stora ytor",
    created_at: "2021-09-12T00:00:00Z",
    updated_at: "2024-11-05T00:00:00Z"
  },
  {
    id: 6,
    name: "Wille 455",
    model: "455 Professional",
    category: "cleaning",
    image: "/maskiner/wille-455-500px.jpg",
    nextServiceDate: "2024-11-25T00:00:00Z",
    responsibleEmployeeId: 4, // Johan Liljenberg
    status: "operational",
    purchaseDate: "2023-01-18T00:00:00Z",
    hours: 890,
    location: "Huvudkontor",
    description: "Kompakt städmaskin för mindre ytor",
    created_at: "2023-01-18T00:00:00Z",
    updated_at: "2024-10-10T00:00:00Z"
  }
]

export default machines
