import { NavigateFunction } from "react-router-dom";
import Swal from "sweetalert2";
import withReactContent from "sweetalert2-react-content";
import { User } from "./state/authSlice";

export type BaseApiResponse<T> = {
  status: "fail";
  message: string;
} | {
  status: "success";
  data: T;
}

export type AuthSuccessResult = {
  token: string,
  user: User
}

export type AppDataSuccessResult = {
  token: string,
  user: User
}

export const swal = withReactContent(Swal);

export const showSuccessAlert = async (body?: string, title?: string) => {
  return swal
    .fire({
      title: <strong>{title ?? "Success!"}</strong>,
      html: body,
      icon: "success",
    })
}

export const showErrorAlert = async (body?: string, title?: string) => {
  return swal
    .fire({
      title: <strong>{title ?? "Error!"}</strong>,
      html: body,
      icon: "error",
    })
}

export const handleAuthResult = async (result: BaseApiResponse<AuthSuccessResult> | unknown, navigate: NavigateFunction) => {
  if (result instanceof Error) {
    showErrorAlert(result.message)
  } else if (typeof result === "object") {
    showSuccessAlert("Operation success.")

    setTimeout(() => navigate("/user"), 2000);
  }
}
