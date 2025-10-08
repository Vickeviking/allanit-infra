import axios from "axios";
import { http as mockHttp } from "./mockClient";

const useMock =
  import.meta.env.VITE_USE_MOCK === "1" ||
  import.meta.env.VITE_USE_MOCK === "true";

export const http = useMock
  ? mockHttp
  : axios.create({
      baseURL: import.meta.env.VITE_API_URL ?? "http://localhost:8000",
      withCredentials: true,
    });
