import { ReactNode, createContext, useState } from "react";

export type ThemeContextProviderProps = {
  children: ReactNode
}

export type ThemeContextValueType = {
  theme: string,
  setTheme: (p: string) => void,
} | undefined

export const ThemeContext = createContext<ThemeContextValueType>(undefined);

export default function ThemeContextProvider(props: ThemeContextProviderProps) {
  const selectedTheme = localStorage.getItem("selectedTheme");
  const [theme, setTheme] = useState(selectedTheme ?? "light");

  localStorage.setItem("selectedTheme", theme);

  const htmlTag = document.getElementsByTagName("html")[0];
  htmlTag.setAttribute("data-bs-theme", theme);

  const contextValue = {
    theme,
    setTheme,
  };

  return (
    <ThemeContext.Provider value={contextValue}>
      {props.children}
    </ThemeContext.Provider>
  );
}
