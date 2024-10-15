
import ReactDOM from "react-dom/client";
import App from "./App";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import ErrorPage from "@/pages/error-page";
import "./index.css";
import { lazy } from "react";

const router = createBrowserRouter([
  {
    path: "/",
    Component: App,
    errorElement: <ErrorPage />,
    children: [
      {
        path: "/",
        Component: lazy(() => import("@/pages/list")),
      },
      {
        path: "settings",
        Component: lazy(() => import("@/pages/settings")),
      },
    ],

  },
]);
ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <RouterProvider router={router} />
);
