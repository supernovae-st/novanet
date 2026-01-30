import { useEffect, useRef, useState, useCallback } from 'react';

interface PathWorkerInput {
  sourceNode: { center: { x: number; y: number }; width: number; height: number };
  targetNode: { center: { x: number; y: number }; width: number; height: number };
  edgePadding?: number;
}

interface PathWorkerOutput {
  edgePath: string;
  reversedPath: string;
  edgeLength: number;
  sourcePoint: { x: number; y: number };
  targetPoint: { x: number; y: number };
}

export function usePathWorker() {
  const workerRef = useRef<Worker | null>(null);
  const [result, setResult] = useState<PathWorkerOutput | null>(null);
  const pendingRef = useRef<((value: PathWorkerOutput) => void) | null>(null);

  useEffect(() => {
    workerRef.current = new Worker('/workers/pathWorker.js');

    workerRef.current.onmessage = (e: MessageEvent<PathWorkerOutput>) => {
      setResult(e.data);
      pendingRef.current?.(e.data);
      pendingRef.current = null;
    };

    return () => {
      workerRef.current?.terminate();
    };
  }, []);

  const calculatePath = useCallback((input: PathWorkerInput): Promise<PathWorkerOutput> => {
    return new Promise((resolve) => {
      pendingRef.current = resolve;
      workerRef.current?.postMessage(input);
    });
  }, []);

  return { calculatePath, result };
}

export type { PathWorkerInput, PathWorkerOutput };
