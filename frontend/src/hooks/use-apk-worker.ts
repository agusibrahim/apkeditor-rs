// Hook for using APK worker
import { useState, useCallback, useRef, useEffect } from 'react';
import type { EditApkPayload, WorkerResponse } from '@/workers/apk-worker';

interface UseApkWorkerReturn {
  isProcessing: boolean;
  progress: number;
  error: string | null;
  processApk: (payload: EditApkPayload) => Promise<Uint8Array | null>;
}

export function useApkWorker(): UseApkWorkerReturn {
  const [isProcessing, setIsProcessing] = useState(false);
  const [progress, setProgress] = useState(0);
  const [error, setError] = useState<string | null>(null);
  const workerRef = useRef<Worker | null>(null);
  const pendingRequestRef = useRef<{
    resolve: (data: Uint8Array | null) => void;
    reject: (error: Error) => void;
  } | null>(null);

  // Initialize worker on mount
  useEffect(() => {
    const worker = new Worker(
      new URL('../workers/apk-worker.ts', import.meta.url),
      { type: 'module' }
    );

    worker.onmessage = (e: MessageEvent<WorkerResponse>) => {
      const { type, success, data, error: errorMsg, progress: prog } = e.data;

      switch (type) {
        case 'progress':
          if (prog !== undefined) {
            setProgress(prog);
          }
          break;

        case 'edit_complete':
          setIsProcessing(false);
          if (success && data) {
            pendingRequestRef.current?.resolve(data);
          } else {
            setError(errorMsg || 'Processing failed');
            pendingRequestRef.current?.resolve(null);
          }
          pendingRequestRef.current = null;
          break;

        case 'error':
          setIsProcessing(false);
          setError(errorMsg || 'Worker error');
          pendingRequestRef.current?.reject(new Error(errorMsg));
          pendingRequestRef.current = null;
          break;
      }
    };

    worker.onerror = (e) => {
      console.error('Worker error:', e);
      setError('Worker error: ' + e.message);
      setIsProcessing(false);
      pendingRequestRef.current?.reject(new Error(e.message));
      pendingRequestRef.current = null;
    };

    workerRef.current = worker;

    // Initialize WASM in worker
    worker.postMessage({ id: 'init', type: 'init' });

    return () => {
      worker.terminate();
      workerRef.current = null;
    };
  }, []);

  const processApk = useCallback(async (payload: EditApkPayload): Promise<Uint8Array | null> => {
    if (!workerRef.current) {
      setError('Worker not initialized');
      return null;
    }

    setIsProcessing(true);
    setProgress(0);
    setError(null);

    return new Promise((resolve, reject) => {
      pendingRequestRef.current = { resolve, reject };

      const id = crypto.randomUUID();
      workerRef.current!.postMessage(
        {
          id,
          type: 'edit_apk',
          payload,
        },
        // Transfer the ArrayBuffer for better performance
        { transfer: [payload.apkData.buffer] }
      );
    });
  }, []);

  return { isProcessing, progress, error, processApk };
}
