import axios from "axios";

export function createApiInstance() {
  const api = axios.create();

  api.defaults.baseURL = import.meta.env.VITE_API_BASE_URL;

  api.defaults.validateStatus = (status) => {
    return status >= 200 && status <= 599;
  };

  api.defaults.headers.common = {
    "content-type": "application/json; charset=UTF-8",
  };

  const authToken = localStorage.getItem("authToken");
  if (authToken) {
    api.defaults.headers.common["Authorization"] = "Bearer " + authToken;
  }

  return api;
}

export default function useApi() {
  return createApiInstance();
}
