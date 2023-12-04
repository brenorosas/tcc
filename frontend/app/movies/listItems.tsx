import * as React from "react";
import ListItemButton from "@mui/material/ListItemButton";
import ListItemIcon from "@mui/material/ListItemIcon";
import ListItemText from "@mui/material/ListItemText";
import LocalMoviesIcon from "@mui/icons-material/LocalMovies";
import { List } from "@mui/material";

export default function MainListItems({
  moviesSelected,
  setMoviesSelected,
}: {
  moviesSelected: boolean;
  setMoviesSelected: React.Dispatch<React.SetStateAction<boolean>>;
}) {
  return (
    <List component="nav">
      <React.Fragment>
        <ListItemButton
          selected={moviesSelected}
          onClick={() => {
            setMoviesSelected(true);
          }}
        >
          <ListItemIcon>
            <LocalMoviesIcon />
          </ListItemIcon>
          <ListItemText primary="Movies" />
        </ListItemButton>
      </React.Fragment>
    </List>
  );
}
