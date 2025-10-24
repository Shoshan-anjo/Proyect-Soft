import { useEffect, useState } from "react";
import { api } from "../api/api";
import dayjs from "dayjs";
import { toast } from "react-toastify";
import "./ReservasPage.css"; // 👈 nuevo CSS dedicado

interface Reserva {
  id: number;
  cabana_id: number;
  fecha_reserva: string;
  hora_inicio: string;
  hora_fin: string;
  estado: string;
  observaciones?: string;
}

export default function ReservasPage() {
  const [reservas, setReservas] = useState<Reserva[]>([]);
  const [nueva, setNueva] = useState({
    cabana_id: "",
    fecha_reserva: "",
    hora_inicio: "",
    hora_fin: "",
    observaciones: "",
  });

  const cargar = async () => {
    try {
      const res = await api.get<Reserva[]>("/reservas");
      setReservas(res.data);
    } catch (err) {
      console.error("❌ Error al cargar reservas:", err);
      toast.error("No se pudieron cargar las reservas del servidor.");
    }
  };

  useEffect(() => {
    void cargar();
  }, []);

  const crearReserva = async () => {
    if (
      !nueva.cabana_id ||
      !nueva.fecha_reserva ||
      !nueva.hora_inicio ||
      !nueva.hora_fin
    ) {
      toast.warning("⚠️ Por favor completa todos los campos obligatorios.");
      return;
    }

    try {
      await api.post("/reservas", {
        cliente_id: 1,
        cabana_id: parseInt(nueva.cabana_id),
        fecha_reserva: nueva.fecha_reserva,
        hora_inicio: nueva.hora_inicio,
        hora_fin: nueva.hora_fin,
        estado: "pendiente",
        observaciones: nueva.observaciones || null,
      });

      toast.success("✅ Reserva creada correctamente");

      setNueva({
        cabana_id: "",
        fecha_reserva: "",
        hora_inicio: "",
        hora_fin: "",
        observaciones: "",
      });

      await cargar();
    } catch (err: any) {
      console.error("Error al crear reserva:", err);
      let msg = "❌ No se pudo crear la reserva.";

      if (err.response) {
        const data = err.response.data;
        if (typeof data === "string" && data.includes("Conflicto")) {
          msg = "⚠️ Conflicto de horario: ya existe una reserva en ese horario.";
        } else if (typeof data === "string") {
          msg = data;
        } else if (typeof data === "object") {
          msg = data.error || data.message || JSON.stringify(data, null, 2);
        }
      }

      msg.includes("Conflicto") ? toast.warning(msg) : toast.error(msg);
    }
  };

  const eliminarReserva = async (id: number) => {
    if (!confirm(`¿Eliminar la reserva #${id}?`)) return;

    try {
      await api.delete(`/reservas/${id}`);
      toast.info(`🗑️ Reserva #${id} eliminada correctamente`);
      await cargar();
    } catch (err) {
      console.error("Error al eliminar reserva:", err);
      toast.error("❌ No se pudo eliminar la reserva");
    }
  };

  return (
    <div className="reservas-container">
      <h2 className="reservas-title">Gestión de Reservas</h2>

      <div className="form-card">
        <h3>Nueva Reserva</h3>
        <div className="form-grid">
          <input
            type="number"
            placeholder="ID Cabaña"
            value={nueva.cabana_id}
            onChange={(e) => setNueva({ ...nueva, cabana_id: e.target.value })}
          />
          <input
            type="date"
            value={nueva.fecha_reserva}
            onChange={(e) =>
              setNueva({ ...nueva, fecha_reserva: e.target.value })
            }
          />
          <input
            type="time"
            value={nueva.hora_inicio}
            onChange={(e) => setNueva({ ...nueva, hora_inicio: e.target.value })}
          />
          <input
            type="time"
            value={nueva.hora_fin}
            onChange={(e) => setNueva({ ...nueva, hora_fin: e.target.value })}
          />
          <input
            type="text"
            placeholder="Observaciones (opcional)"
            value={nueva.observaciones}
            onChange={(e) =>
              setNueva({ ...nueva, observaciones: e.target.value })
            }
          />
          <button onClick={crearReserva}>Crear Reserva</button>
        </div>
      </div>

      <div className="table-card">
        <h3>Reservas Registradas</h3>
        <table className="styled-table">
          <thead>
            <tr>
              <th>ID</th>
              <th>Cabaña</th>
              <th>Fecha</th>
              <th>Inicio</th>
              <th>Fin</th>
              <th>Estado</th>
              <th>Acción</th>
            </tr>
          </thead>
          <tbody>
            {reservas.length === 0 ? (
              <tr>
                <td colSpan={7} style={{ textAlign: "center", padding: "1rem" }}>
                  No hay reservas registradas
                </td>
              </tr>
            ) : (
              reservas.map((r) => (
                <tr key={r.id}>
                  <td>{r.id}</td>
                  <td>{r.cabana_id}</td>
                  <td>{dayjs(r.fecha_reserva).format("DD/MM/YYYY")}</td>
                  <td>{r.hora_inicio}</td>
                  <td>{r.hora_fin}</td>
                  <td>
                    <span
                      className={`estado-badge ${
                        r.estado === "pendiente"
                          ? "pendiente"
                          : r.estado === "completada"
                          ? "completada"
                          : "cancelada"
                      }`}
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
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}
