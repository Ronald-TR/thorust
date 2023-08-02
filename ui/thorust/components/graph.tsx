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

import "reactflow/dist/style.css";
import { read } from "graphlib-dot";
import {
  NodeStyle,
  customizeEdgeByParent,
  customizeNodeStyle,
  extractNodeInfo,
  getLayoutedElements,
} from "./helpers";
import {
  Avatar,
  Box,
  Button,
  ButtonGroup,
  Stack,
  Typography,
} from "@mui/material";
import BtnStatusGroup from "./ThemeRegistry/BtnStatusGroup";

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
        status: status,
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
  }, [dot]);
  return (
    <ReactFlow
      nodes={nodes}
      edges={edges}
      onNodesChange={onNodesChange}
      onEdgesChange={onEdgesChange}
      onConnect={onConnect}
      connectionLineType={ConnectionLineType.SmoothStep}
      fitView
      onNodeClick={(event, node) => console.log(event, node)}
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
        <BtnStatusGroup
          nodes={nodes}
          edges={edges}
          setNodes={setNodes}
          setEdges={setEdges}
        />
      </Panel>
      <Controls />
      <Background />
    </ReactFlow>
  );
};

export default LayoutFlow;
