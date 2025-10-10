import { http } from "./http";
import type { Machine } from "@/types/domain";

export interface MachineResponse {
  results: Machine[];
  total: number;
  page: number;
  per_page: number;
}

export const MachinesAPI = {
  list: (params?: any) => 
    http.get<MachineResponse>("/api/machines", { params }),
  
  get: (id: number) => 
    http.get<Machine>(`/api/machines/${id}`),
  
  create: (machine: Omit<Machine, 'id' | 'created_at' | 'updated_at'>) => 
    http.post<Machine>("/api/machines", machine),
  
  update: (id: number, machine: Partial<Machine>) => 
    http.put<Machine>(`/api/machines/${id}`, machine),
  
  delete: (id: number) => 
    http.delete(`/api/machines/${id}`),
};
