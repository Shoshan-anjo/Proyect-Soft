import { useEffect, useState } from "react";
import { api } from "../api/api";
import "./ReservasPage.css";

interface Reserva {
  id: number;
  cliente_id: number;
  cabana_id: number;
  fecha_reserva: string;
  hora_inicio: string;
  hora_fin: string;
  estado: string;
  observaciones?: string;
}

interface NuevaReserva {
  cliente_id: number;
  cabana_id: number;
  fecha_reserva: string;
  hora_inicio: string;
  hora_fin: string;
  observaciones?: string;
}

export default function ReservasPage() {
  const [reservas, setReservas] = useState<Reserva[]>([]);
  const [nuevaReserva, setNuevaReserva] = useState<NuevaReserva>({
    cliente_id: 1,
    cabana_id: 1,
    fecha_reserva: "",
    hora_inicio: "",
    hora_fin: "",
    observaciones: "",
  });
  const [loading, setLoading] = useState(true);
  const [ultimaActualizacion, setUltimaActualizacion] = useState<string>("");

  // 🔁 Cargar reservas desde backend
  const cargar = async () => {
    try {
      const res = await api.get<Reserva[]>("/reservas");
      setReservas(res.data);
      setLoading(false);
      setUltimaActualizacion(new Date().toLocaleTimeString());
    } catch (err) {
      console.error("❌ Error al cargar reservas:", err);
    }
  };

  // ➕ Crear nueva reserva
  const crearReserva = async () => {
    try {
      await api.post("/reservas", nuevaReserva);
      setNuevaReserva({
        cliente_id: 1,
        cabana_id: 1,
        fecha_reserva: "",
        hora_inicio: "",
        hora_fin: "",
        observaciones: "",
      });
      await cargar(); // refresca lista sin esperar SSE
    } catch (err: any) {
      console.error("❌ Error al crear reserva:", err.response?.data || err);
      alert(err.response?.data?.error || "Error al crear reserva");
    }
  };

  // 🗑️ Eliminar reserva
  const eliminarReserva = async (id: number) => {
    if (!window.confirm("¿Seguro que deseas eliminar esta reserva?")) return;

    try {
      await api.delete(`/reservas/${id}`);
      setReservas((prev) => prev.filter((r) => r.id !== id));
    } catch (err) {
      console.error("❌ Error al eliminar reserva:", err);
      alert("Error al eliminar la reserva");
    }
  };

  // 🔔 Suscribirse a eventos SSE (tiempo real)
  useEffect(() => {
    void cargar();

    const base = api.defaults.baseURL ?? "http://127.0.0.1:8000";
    const wsUrl = `${base}/ws`;

    let eventSource = new EventSource(wsUrl);
    console.log("🌐 Conectado a SSE:", wsUrl);

    eventSource.onmessage = (event) => {
      if (event.data === "actualizar") {
        console.log("🔁 Evento recibido: recargando reservas...");
        cargar();
      }
    };

    eventSource.onerror = (err) => {
      console.warn("⚠️ SSE desconectado, reconectando...", err);
      eventSource.close();
      setTimeout(() => {
        eventSource = new EventSource(wsUrl);
      }, 2000);
    };

    return () => eventSource.close();
  }, []);

  return (
    <div className="reservas-container">
      <h2 className="reservas-title">📅 Reservas en Tiempo Real</h2>

      {/* === 🧾 FORMULARIO === */}
      <div className="form-card">
        <h3>Nueva Reserva</h3>
        <div className="form-grid">
          <input
            type="number"
            placeholder="ID Cliente"
            value={nuevaReserva.cliente_id}
            onChange={(e) =>
              setNuevaReserva({
                ...nuevaReserva,
                cliente_id: parseInt(e.target.value),
              })
            }
          />
          <input
            type="number"
            placeholder="ID Cabaña"
            value={nuevaReserva.cabana_id}
            onChange={(e) =>
              setNuevaReserva({
                ...nuevaReserva,
                cabana_id: parseInt(e.target.value),
              })
            }
          />
          <input
            type="date"
            value={nuevaReserva.fecha_reserva}
            onChange={(e) =>
              setNuevaReserva({
                ...nuevaReserva,
                fecha_reserva: e.target.value,
              })
            }
          />
          <input
            type="time"
            value={nuevaReserva.hora_inicio}
            onChange={(e) =>
              setNuevaReserva({
                ...nuevaReserva,
                hora_inicio: e.target.value,
              })
            }
          />
          <input
            type="time"
            value={nuevaReserva.hora_fin}
            onChange={(e) =>
              setNuevaReserva({
                ...nuevaReserva,
                hora_fin: e.target.value,
              })
            }
          />
          <input
            type="text"
            placeholder="Observaciones (opcional)"
            value={nuevaReserva.observaciones}
            onChange={(e) =>
              setNuevaReserva({
                ...nuevaReserva,
                observaciones: e.target.value,
              })
            }
          />
          <button onClick={crearReserva}>Agregar Reserva</button>
        </div>
      </div>

      {/* === 📋 TABLA === */}
      <div className="table-card">
        <h3>Listado de Reservas</h3>
        <p style={{ fontSize: "0.9rem", color: "#555" }}>
          Última actualización: {ultimaActualizacion || "—"}
        </p>

        {loading ? (
          <p>Cargando reservas...</p>
        ) : (
          <table className="styled-table">
            <thead>
              <tr>
                <th>ID</th>
                <th>Cliente</th>
                <th>Cabaña</th>
                <th>Fecha</th>
                <th>Hora Inicio</th>
                <th>Hora Fin</th>
                <th>Estado</th>
                <th>Acción</th>
              </tr>
            </thead>
            <tbody>
              {reservas.length > 0 ? (
                reservas.map((r) => (
                  <tr key={r.id}>
                    <td>{r.id}</td>
                    <td>{r.cliente_id}</td>
                    <td>{r.cabana_id}</td>
                    <td>
                      {new Date(r.fecha_reserva).toLocaleDateString("es-BO")}
                    </td>
                    <td>{r.hora_inicio}</td>
                    <td>{r.hora_fin}</td>
                    <td>
                      <span
                        className={`estado-badge ${r.estado
                          .toLowerCase()
                          .replace(/\s+/g, "")}`}
                      >
                        {r.estado}
                      </span>
                    </td>
                    <td>
                      <button
                        className="delete-btn"
                        onClick={() => eliminarReserva(r.id)}
                      >
                        Eliminar
                      </button>
                    </td>
                  </tr>
                ))
              ) : (
                <tr>
                  <td colSpan={8}>No hay reservas registradas.</td>
                </tr>
              )}
            </tbody>
          </table>
        )}
      </div>
    </div>
  );
}
