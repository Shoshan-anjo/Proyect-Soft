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

    const base = api.defaults.baseURL ?? "";
    const wsUrl = `${base}/ws`;

    let eventSource = new EventSource(wsUrl);
    console.log("🌐 Conectado a SSE:", wsUrl);

    eventSource.onmessage = (event) => {
      if (event.data === "actualizar") {
        console.log("🔁 Recibido evento 'actualizar' → recargando cabañas");
        cargar();
      }
    };

    eventSource.onerror = (err) => {
      console.warn("⚠️ Conexión SSE perdida, reconectando...", err);
      eventSource.close();
      setTimeout(() => {
        eventSource = new EventSource(wsUrl);
      }, 2000);
    };

    return () => eventSource.close();
  }, []);

  return (
    <div className="container">
      <h2 className="titulo">Cabañas en tiempo real 🏠</h2>
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
