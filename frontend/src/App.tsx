import { BrowserRouter as Router, Routes, Route, Link } from "react-router-dom";
import CabanasPage from "./pages/CabanasPage";
import ReservasPage from "./pages/ReservasPage";
import "./App.css";

export default function App() {
  return (
    <Router>
      <div className="app-container">
        {/* 🧭 Sidebar lateral */}
        <aside className="sidebar">
          <h2 className="sidebar-title">Dubai RestoBar</h2>
          <nav>
            <ul>
              <li>
                <Link to="/cabanas">Cabañas</Link>
              </li>
              <li>
                <Link to="/reservas">Reservas</Link>
              </li>
            </ul>
          </nav>
        </aside>

        {/* 📄 Contenido principal */}
        <main className="content">
          <Routes>
            <Route path="/" element={<CabanasPage />} />
            <Route path="/cabanas" element={<CabanasPage />} />
            <Route path="/reservas" element={<ReservasPage />} />
          </Routes>
        </main>
      </div>
    </Router>
  );
}
