"use client";

import * as React from "react";
import Box from "@mui/material/Box";
import {
  Alert,
  AlertColor,
  AlertTitle,
  Button,
  ButtonGroup,
  Container,
} from "@mui/material";
import LayoutFlow from "@/components/graph";

export default function HomePage() {
  const [severity, setSeverity] = React.useState<AlertColor>("info");
  const [dot, setDot] = React.useState<string>("");
  const fetchDot = async () => {
    fetch("http://localhost:4000/dot")
      .then((resp) => resp.text())
      .then((data) => {
        setDot(data);
      });
  };
  React.useEffect(() => {
    const id = setInterval(async () => {
      fetchDot();
      if (dot.includes("Failed")) setSeverity("error");
    }, 500);
    fetchDot();
    return () => clearInterval(id);
  }, [dot]);
  const runWorkflow = async () => {
    setSeverity("warning");
    fetch("http://localhost:4000/run_all");
  };
  const resetWorkflow = async () => {
    fetch("http://localhost:4000/reset").then(() => {
      fetchDot();
      setSeverity("info");
    });
  };

  return (
    <Box
      sx={{
        display: "flex",
      }}
    >
      <Box sx={{}}>
        <ButtonGroup>
          <Button
            color={severity}
            variant="outlined"
            onClick={() => runWorkflow()}
          >
            Run workflow
          </Button>
          <Button color="info" variant="outlined" onClick={() => resetWorkflow()}>
            Reset
          </Button>
        </ButtonGroup>
        <Container maxWidth="sm">
          <Box sx={{ height: "80vh", width: "100vh" }}>
            {dot.length > 0 && <LayoutFlow dot={dot} />}
          </Box>
        </Container>
      </Box>
    </Box>
  );
}
