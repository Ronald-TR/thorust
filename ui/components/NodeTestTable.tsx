"use client";

import * as React from "react";
import Box from "@mui/material/Box";
import Collapse from "@mui/material/Collapse";
import IconButton from "@mui/material/IconButton";
import Table from "@mui/material/Table";
import TableBody from "@mui/material/TableBody";
import TableCell from "@mui/material/TableCell";
import TableContainer from "@mui/material/TableContainer";
import TableHead from "@mui/material/TableHead";
import TableRow from "@mui/material/TableRow";
import Typography from "@mui/material/Typography";
import Paper from "@mui/material/Paper";
import KeyboardArrowDownIcon from "@mui/icons-material/KeyboardArrowDown";
import KeyboardArrowUpIcon from "@mui/icons-material/KeyboardArrowUp";
import {
  Container,
  Icon,
  List,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  ListSubheader,
  Toolbar,
} from "@mui/material";
import { ExpandLess, ExpandMore, Schema } from "@mui/icons-material";

interface ITestNode {
  id: number;
  name: string;
  description: string;
  test_id: string;
  service: string;
  history: [
    {
      from_status: string;
      to_status: string;
      from_created_at: string;
      to_created_at: string;
      duration_millis: number;
    }
  ];
}

function createData({
  id,
  name,
  description,
  test_id,
  service,
  history,
}: ITestNode) {
  return {
    id,
    name,
    description,
    test_id,
    service,
    history,
  };
}

function Row(props: { row: ReturnType<typeof createData> }) {
  const { row } = props;
  const [open, setOpen] = React.useState(false);

  return (
    <React.Fragment>
      <TableRow sx={{ "& > *": { borderBottom: "unset" } }}>
        <TableCell>
          <IconButton
            aria-label="expand row"
            size="small"
            onClick={() => setOpen(!open)}
          >
            {open ? <KeyboardArrowUpIcon /> : <KeyboardArrowDownIcon />}
          </IconButton>
        </TableCell>
        <TableCell align="left">{row.test_id}</TableCell>
        <TableCell align="center">{row.name}</TableCell>
        <TableCell align="center">{row.description}</TableCell>
        <TableCell align="center">{getTestDuration(row.history)}</TableCell>
      </TableRow>
      <TableRow>
        <TableCell style={{ paddingBottom: 0, paddingTop: 0 }} colSpan={6}>
          <Collapse in={open} timeout="auto" unmountOnExit>
            <Box sx={{ margin: 1 }}>
              <Typography variant="h6" gutterBottom component="div">
                History
              </Typography>
              <Table size="small" aria-label="purchases">
                <TableHead>
                  <TableRow>
                    <TableCell>From</TableCell>
                    <TableCell>To</TableCell>
                    <TableCell align="center">Duration</TableCell>
                    <TableCell align="center">From Date</TableCell>
                    <TableCell align="center">To Date</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {row.history.map((historyRow) => (
                    <TableRow key={historyRow.from_status}>
                      <TableCell component="th" scope="row">
                        {historyRow.from_status}
                      </TableCell>
                      <TableCell>{historyRow.to_status}</TableCell>
                      <TableCell align="center">
                        {(historyRow.duration_millis / 1000).toFixed(2)}s
                      </TableCell>
                      <TableCell align="center">
                        {historyRow.from_created_at}
                      </TableCell>
                      <TableCell align="center">
                        {historyRow.to_created_at}
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </Box>
          </Collapse>
        </TableCell>
      </TableRow>
    </React.Fragment>
  );
}

function getTestDuration(history: any) {
  const last = history[history.length - 1];
  if (["Completed", "Failed"].includes(last.to_status)) {
    return `${(last.duration_millis / 1000).toFixed(2)}s - ${last.to_status}`;
  };
  return 'No record - ${last.to_status}';
}
export default function NodeTestTable() {
  const [open, setOpen] = React.useState<{ [key: string]: boolean }>({});
  const [rows, setRows] = React.useState<ITestNode[]>([]);

  const handleClick = (index: string) => {
    if (!open[index]) setOpen({ [index]: true });
    setOpen({ [index]: !open[index] });
  };
  const services: { [key: string]: ITestNode[] } = rows.reduce(
    (acc: any, node: ITestNode) => {
      if (!acc[node.service]) {
        acc[node.service] = [];
      }
      acc[node.service].push(node);
      return acc;
    },
    {}
  );
  React.useEffect(() => {
    fetch("http://localhost:4000/nodes")
      .then((resp) => resp.json())
      .then((data) => {
        setRows(data.map((node: ITestNode) => createData(node)));
      });
  }, []);

  return (
    <List
      sx={{ width: "100%", bgcolor: "background.paper" }}
      subheader={
        <ListSubheader component="div" id="nested-list-subheader">
          Service Report
        </ListSubheader>
      }
    >
      {Object.entries(services).map(([service, nodes]) => (
        <List key={service}>
          <ListItemButton onClick={() => handleClick(service)}>
            <ListItemIcon>
              <Schema />
            </ListItemIcon>
            <ListItemText primary={service} />
            {open ? <ExpandLess /> : <ExpandMore />}
          </ListItemButton>
          <Collapse
            key={service}
            in={open[service]}
            timeout="auto"
            unmountOnExit
          >
            <TableContainer component={Paper}>
              <Table aria-label="collapsible table">
                <TableHead>
                  <TableRow>
                    <TableCell />
                    <TableCell align="left">Test ID</TableCell>
                    <TableCell align="center">Name</TableCell>
                    <TableCell align="center">Description</TableCell>
                    <TableCell align="center">Duration</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {nodes.map((row: ITestNode) => (
                    <Row key={row.name} row={createData(row)} />
                  ))}
                </TableBody>
              </Table>
            </TableContainer>
          </Collapse>
        </List>
      ))}
    </List>
  );
}
