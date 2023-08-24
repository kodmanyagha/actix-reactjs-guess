import { createAsyncThunk, createSlice } from "@reduxjs/toolkit";
import { createApiInstance } from "../../hooks/useApi";
import {
  AppDataSuccessResult,
  AuthSuccessResult,
  BaseApiResponse,
} from "../types";

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

export const getAppDataAction = createAsyncThunk(
  "authSlice/getAppDataActionThunk",
  async (
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    _ = undefined,
    {
      // dispatch,
      // getState,
      fulfillWithValue,
      rejectWithValue,
    }
  ) => {
    const api = createApiInstance();

    const response = await api.get<BaseApiResponse<AppDataSuccessResult>>(
      "/api/v1/appData"
    );

    if (response.data.status === "success") {
      return fulfillWithValue(response.data.data);
    } else {
      return rejectWithValue(response.data.message);
    }
  }
);

export const loginAction = createAsyncThunk(
  "authSlice/loginActionThunk",
  async (
    postData: object,
    {
      // dispatch,
      // getState,
      fulfillWithValue,
      rejectWithValue,
    }
  ) => {
    const api = createApiInstance();

    const response = await api.post<BaseApiResponse<AuthSuccessResult>>(
      "/api/v1/auth/login",
      postData
    );
    if (response.data.status === "success") {
      return fulfillWithValue(response.data.data);
    } else {
      return rejectWithValue(response.data.message);
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
      fulfillWithValue,
      rejectWithValue,
    }
  ) => {
    const api = createApiInstance();

    const response = await api.post<BaseApiResponse<AuthSuccessResult>>(
      "/api/v1/auth/register",
      postData
    );

    if (response.data.status === "success") {
      return fulfillWithValue(response.data.data);
    } else {
      return rejectWithValue(response.data.message);
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
      .addCase(getAppDataAction.pending, (state, action) => {
        console.log("getAppDataAction.pending", state, action);
      })
      .addCase(getAppDataAction.rejected, (state, action) => {
        console.log("getAppDataAction.rejected", state, action);
      })
      .addCase(getAppDataAction.fulfilled, (state, action) => {
        console.log("getAppDataAction.fulfilled", state, action);
        state.token = action.payload?.token;
        state.user = action.payload?.user;
      })
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

        localStorage.setItem("authToken", action.payload.token as string);

        state.token = action.payload.token as string;
        state.user = action.payload.user as User;
        state.status = "logged_in";
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

        localStorage.setItem("authToken", action.payload.token as string);

        state.token = action.payload.token as string;
        state.user = action.payload.user as User;
        state.status = "logged_in";
      });
  },
});

export const { removeToken, setToken, setUser } = authSlice.actions;

export default authSlice.reducer;
