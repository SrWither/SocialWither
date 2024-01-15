import axios from "axios";

const apiUrl = import.meta.env.VITE_APIURL;

const axiosInstance = axios.create({
  baseURL: apiUrl,
  headers: {
    "Content-Type": "application/json",
  },
});

export default axiosInstance;
