pub const HISTORY_WITH_DURATION_BETWEEN_STATUS: &str = "
SELECT
  node,
  from_status,
  to_status,
  created_at,
  next_created_at,
  (strftime('%s', next_created_at) - strftime('%s', created_at)) * 1000 AS time_difference_millis
FROM (
  SELECT
    id,
    node,
    status AS from_status,
    created_at,
    (SELECT created_at FROM node_history WHERE strftime('%s', created_at) > strftime('%s', nh.created_at) and node = nh.node ORDER BY created_at ASC LIMIT 1) AS next_created_at,
    (SELECT status FROM node_history WHERE strftime('%s', created_at) > strftime('%s', nh.created_at) and node = nh.node ORDER BY created_at ASC LIMIT 1) AS to_status
  FROM
    node_history AS nh
) AS OrderedHistory WHERE node = ?1;";

pub const ALL_HISTORY_WITH_DURATION_BETWEEN_STATUS: &str = "
SELECT
  node,
  from_status,
  to_status,
  created_at,
  next_created_at,
  (strftime('%s', next_created_at) - strftime('%s', created_at)) * 1000 AS time_difference_millis
FROM (
  SELECT
    id,
    node,
    status AS from_status,
    created_at,
    (SELECT created_at FROM node_history WHERE strftime('%s', created_at) > strftime('%s', nh.created_at) and node = nh.node ORDER BY created_at ASC LIMIT 1) AS next_created_at,
    (SELECT status FROM node_history WHERE strftime('%s', created_at) > strftime('%s', nh.created_at) and node = nh.node ORDER BY created_at ASC LIMIT 1) AS to_status
  FROM
    node_history AS nh
) AS OrderedHistory";
