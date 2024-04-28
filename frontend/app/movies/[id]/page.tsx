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
  overview: string;
}

interface Recommendation {
  recommendation_title: string;
  recommendation_movies: Movie[];
  recommendation_type: string;
}

const MovieCard = ({ movie, recommendation }: { movie: Movie, recommendation: Recommendation }) => {
  return (
    <div className="movie-card">
      <Link href={`/movies/${movie.id}`} onClick={() => {
        api.post(`/users/register-choice`, {recommendation_type: recommendation.recommendation_type});
      }}>
        <img src={movie.poster_path} alt={movie.title} className="poster" />
        {/* <h3 className="title">{movie.title}</h3> */}
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

const MainMovieCard = ({ movie }: { movie: Movie }) => {
  return (
    <div className="main-movie-card">
      <div className="content">
        <img src={movie.poster_path} alt={movie.title} className="poster" />
        <div className="details">
          <h1 className="title">{movie.title}</h1>
          <p className="overview">{movie.overview}</p>
        </div>
      </div>
      <style jsx>{`
        .main-movie-card {
          text-align: center;
          margin-bottom: 20px;
          width: 100%;
        }
        .content {
          display: flex;
          align-items: flex-start;
          justify-content: center;
        }
        .poster {
          width: 200px; /* Adjusted size for poster */
          height: auto;
          margin-right: 20px; /* Added margin to separate poster from details */
        }
        .details {
          flex: 1;
          text-align: left; /* Aligning details to the left */
        }
        .title {
          margin-top: 0; /* Reset margin */
        }
        .overview {
          margin-top: 10px; /* Adjust margin for overview */
          overflow: hidden;
          word-wrap: break-word; /* Allow the overview to break into multiple lines */
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
          <MovieCard key={movie.id} movie={movie} recommendation={recommendation} />
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
    router.replace("/");
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
      <MainMovieCard movie={mainMovie as Movie} />
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
