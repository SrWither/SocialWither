import { AxiosResponse } from "axios";
import axiosInstance from "./axios";

export interface Post {
  id: string;
  user_id: string;
  title: string;
  content: string;
  edited: boolean;
}

export const getPosts = async (): Promise<Post[]> => {
  try {
    const response: AxiosResponse<Post[]> = await axiosInstance.get("/posts");

    return response.data;
  } catch (error) {
    console.error("Error getting posts");
    return [];
  }
};

export const getPostById = async (id: string): Promise<Post | null> => {
  try {
    const response: AxiosResponse<Post> = await axiosInstance.get(
      `/posts/${id}`
    );

    return response.data;
  } catch (error) {
    console.error("Error getting post");
    return null;
  }
};

export const createPost = async (
  title: string,
  content: string,
  token: string | null
): Promise<Post | null> => {
  const postData = {
    title,
    content,
    user_id: "",
  };

  try {
    const response = await axiosInstance.post("/modifypost", postData, {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });

    return response.data;
  } catch (error) {
    console.error("Error to create post");
    return null;
  }
};

export const editPost = async (
  id: string,
  title: string,
  content: string,
  token: string | null
): Promise<Post | null> => {
  const postData = {
    title,
    content,
    user_id: "",
  };

  try {
    const response: AxiosResponse<Post> = await axiosInstance.put(
      `/modifypost/${id}`,
      postData,
      {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      }
    );

    return response.data;
  } catch (error) {
    console.error(`Error to edit post with ID ${id}`);
    return null;
  }
};

export const deletePost = async (
  id: string,
  token: string | null
): Promise<void> => {
  try {
    await axiosInstance.delete(`/modifypost/${id}`, {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });
  } catch (error) {
    console.error(`Error to delete post with ID ${id}`);
  }
};
