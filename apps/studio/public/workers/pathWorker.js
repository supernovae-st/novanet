// Path calculation worker
// Offloads edge path calculations from the main thread for improved performance

self.onmessage = function(e) {
  const { sourceNode, targetNode, edgePadding = 16 } = e.data;

  // Calculate intersection points
  const sourcePoint = getNodeIntersection(
    sourceNode.center,
    sourceNode.width,
    sourceNode.height,
    targetNode.center,
    edgePadding
  );

  const targetPoint = getNodeIntersection(
    targetNode.center,
    targetNode.width,
    targetNode.height,
    sourceNode.center,
    edgePadding
  );

  // Generate curved path
  const dx = targetPoint.x - sourcePoint.x;
  const dy = targetPoint.y - sourcePoint.y;
  const distance = Math.sqrt(dx * dx + dy * dy);
  const curvature = Math.min(distance * 0.3, 100);

  const midX = (sourcePoint.x + targetPoint.x) / 2;
  const midY = (sourcePoint.y + targetPoint.y) / 2;

  const perpX = -dy / distance;
  const perpY = dx / distance;

  const controlX = midX + perpX * curvature;
  const controlY = midY + perpY * curvature;

  const edgePath = `M${sourcePoint.x},${sourcePoint.y} Q${controlX},${controlY} ${targetPoint.x},${targetPoint.y}`;
  const reversedPath = `M${targetPoint.x},${targetPoint.y} Q${controlX},${controlY} ${sourcePoint.x},${sourcePoint.y}`;

  self.postMessage({
    edgePath,
    reversedPath,
    edgeLength: distance,
    sourcePoint,
    targetPoint,
  });
};

function getNodeIntersection(nodeCenter, nodeWidth, nodeHeight, targetCenter, padding) {
  const dx = targetCenter.x - nodeCenter.x;
  const dy = targetCenter.y - nodeCenter.y;

  const halfWidth = nodeWidth / 2 + padding;
  const halfHeight = nodeHeight / 2 + padding;

  const absDx = Math.abs(dx);
  const absDy = Math.abs(dy);

  let ratio;
  if (absDx * halfHeight > absDy * halfWidth) {
    ratio = halfWidth / (absDx || 1);
  } else {
    ratio = halfHeight / (absDy || 1);
  }

  return {
    x: nodeCenter.x + dx * ratio,
    y: nodeCenter.y + dy * ratio,
  };
}
