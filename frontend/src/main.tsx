import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./index.css";

// 🧩 Importar react-toastify globalmente
import { ToastContainer } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
    {/* Contenedor global de notificaciones */}
    <ToastContainer
      position="top-right"
      autoClose={3000}
      hideProgressBar={false}
      newestOnTop={false}
      closeOnClick
      pauseOnHover
      theme="colored"
    />
  </React.StrictMode>
);
