import { http } from "./http";

export const NotesAPI = {
  list: () => http.get("/api/notes"),
  create: (p: any) => http.post("/api/notes", p)
};
