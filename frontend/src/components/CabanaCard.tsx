import type { Cabana } from "../pages/CabanasPage";

interface Props {
  cabana: Cabana;
}

export default function CabanaCard({ cabana }: Props) {
  const color =
    cabana.estado === "disponible"
      ? "border-green-500"
      : "border-red-500 opacity-75";

  return (
    <div
      className={`cabana-card border-2 ${color} rounded-lg p-4 shadow-md transition-transform hover:scale-[1.03]`}
    >
      <div className="flex justify-between items-center mb-2">
        <h3 className="text-lg font-semibold">{cabana.nombre}</h3>
        <span className="text-sm text-gray-500">#{cabana.id}</span>
      </div>

      <p>
        <b>Capacidad:</b> {cabana.capacidad} personas
      </p>

      {cabana.ubicacion && (
        <p>
          <b>Ubicación:</b> {cabana.ubicacion}
        </p>
      )}

      <p>
        <b>Estado:</b>{" "}
        {cabana.estado === "disponible" ? "🟢 Disponible" : "🔴 Ocupada"}
      </p>

      {cabana.precio_hora && (
        <p>
          <b>Precio/hora:</b> {cabana.precio_hora} Bs
        </p>
      )}

      {cabana.descripcion && (
        <p className="text-sm mt-2 text-gray-700">{cabana.descripcion}</p>
      )}
    </div>
  );
}
