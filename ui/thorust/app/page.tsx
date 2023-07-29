import * as React from "react";
import Box from "@mui/material/Box";
import { Container } from "@mui/material";
import LayoutFlow from "@/components/graph";

export default function HomePage() {
  const dot = `digraph {
    0 [ label = "foo.test1-Completed" ]
    1 [ label = "foo.test2-NotStarted" ]
    2 [ label = "foo.test3-Failed" ]
    3 [ label = "foo.test4-Skipped" ]
    4 [ label = "foo.test5-Skipped" ]
    5 [ label = "foo.test6-Skipped" ]
    6 [ label = "foo.test7-Skipped" ]
    7 [ label = "bar.test1-Skipped" ]
    8 [ label = "bar.test2-NotStarted" ]
    1 -> 3 [ label = "0" ]
    2 -> 3 [ label = "1" ]
    0 -> 4 [ label = "2" ]
    2 -> 4 [ label = "3" ]
    3 -> 5 [ label = "4" ]
    4 -> 5 [ label = "5" ]
    3 -> 6 [ label = "6" ]
    4 -> 6 [ label = "7" ]
    6 -> 7 [ label = "8" ]
}`;
  return (
    <Box
      sx={{
        display: "flex",
      }}
    >
      <Box sx={{}}>
        {/* <Alert severity="info" sx={{ mt: 2, mb: 5 }}>
          <AlertTitle>Thorust</AlertTitle>
          Workflow
        </Alert> */}
        <Container maxWidth="sm">
          <Box sx={{ height: "80vh", width: "100vh" }}>
            <LayoutFlow dot={dot} />
            {/* <GraphvizPage dot={dot} /> */}
          </Box>
        </Container>
      </Box>
    </Box>
  );
}
