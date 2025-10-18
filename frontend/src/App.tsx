import { BrowserRouter as Router, Routes, Route, Link } from "react-router-dom";
import CabanasPage from "./pages/CabanasPage";
import ReservasPage from "./pages/ReservasPage";

export default function App() {
  return (
    <Router>
      <nav className="navbar">
        <Link to="/">🏠 Inicio</Link>
        <Link to="/cabanas">Cabañas</Link>
        <Link to="/reservas">Reservas</Link>
      </nav>

      <Routes>
        <Route path="/" element={<CabanasPage />} />
        <Route path="/cabanas" element={<CabanasPage />} />
        <Route path="/reservas" element={<ReservasPage />} />
      </Routes>
    </Router>
  );
}
