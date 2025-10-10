import { http } from "./http";
import type { MachineJournal } from "@/types/domain";

export interface MachineJournalResponse {
  results: MachineJournal[];
  total: number;
  page: number;
  per_page: number;
}

export const MachineJournalsAPI = {
  list: (params?: any) => 
    http.get<MachineJournalResponse>("/api/journals/machine", { params }),
  
  get: (id: string) => 
    http.get<MachineJournal>(`/api/journals/machine/${id}`),
  
  getByMachine: (machineId: string) => 
    http.get<MachineJournalResponse>(`/api/journals/machine/${machineId}`),
  
  create: (journal: Omit<MachineJournal, 'id' | 'createdAt' | 'updatedAt'>) => 
    http.post<MachineJournal>("/api/journals/machine", journal),
  
  update: (id: string, journal: Partial<MachineJournal>) => 
    http.put<MachineJournal>(`/api/journals/machine/${id}`, journal),
  
  delete: (id: string) => 
    http.delete(`/api/journals/machine/${id}`),
};
