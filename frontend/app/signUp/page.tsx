"use client";
import * as React from "react";
import Avatar from "@mui/material/Avatar";
import CssBaseline from "@mui/material/CssBaseline";
import TextField from "@mui/material/TextField";
import Link from "@mui/material/Link";
import Grid from "@mui/material/Grid";
import Box from "@mui/material/Box";
import LockOutlinedIcon from "@mui/icons-material/LockOutlined";
import Container from "@mui/material/Container";
import { createTheme, ThemeProvider } from "@mui/material/styles";
import { getCookie, setCookie } from "cookies-next";
import { useRouter } from "next/navigation";
import { Alert, AlertColor, Snackbar } from "@mui/material";
import api from "../api";
import { LoadingButton } from "@mui/lab";


// TODO remove, this demo shouldn't need to reset the theme.
const defaultTheme = createTheme();

export default function SignUp() {
  let authToken = getCookie("authToken");
  let router = useRouter();
  if (authToken) {
    router.replace("/movies");
  }

  let [openSnackBar, setOpenSnackBar] = React.useState(false);
  let [snackBarSeverity, setSnackBarSeverity] =
    React.useState<AlertColor>("success");
  let [snackBarMessage, setSnackBarMessage] = React.useState<string>("");
  let [isSignUpLoading, setIsSignUpLoading] = React.useState(false);

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    setIsSignUpLoading(true);
    event.preventDefault();
    const data = new FormData(event.currentTarget);
    const email = data.get("email");
    const password = data.get("password");
    const passwordConfirmation = data.get("passwordConfirmation");
    const apiUrl = process.env.NEXT_PUBLIC_API_URL;

    try {
      await api.post("/users/register", {
        email,
        password,
        passwordConfirmation,
      });
      setSnackBarSeverity("success");
      setSnackBarMessage("User created successfully");
      setOpenSnackBar(true);
      setTimeout(() => {
        router.push("/");
      }, 2000);
    } catch (error: any) {
      setSnackBarSeverity("error");
      setSnackBarMessage(error.response.data.ptBrMessage);
      setOpenSnackBar(true);
    }
    setIsSignUpLoading(false);
  };

  return (
    <ThemeProvider theme={defaultTheme}>
      <Container component="main" maxWidth="xs">
        <CssBaseline />
        <Box
          sx={{
            marginTop: 8,
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
          }}
        >
          <Avatar sx={{ m: 1, bgcolor: "secondary.main" }}>
            <LockOutlinedIcon />
          </Avatar>
          <Box
            component="form"
            noValidate
            onSubmit={handleSubmit}
            sx={{ mt: 3 }}
          >
            <Grid container spacing={2}>
              <Grid item xs={12}>
                <TextField
                  required
                  fullWidth
                  id="email"
                  label="Email"
                  name="email"
                  autoComplete="email"
                />
              </Grid>
              <Grid item xs={12}>
                <TextField
                  required
                  fullWidth
                  name="password"
                  label="Senha"
                  type="password"
                  id="password"
                  autoComplete="new-password"
                />
              </Grid>
              <Grid item xs={12}>
                <TextField
                  required
                  fullWidth
                  name="passwordConfirmation"
                  label="Confirmar senha"
                  type="password"
                  id="passwordConfirmation"
                  autoComplete="new-password-confirmation"
                />
              </Grid>
            </Grid>
            <LoadingButton
              loading={isSignUpLoading}
              type="submit"
              fullWidth
              variant="contained"
              sx={{ mt: 3, mb: 2 }}
            >
              Cadastrar
            </LoadingButton>
            <Grid container justifyContent="flex-end">
              <Grid item>
                <Link href="/" variant="body2">
                  Já tem uma conta? Faça login
                </Link>
              </Grid>
            </Grid>
          </Box>
        </Box>
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
    </ThemeProvider>
  );
}
