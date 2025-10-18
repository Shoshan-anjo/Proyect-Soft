import { useEffect, useState } from "react";
import { api } from "../api/api";
import dayjs from "dayjs";

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
  const [loading, setLoading] = useState(false);

  // 🧩 Cargar reservas desde la API
  const cargar = async () => {
    try {
      setLoading(true);
      const res = await api.get<Reserva[]>("/reservas");
      setReservas(res.data);
    } catch (err) {
      console.error("❌ Error al cargar reservas:", err);
    } finally {
      setLoading(false);
    }
  };

  // ✅ useEffect sin devolver promesas directamente
  useEffect(() => {
    void cargar();
  }, []);

  // 🧩 Crear nueva reserva (usando cliente_id fijo por ahora)
  const crearReserva = async () => {
    if (!nueva.cabana_id || !nueva.fecha_reserva || !nueva.hora_inicio || !nueva.hora_fin) {
      alert("⚠️ Completa todos los campos obligatorios");
      return;
    }

    try {
      await api.post("/reservas", {
        cliente_id: 1, // cliente demo hasta implementar login real
        cabana_id: parseInt(nueva.cabana_id),
        fecha_reserva: nueva.fecha_reserva,
        hora_inicio: nueva.hora_inicio,
        hora_fin: nueva.hora_fin,
        estado: "pendiente",
        observaciones: nueva.observaciones || null,
      });

      alert("✅ Reserva creada correctamente");
      setNueva({
        cabana_id: "",
        fecha_reserva: "",
        hora_inicio: "",
        hora_fin: "",
        observaciones: "",
      });
      cargar();
    } catch (err) {
      console.error("❌ Error al crear reserva:", err);
      alert("❌ No se pudo crear la reserva");
    }
  };

  // 🧩 Eliminar reserva y liberar cabaña
  const eliminarReserva = async (id: number) => {
    const confirmar = confirm("¿Seguro que deseas eliminar esta reserva?");
    if (!confirmar) return;

    try {
      await api.delete(`/reservas/${id}`);
      alert("🗑️ Reserva eliminada correctamente");
      cargar();
    } catch (err) {
      console.error("❌ Error al eliminar reserva:", err);
      alert("No se pudo eliminar la reserva");
    }
  };

  return (
    <div className="container" style={{ padding: "20px" }}>
      <h2>📅 Gestión de Reservas</h2>

      <div className="form" style={{ display: "flex", gap: "10px", marginBottom: "20px" }}>
        <input
          type="number"
          placeholder="ID Cabaña"
          value={nueva.cabana_id}
          onChange={(e) => setNueva({ ...nueva, cabana_id: e.target.value })}
        />
        <input
          type="date"
          value={nueva.fecha_reserva}
          onChange={(e) => setNueva({ ...nueva, fecha_reserva: e.target.value })}
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
          placeholder="Observaciones"
          value={nueva.observaciones}
          onChange={(e) => setNueva({ ...nueva, observaciones: e.target.value })}
        />
        <button onClick={crearReserva}>➕ Crear</button>
      </div>

      {loading ? (
        <p>Cargando reservas...</p>
      ) : (
        <table border={1} cellPadding={5}>
          <thead>
            <tr>
              <th>ID</th>
              <th>Cabaña</th>
              <th>Fecha</th>
              <th>Inicio</th>
              <th>Fin</th>
              <th>Estado</th>
              <th>Acciones</th>
            </tr>
          </thead>
          <tbody>
            {reservas.length > 0 ? (
              reservas.map((r) => (
                <tr key={r.id}>
                  <td>{r.id}</td>
                  <td>{r.cabana_id}</td>
                  <td>{dayjs(r.fecha_reserva).format("DD/MM/YYYY")}</td>
                  <td>{r.hora_inicio}</td>
                  <td>{r.hora_fin}</td>
                  <td>{r.estado}</td>
                  <td>
                    <button
                      style={{ backgroundColor: "#e74c3c", color: "white", border: "none", padding: "4px 8px", cursor: "pointer" }}
                      onClick={() => eliminarReserva(r.id)}
                    >
                      Eliminar
                    </button>
                  </td>
                </tr>
              ))
            ) : (
              <tr>
                <td colSpan={7}>No hay reservas registradas</td>
              </tr>
            )}
          </tbody>
        </table>
      )}
    </div>
  );
}
