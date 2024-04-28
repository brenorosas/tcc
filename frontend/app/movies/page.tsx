/* eslint-disable react-hooks/rules-of-hooks */
"use client";
import * as React from "react";
import api from "../api";
import { deleteCookie, getCookie } from "cookies-next";
import { useRouter } from "next/navigation";
import {
  Alert,
  AlertColor,
  Button,
  Container,
  Link,
  Snackbar,
} from "@mui/material";

export default function Movies() {
  const authToken = getCookie("authToken");
  const router = useRouter();
  const [openSnackBar, setOpenSnackBar] = React.useState(false);
  const [snackBarSeverity, setSnackBarSeverity] =
    React.useState<AlertColor>("success");
  const [snackBarMessage, setSnackBarMessage] = React.useState<string>("");
  const [loading, setLoading] = React.useState(true);
  const [movies, setMovies] = React.useState([]);
  const [totalResults, setTotalResults] = React.useState(0);
  const [paginationModel, setPaginationModel] = React.useState({
    page: 1,
    pageSize: 20,
  });

  const handleSignOut = () => {
    deleteCookie("authToken");
    router.replace("/");
  };

  const handlePaginationChange = (newPage: any) => {
    setPaginationModel((prevModel) => ({
      ...prevModel,
      page: newPage,
    }));
  };

  React.useEffect(() => {
    if (!authToken) {
      router.replace("/");
    } else {
      api.defaults.headers.common["Authorization"] = `Bearer ${authToken}`;
      setLoading(true);
      api
        .get(`/movies/tmdb/movie/discover?page=${paginationModel.page}`)
        .then((response) => {
          setLoading(false);
          setMovies(response.data.results);
          setTotalResults(response.data.total_results);
        })
        .catch((error) => {
          setSnackBarSeverity("error");
          setSnackBarMessage(error.response.data.ptBrMessage);
          setOpenSnackBar(true);
        });
    }
  }, [authToken, paginationModel]);

  const renderMovies = () => {
    const moviesPerPage = 4;
    const rows = [];
    for (let i = 0; i < movies.length; i += moviesPerPage) {
      const rowMovies = movies.slice(i, i + moviesPerPage);
      rows.push(
        <div
          key={i}
          style={{
            display: "flex",
            marginBottom: "20px",
            overflow: "hidden",
          }}
        >
          {rowMovies.map((movie: any) => (
            <div
              key={movie.id}
              style={{
                width: "100%",
              }}
            >
              <Link href={`/movies/${movie.id}`}>
                <div style={{ display: "flex", justifyContent: "center" }}>
                  <img
                    src={`https://image.tmdb.org/t/p/w500${movie.poster_path}`}
                    alt={movie.title}
                    style={{
                      width: "80%",
                      height: "auto",
                    }}
                  />
                </div>
              </Link>
            </div>
          ))}
        </div>
      );
    }
    return rows;
  };

  return (
    <Container>
      <Button
        onClick={handleSignOut}
        style={{ position: "absolute", top: 0, right: 0, margin: "10px" }}
      >
        Sign Out
      </Button>
      {renderMovies()}
      <div
        style={{
          display: "flex",
          justifyContent: "center",
          marginTop: "20px",
        }}
      >
        <Button
          disabled={paginationModel.page === 1}
          onClick={() => handlePaginationChange(paginationModel.page - 1)}
          style={{ marginRight: "10px" }}
        >
          Previous
        </Button>
        <Button
          disabled={
            paginationModel.page * paginationModel.pageSize >= totalResults
          }
          onClick={() => handlePaginationChange(paginationModel.page + 1)}
          style={{ marginLeft: "10px" }}
        >
          Next
        </Button>
      </div>
      <Snackbar
        open={openSnackBar}
        autoHideDuration={6000}
        onClose={() => setOpenSnackBar(false)}
      >
        <Alert
          onClose={() => setOpenSnackBar(false)}
          severity={snackBarSeverity}
          sx={{ width: "100%" }}
        >
          {snackBarMessage}
        </Alert>
      </Snackbar>
    </Container>
  );
}
