import { Box, Button, ButtonGroup, Typography } from "@mui/material";
import React from "react";
import { NodeStyle, getLayoutedElements } from "../helpers";

export default function BtnStatusGroup({
  nodes,
  edges,
  setNodes,
  setEdges,
}: {
  nodes: any;
  edges: any;
  setNodes: any;
  setEdges: any;
}) {
  const [filterStatus, setFilterStatus] = React.useState<string[]>([]);
  const filterNodesByStatus = (status: string) => {
    const filter = () => {
      if (filterStatus.includes(status)) {
        return filterStatus.filter((s) => s !== status);
      } else {
        return [...filterStatus, status];
      }
    };
    let newFilter = filter();
    const { nodes: layoutedNodes, edges: layoutedEdges } = getLayoutedElements(
      nodes,
      edges,
      "LR",
      newFilter
    );
    setFilterStatus(newFilter);
    setNodes([...layoutedNodes]);
    setEdges([...layoutedEdges]);
    return;
  };
  return (
    <ButtonGroup size="small" aria-label="small button group">
      {Object.entries(NodeStyle)
        .filter(([key]) => key !== "Default")
        .map(([key, style]) => (
          <Button
            key={key}
            onClick={() => filterNodesByStatus(key)}
            style={{
              color: style.border,
              backgroundColor: filterStatus.includes(key)
                ? style.node
                : "white",
            }}
            startIcon={
              <Box
                sx={{
                  width: 17,
                  height: 17,
                  backgroundColor: style.border,
                }}
              >
                <Typography
                  sx={{
                    color: style.node,
                    fontSize: 12,
                    fontWeight: "bold",
                  }}
                >
                  {nodes.filter((node: any) => node.status === key).length}
                </Typography>
              </Box>
            }
          >
            {key}
          </Button>
        ))}
    </ButtonGroup>
  );
}
