import { AxiosResponse } from "axios";
import axiosInstance from "./axios";

export interface Auth {
  token: string | null;
}

export const authLogin = async (
  email: string,
  password: string
): Promise<Auth> => {
  const credentials = {
    email: email,
    password: password,
  };

  try {
    const response: AxiosResponse<Auth> = await axiosInstance.post(
      "/login",
      credentials
    );

    return response.data;
  } catch (error) {
    console.error("Error during login:", error);
    return {
      token: null,
    };
  }
};

export const authRegister = async (
  email: string,
  password: string
): Promise<Auth> => {
  try {
    const credentials = {
      email,
      password,
    };

    const response: AxiosResponse<Auth> = await axiosInstance.post(
      "/register",
      credentials
    );

    return response.data;
  } catch (error) {
    console.error("Error during registration:", error);
    return {
      token: null,
    };
  }
};

export const checkAuth = async (token: string | null): Promise<boolean> => {
  try {
    const response = await axiosInstance.get("/checkauth", {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });
    if (response.status === 200) {
      return true;
    } else {
      return false;
    }
  } catch (error) {
    console.error("You are not authenticated");
    return false;
  }
};
