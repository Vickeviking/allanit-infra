import { http } from "./http";

export const ContactsAPI = {
  list: () => http.get("/api/contacts")
};
