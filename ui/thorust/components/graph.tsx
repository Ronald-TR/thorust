"use client";

import React, { useCallback, useEffect } from "react";
import ReactFlow, {
  addEdge,
  ConnectionLineType,
  Panel,
  useNodesState,
  useEdgesState,
  Controls,
  Background,
} from "reactflow";
import dagre from "dagre";

import "reactflow/dist/style.css";
import { read } from "graphlib-dot";
import {
  NodeStyle,
  customizeEdgeByParent,
  customizeNodeStyle,
  extractNodeInfo,
} from "./helpers";
import {
  Avatar,
  Box,
  Button,
  ButtonGroup,
  Stack,
  Typography,
} from "@mui/material";

const dagreGraph = new dagre.graphlib.Graph();
dagreGraph.setDefaultEdgeLabel(() => ({}));

const nodeWidth = 172;
const nodeHeight = 36;
// LR - left to right - horizontal
// TB - top to bottom - vertical
const getLayoutedElements = (nodes: any, edges: any, direction = "LR") => {
  const isHorizontal = direction === "LR";
  dagreGraph.setGraph({ rankdir: direction });

  nodes.forEach((node: any) => {
    dagreGraph.setNode(node.id, { width: nodeWidth, height: nodeHeight });
  });

  edges.forEach((edge: any) => {
    dagreGraph.setEdge(edge.source, edge.target);
  });

  dagre.layout(dagreGraph);

  nodes.forEach((node: any) => {
    const nodeWithPosition = dagreGraph.node(node.id);
    node.targetPosition = isHorizontal ? "left" : "top";
    node.sourcePosition = isHorizontal ? "right" : "bottom";

    // We are shifting the dagre node position (anchor=center center) to the top left
    // so it matches the React Flow node anchor point (top left).
    node.position = {
      x: nodeWithPosition.x - nodeWidth / 2,
      y: nodeWithPosition.y - nodeHeight / 2,
    };

    return node;
  });

  return { nodes, edges };
};

const LayoutFlow = ({ dot }: { dot: string }) => {
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);

  const onConnect = useCallback(
    (params: any) =>
      setEdges((eds) =>
        addEdge(
          { ...params, type: ConnectionLineType.SmoothStep, animated: true },
          eds
        )
      ),
    []
  );
  const onLayout = useCallback(
    (direction: any) => {
      const { nodes: layoutedNodes, edges: layoutedEdges } =
        getLayoutedElements(nodes, edges, direction);

      setNodes([...layoutedNodes]);
      setEdges([...layoutedEdges]);
    },
    [nodes, edges]
  );
  useEffect(() => {
    // Build the default nodes from the dot file.
    const graph = read(dot);
    const nodes = graph.nodes().map((nodeId, index) => {
      const node = graph.node(nodeId);
      const { label, status } = extractNodeInfo(node.label!);
      return {
        id: nodeId,
        data: { label: label },
        type: "default",
        ...customizeNodeStyle(node),
        position: {
          x: 0,
          y: 0,
        },
      };
    });

    const edges = graph.edges().map((edge) => ({
      id: `${edge.v}-${edge.w}`,
      source: edge.v,
      target: edge.w,
      ...customizeEdgeByParent(graph.node(edge.v)),
    }));

    const { nodes: layoutedNodes, edges: layoutedEdges } = getLayoutedElements(
      nodes,
      edges
    );
    setNodes([...layoutedNodes]);
    setEdges([...layoutedEdges]);
  }, []);
  return (
    <ReactFlow
      nodes={nodes}
      edges={edges}
      onNodesChange={onNodesChange}
      onEdgesChange={onEdgesChange}
      onConnect={onConnect}
      connectionLineType={ConnectionLineType.SmoothStep}
      fitView
    >
      <Panel position="bottom-right">
        <Typography variant="overline" display="block">
          layout orientation
        </Typography>
        <ButtonGroup
          orientation="vertical"
          variant="outlined"
          aria-label="outlined button group"
        >
          <Button onClick={() => onLayout("TB")}>vertical</Button>
          <Button onClick={() => onLayout("LR")}>horizontal</Button>
        </ButtonGroup>
      </Panel>
      <Panel position="top-left">
        <ButtonGroup size="small" aria-label="small button group">
          {Object.keys(NodeStyle)
            .filter((key) => key !== "Default")
            .map((key) => (
              <Button
                style={{ color: NodeStyle[key].border }}
                startIcon={
                  <Box
                    sx={{
                      width: 15,
                      height: 15,
                      backgroundColor: NodeStyle[key].border,
                    }}
                  />
                }
              >
                {key}
              </Button>
            ))}
        </ButtonGroup>
      </Panel>
      <Controls />
      <Background color="#ccc" />
    </ReactFlow>
  );
};

export default LayoutFlow;
