import { useEffect, useState } from "react";
import { api } from "../api/api";
import CabanaCard from "../components/CabanaCard";

export interface Cabana {
  id: number;
  nombre: string;
  capacidad: number;
  ubicacion?: string;
  estado: string;
  descripcion?: string;
  precio_hora?: number;
}

export default function CabanasPage() {
  const [cabanas, setCabanas] = useState<Cabana[]>([]);

  // 🔄 Cargar y ordenar por ID
  const cargar = async () => {
    try {
      const res = await api.get<Cabana[]>("/cabanas");
      const ordenadas = res.data.sort((a, b) => a.id - b.id);
      setCabanas(ordenadas);
    } catch (err) {
      console.error("Error al cargar cabañas:", err);
    }
  };

  useEffect(() => {
    void cargar();
  }, []);

  return (
  <div className="container">
    <h2 className="titulo">🏠 Cabañas disponibles</h2>

    <div className="cabanas-grid">
      {cabanas.length > 0 ? (
        cabanas.map((c) => <CabanaCard key={c.id} cabana={c} />)
      ) : (
        <p>No hay cabañas registradas.</p>
      )}
    </div>
  </div>
);

}
