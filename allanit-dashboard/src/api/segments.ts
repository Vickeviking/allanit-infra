import { http } from "./http";

export const SegmentsAPI = {
  list: () => http.get("/api/segments")
};
