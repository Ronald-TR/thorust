"use client";
import "reactflow/dist/style.css";
import { blue, green, grey, red, yellow } from "@mui/material/colors";

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
    node: blue[50],
    edge: blue[500],
    border: blue[500],
    animated: true,
  },
  Completed: {
    node: green[50],
    edge: green[500],
    border: green[500],
    animated: false,
  },
  Failed: {
    node: red[50],
    edge: red[500],
    border: red[500],
    animated: true,
  },
  NotStarted: {
    node: yellow[50],
    edge: yellow[500],
    border: yellow[700],
    animated: true,
  },
  Skipped: {
    node: grey[50],
    edge: grey[500],
    border: grey[500],
    animated: true,
  },
  Default: {
    node: blue[50],
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
