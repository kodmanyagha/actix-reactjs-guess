import { createAsyncThunk, createSlice } from "@reduxjs/toolkit";
import { createApiInstance } from "../../hooks/useApi";
import { AuthSuccessResult, BaseApiResponse } from "../types";

export type User = {
  id: number;
  firstname: string;
  lastname: string;
  username: string;
  created_at: string;
  updated_at: string;
};

export type AuthStateType = {
  status: "not_logged_in" | "loading" | "logged_in";
  token: string | null;
  user: User | null;
};

const initialState: AuthStateType = {
  status: "not_logged_in",
  token: localStorage.getItem("authToken"),
  user: null,
};

function handleResponse(response: BaseApiResponse<AuthSuccessResult>) {
  console.log(">> Handling resp", response);

  if (typeof response.data === "object" && response.status === "success") {
    return response;
  } else {
    throw new Error(typeof response === "object" ? response.message : response);
  }
}

export const loginAction = createAsyncThunk(
  "authSlice/loginActionThunk",
  async (
    postData: object,
    {
      // dispatch,
      // getState,
      // fulfillWithValue,
      rejectWithValue,
    }
  ) => {
    const api = createApiInstance();

    const response = await api.post<BaseApiResponse<AuthSuccessResult>>(
      "/api/v1/auth/login",
      postData
    );
    try {
      return handleResponse(response.data);
    } catch (e) {
      return rejectWithValue(e);
    }
  }
);

export const registerAction = createAsyncThunk(
  "authSlice/registerActionThunk",
  async (
    postData: object,
    {
      // dispatch,
      // getState,
      // fulfillWithValue,
      rejectWithValue,
    }
  ) => {
    const api = createApiInstance();

    const response = await api.post<BaseApiResponse<AuthSuccessResult>>(
      "/api/v1/auth/register",
      postData
    );

    try {
      return handleResponse(response.data);
    } catch (e) {
      return rejectWithValue(e);
    }
  }
);

const authSlice = createSlice({
  name: "authSlice",
  initialState,
  reducers: {
    setToken: (state, action) => {
      localStorage.setItem("authToken", action.payload);

      state.token = action.payload;
    },
    setUser: (state, action) => {
      state.user = action.payload;
    },
    removeToken: (state) => {
      localStorage.removeItem("authToken");

      state.token = null;
      state.user = null;
    },
  },
  extraReducers: (builder) => {
    builder
      .addCase(registerAction.pending, (state, action) => {
        console.log("registerThunk.pending", state, action);

        state.status = "loading";
      })
      .addCase(registerAction.rejected, (state, action) => {
        console.log("registerThunk.rejected", state, action);
        localStorage.removeItem("authToken");
        state.status = "not_logged_in";
        state.user = null;
        state.token = null;
      })
      .addCase(registerAction.fulfilled, (state, action) => {
        console.log("registerThunk.fulfilled", state, action);

        if (action.payload.status === "success") {
          localStorage.setItem(
            "authToken",
            action.payload.data?.token as string
          );

          state.token = action.payload.data?.token as string;
          state.user = action.payload.data?.user as User;
          state.status = "logged_in";
        }
      })
      .addCase(loginAction.pending, (state, action) => {
        console.log("loginAction.pending", state, action);
        state.status = "loading";
      })
      .addCase(loginAction.rejected, (state, action) => {
        console.log("loginAction.rejected", state, action);
        localStorage.removeItem("authToken");
        state.status = "not_logged_in";
        state.user = null;
        state.token = null;
      })
      .addCase(loginAction.fulfilled, (state, action) => {
        console.log("loginAction.fulfilled", state, action);

        if (action.payload.status === "success") {
          localStorage.setItem(
            "authToken",
            action.payload.data?.token as string
          );

          state.token = action.payload.data?.token as string;
          state.user = action.payload.data?.user as User;
          state.status = "logged_in";
        }
      });
  },
});

export const { removeToken, setToken, setUser } = authSlice.actions;

export default authSlice.reducer;
