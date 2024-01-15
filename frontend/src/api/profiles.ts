import { AxiosResponse } from "axios";
import axiosInstance from "./axios";

export interface Profile {
  id: number;
  username: string;
  avatar: string;
  user_id: string;
  bio: string;
}

interface SimpleResponse {
  message: string;
}

export const getMyProfile = async (token: string): Promise<Profile | null> => {
  try {
    const response: AxiosResponse<Profile> = await axiosInstance.get(
      "/profiles/myprofile",
      {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      }
    );

    return response.data;
  } catch (error) {
    console.error("Error getting profile");
    return null;
  }
};

export const getProfileById = async (id: string): Promise<Profile | null> => {
  try {
    const response: AxiosResponse<Profile> = await axiosInstance.get(
      `/profiles/${id}`
    );

    return response.data;
  } catch (error) {
    console.error(`Error getting profile with ID ${id}`);
    return null;
  }
};

export const uploadImage = async (image: File, token: string): Promise<string> => {
  const form = new FormData();
  form.append("file", image);

  try {
    const imageUrl: AxiosResponse<SimpleResponse> = await axiosInstance.post(
      "/upload",
      form,
      {
        headers: {
          "Content-Type": "multipart/form-data",
          Authorization: `Bearer ${token}`,
        },
      }
    );

    return imageUrl.data.message;
  } catch (error) {
    console.error("Error during image upload:", error);
    throw new Error("Error during image upload");
  }
};

export const createProfile = async (
  username: string,
  avatar: File,
  bio: string,
  token: string
): Promise<Profile | null> => {
  try {
    const avatarUrl = await uploadImage(avatar, token);

    const profileData = {
      username,
      avatar: avatarUrl,
      bio,
    };

    const headers = {
      Authorization: `Bearer ${token}`,
    };

    const response: AxiosResponse<Profile> = await axiosInstance.post(
      "/create-profile",
      profileData,
      { headers }
    );

    return response.data;
  } catch (error) {
    console.error("Error creating profile:", error);
    return null;
  }
};

export const checkProfile = async (token: string): Promise<boolean> => {
  try {
    const response = await axiosInstance.get("/profiles/myprofile", {
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
    console.error("You don't have a profile");
    return false;
  }
};
