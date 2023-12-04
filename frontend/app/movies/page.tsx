'use client'
import * as React from "react";
import { DataGrid, GridColDef } from "@mui/x-data-grid";
import api from "../api";
import { getCookie } from "cookies-next";
import { useRouter } from "next/navigation";

export default function Movies() {
  let authToken = getCookie("authToken");
  let router = useRouter();
  if (!authToken) {
    router.replace("/signIn");
  }
  const [loading, setLoading] = React.useState(true);
  const [rows, setRows] = React.useState<[]>([]);
  const [rowCount, setRowCount] = React.useState(0);
  const [paginationModel, setPaginationModel] = React.useState({
    page: 0,
    pageSize: 20,
  });
  const columns: GridColDef[] = [
    { field: "id", headerName: "ID", width: 70 },
    { field: "title", headerName: "Title", width: 300 },
  ];

  React.useEffect(() => {
    api.defaults.headers.common["Authorization"] = `Bearer ${authToken}`;
    setLoading(true);
    console.log(paginationModel);
    api
      .get(`/movies/tmdb/movie/discover?page=${paginationModel.page + 1}`)
      .then((response: any) => {
        setLoading(false);
        setRows(response.data.results);
        setRowCount(response.data.total_results);
        console.log(response.data);
      }).catch((error: any) => {
          console.log(error);
          setLoading(false);
      });
  }, [authToken, paginationModel]);
  return (
    <DataGrid
      rowCount={rowCount}
      columns={columns}
      rows={rows}
      paginationMode="server"
      loading={loading}
      pageSizeOptions={[20]}
      paginationModel={paginationModel}
      onPaginationModelChange={setPaginationModel}
    >
      oi
    </DataGrid>
  );
}
