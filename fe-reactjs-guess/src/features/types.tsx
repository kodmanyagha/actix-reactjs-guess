import { NavigateFunction } from "react-router-dom";
import Swal, { SweetAlertResult } from "sweetalert2";
import withReactContent from "sweetalert2-react-content";
import { User } from "./state/authSlice";

export type BaseApiResponse<T> = {
  status: "fail" | "success";
  data?: T;
  message?: string;
};

export type AuthSuccessResult = {
  token: string,
  user: User
}

export const swal = withReactContent(Swal);

export const handleAuthResult = (result: BaseApiResponse<AuthSuccessResult> | unknown, navigate: NavigateFunction) => {
  console.log(">> result", result)

  if (result instanceof Error) {
    swal
      .fire({
        title: <p>{result.message}</p>,
        icon: "info",
      })
      .then((result: SweetAlertResult) => {
        console.log(">>>  result:", result);

        if (result.isConfirmed) {
          console.log("Confirmed.");
        } else {
          console.log("Not confirmed.");
        }
      });

  } else if (typeof result === "object") {
    swal
      .fire({
        title: <p>Login success, redirecting.</p>,
        icon: "success",
      })
      .then((result: SweetAlertResult) => {
        console.log(">>>  result:", result);

        if (result.isConfirmed) {
          console.log("Confirmed.");
        } else {
          console.log("Not confirmed.");
        }
      });

    setTimeout(() => navigate("/user"), 2000);
  }
};
