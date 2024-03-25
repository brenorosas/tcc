"use client";
import * as React from "react";
import { GridColDef } from "@mui/x-data-grid";
import api from "../../api";
import { getCookie } from "cookies-next";
import { useSearchParams, useRouter } from "next/navigation";
import { Grid } from "@mui/material";

export default function Page(props: any) {
  let authToken = getCookie("authToken");
  let router = useRouter();
  if (!authToken) {
    router.replace("/signIn");
  }

  const searchParams = useSearchParams();

  const { id } = props.params;

  const [loading, setLoading] = React.useState(true);
  console.log("here", id);

  const columns: GridColDef[] = [
    { field: "id", headerName: "ID", width: 70 },
    { field: "title", headerName: "Title", width: 300 },
  ];

  React.useEffect(() => {
    api.defaults.headers.common["Authorization"] = `Bearer ${authToken}`;
    setLoading(true);
    api
      .get(`/movies/tmdb/movie/discover/${id}`)
      .then((response: any) => {
        console.log(response.data);
      })
      .catch((error: any) => {
        console.log(error);
        setLoading(false);
      });
  }, [authToken, id]);
  return <Grid></Grid>;
}
