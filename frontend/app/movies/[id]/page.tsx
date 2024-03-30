"use client";
import * as React from "react";
import api from "../../api";
import { getCookie } from "cookies-next";
import { useRouter } from "next/navigation";
import { Alert, AlertColor, Container, Grid, Snackbar } from "@mui/material";
import Link from "next/link";

interface Movie {
  id: number;
  poster_path: string;
  title: string;
}

interface Recommendation {
  recommendation_title: string;
  recommendation_movies: Movie[];
}

const MovieCard = ({ movie }: { movie: Movie }) => {
  return (
    <div className="movie-card">
      <Link href={`/movies/${movie.id}`}>
        <img src={movie.poster_path} alt={movie.title} className="poster" />
        <h3 className="title">{movie.title}</h3>
      </Link>
      <style jsx>{`
        .movie-card {
          display: inline-block;
          margin: 10px;
          text-align: center;
          width: 180px; /* Fixed width for movie card */
        }
        .poster {
          width: 150px; /* Ajuste o tamanho do pôster conforme necessário */
          height: auto;
        }
        .title {
          margin-top: 5px; /* Adjust margin for title */
          overflow: hidden;
          text-overflow: ellipsis;
          white-space: nowrap;
        }
      `}</style>
    </div>
  );
};

const Recommendation = ({
  recommendation,
}: {
  recommendation: Recommendation;
}) => {
  return (
    <div className="recommendation">
      <h2>{recommendation.recommendation_title}</h2>
      <div className="movies-list">
        {recommendation.recommendation_movies.map((movie) => (
          <MovieCard key={movie.id} movie={movie} />
        ))}
      </div>
      <style jsx>{`
        .movies-list {
          white-space: nowrap;
          overflow-x: auto;
          overflow-y: hidden;
        }
      `}</style>
    </div>
  );
};

export default function Page(props: any) {
  let authToken = getCookie("authToken");
  let router = useRouter();
  if (!authToken) {
    router.replace("/signIn");
  }

  let [openSnackBar, setOpenSnackBar] = React.useState(false);
  let [snackBarSeverity, setSnackBarSeverity] =
    React.useState<AlertColor>("success");
  let [snackBarMessage, setSnackBarMessage] = React.useState<string>("");

  const { id } = props.params;

  const [loading, setLoading] = React.useState(true);

  let [mainMovie, setMainMovie] = React.useState<Movie | null>(null);
  let [recommendations, setRecommendations] = React.useState<Recommendation[]>(
    []
  );

  console.log(mainMovie);
  console.log(recommendations);

  React.useEffect(() => {
    api.defaults.headers.common["Authorization"] = `Bearer ${authToken}`;
    setLoading(true);
    api
      .get(`/movies/tmdb/movie/discover/${id}`)
      .then((response: any) => {
        setMainMovie(response.data.movie);
        setRecommendations(response.data.recommendations);
        setLoading(false);
      })
      .catch((error: any) => {
        setSnackBarSeverity("error");
        setSnackBarMessage(error.response.data.ptBrMessage);
        setOpenSnackBar(true);
        setLoading(false);
      });
  }, [authToken, id]);

  if (loading || !mainMovie || !recommendations) {
    return (
      <Container>
        <h1>Loading...</h1>
      </Container>
    );
  }

  return (
    <Container>
      <MovieCard movie={mainMovie as Movie} />
      <div>
        {recommendations.map((recommendation, index) => (
          <Recommendation key={index} recommendation={recommendation} />
        ))}
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
