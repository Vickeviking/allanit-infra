import { http } from "./http";

export const CampaignsAPI = {
  list: () => http.get("/api/campaigns"),
  sends: () => http.get("/api/campaigns/sends"),
  create: (payload: any) => http.post("/api/campaigns", payload)
};
