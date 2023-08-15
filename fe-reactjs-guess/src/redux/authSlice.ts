import { createAsyncThunk, createSlice } from "@reduxjs/toolkit";
import { createApiInstance } from "../hooks/useApi";

export type User = {
  id: number;
  firstname: string;
  lastname: string;
  fullname: string;
  username: string;
};

export type AuthStateType = {
  token: string | null;
  user: User | null;
};

const initialState: AuthStateType = {
  token: localStorage.getItem("authToken"),
  user: null,
};

export const registerAction = createAsyncThunk(
  "authSlice/registerThunk",
  //async (postData: object, thunkAPI) => {
  async (postData: object) => {
    // console.log(">>>  thunkAPI:", thunkAPI);

    const api = createApiInstance();

    const response = await api.post("/api/v1/auth/register", postData);
    console.log(">>>  response:", response);

    return response.data;
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
  extraReducers(builder) {
    builder
      .addCase(registerAction.pending, (state, action) => {
        console.log("registerThunk.pending", state, action);
      })
      .addCase(registerAction.rejected, (state, action) => {
        console.log("registerThunk.rejected", state, action);
      })
      .addCase(registerAction.fulfilled, (state, action) => {
        console.log("registerThunk.fulfilled", state, action);
      });
  },
});

export const { removeToken, setToken, setUser } = authSlice.actions;

export default authSlice.reducer;
