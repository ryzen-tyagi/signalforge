import type { ReactNode } from "react";

type Column<T> = {
  key: keyof T;
  label: string;
  render?: (row: T) => ReactNode;
};

export function DataTable<T extends { id: string }>({
  columns,
  rows,
}: {
  columns: Array<Column<T>>;
  rows: T[];
}) {
  return (
    <table className="table">
      <thead>
        <tr>
          {columns.map((column) => (
            <th key={String(column.key)}>{column.label}</th>
          ))}
        </tr>
      </thead>
      <tbody>
        {rows.map((row) => (
          <tr key={row.id}>
            {columns.map((column) => (
              <td key={String(column.key)}>
                {column.render ? column.render(row) : String(row[column.key])}
              </td>
            ))}
          </tr>
        ))}
      </tbody>
    </table>
  );
}
