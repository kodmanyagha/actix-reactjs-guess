import { configureStore } from "@reduxjs/toolkit";

import { useDispatch } from "react-redux";
import authReducer from "./authSlice";

const store = configureStore({
  reducer: {
    authState: authReducer,
  },
});

export type AppStateType = ReturnType<typeof store.getState>;

// https://github.com/reduxjs/redux-toolkit/issues/2450
export type AppDispatch = typeof store.dispatch;
export const useAppDispatch: () => AppDispatch = useDispatch;

export default store;
