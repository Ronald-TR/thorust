"use client";
import "reactflow/dist/style.css";
import { blue, green, grey, red, yellow } from "@mui/material/colors";
import dagre from "dagre";

const enum NodeStatus {
  Completed = "Completed",
  Failed = "Failed",
  Skipped = "Skipped",
  NotStarted = "NotStarted",
}

export interface INodeStyle {
  node: string;
  edge: string;
  border: string;
  animated: boolean;
}

export const NodeStyle = {
  Running: {
    node: blue[100],
    edge: blue[500],
    border: blue[500],
    animated: true,
  },
  Completed: {
    node: green[100],
    edge: green[500],
    border: green[500],
    animated: false,
  },
  Failed: {
    node: red[100],
    edge: red[500],
    border: red[500],
    animated: true,
  },
  NotStarted: {
    node: yellow[100],
    edge: yellow[500],
    border: yellow[700],
    animated: true,
  },
  Skipped: {
    node: grey[100],
    edge: grey[500],
    border: grey[500],
    animated: true,
  },
  Default: {
    node: blue[100],
    edge: blue[500],
    border: blue[500],
    animated: true,
  },
} as const;

export function getNodeStyleFromStatus(status: string): INodeStyle {
  switch (status) {
    case "Completed":
      return NodeStyle.Completed;
    case "Failed":
      return NodeStyle.Failed;
    case "Skipped":
      return NodeStyle.Skipped;
    case "NotStarted":
      return NodeStyle.NotStarted;
    case "Running":
      return NodeStyle.Running;
    default:
      return NodeStyle.Default;
  }
}

export function extractNodeInfo(nodeLabel: string): {
  label: string;
  status: string;
} {
  const x = nodeLabel.split("-");
  const status = x.pop() || "";
  const label = x.join("-");
  return {
    label,
    status,
  };
}

// customize edge based on the parent status.
export function customizeEdgeByParent(node: any): {
  animated: boolean;
  type: string;
  style: {
    stroke: string;
  };
} {
  let nodeStyle = getNodeStyleFromStatus(extractNodeInfo(node.label!).status);
  let animated = nodeStyle.animated;
  let type = "smoothstep";
  let style = {
    stroke: nodeStyle.edge,
  };
  return {
    animated,
    type,
    style,
  };
}

export function customizeNodeStyle(node: any): {
  style: {
    background: string;
    color: string;
    border: string;
  };
} {
  let nodeStyle = getNodeStyleFromStatus(extractNodeInfo(node.label!).status);

  let style = {
    background: nodeStyle.node,
    color: "dark",
    border: `2px solid ${nodeStyle.border}`,
  };
  return {
    style,
  };
}

const dagreGraph = new dagre.graphlib.Graph();
dagreGraph.setDefaultEdgeLabel(() => ({}));
const nodeWidth = 172;
const nodeHeight = 36;
// LR - left to right - horizontal
// TB - top to bottom - vertical
export const getLayoutedElements = (
  nodes: any,
  edges: any,
  direction = "LR",
  status: string[] = []
) => {
  // Check if the node status is in the filter status
  // If the filter status is empty, show all nodes and edges.
  nodes.forEach((node: any) => {
    node.hidden = !(status.includes(node.status) || status.length === 0);
  });
  edges.forEach((edge: any) => {
    edge.hidden = !(
      status.includes(edge.source.status) ||
      status.includes(edge.target.status) ||
      status.length === 0
    );
  });
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
