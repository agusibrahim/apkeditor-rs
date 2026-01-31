// APK Processing Web Worker
// This worker handles heavy APK operations off the main thread

// Worker message types
export interface WorkerRequest {
  id: string;
  type: 'init' | 'edit_apk';
  payload?: EditApkPayload;
}

export interface EditApkPayload {
  apkData: Uint8Array;
  packageName: string | null;
  appName: string | null;
  versionCode: number | null;
  versionName: string | null;
  useCustomKey: boolean;
  keystoreData?: Uint8Array;
  keystorePassword?: string;
  keyAlias?: string;
  keyPassword?: string;
}

export interface WorkerResponse {
  id: string;
  type: 'init_complete' | 'edit_complete' | 'progress' | 'error';
  success?: boolean;
  data?: Uint8Array;
  error?: string;
  progress?: number;
}

// WASM module references
let wasmInit: typeof import('../wasm/rsapkeditor.js').default;
let edit_apk: typeof import('../wasm/rsapkeditor.js').edit_apk;
let edit_apk_with_keystore: typeof import('../wasm/rsapkeditor.js').edit_apk_with_keystore;
let init_panic_hook: typeof import('../wasm/rsapkeditor.js').init_panic_hook;
let isInitialized = false;

// Handle messages from main thread
self.onmessage = async (e: MessageEvent<WorkerRequest>) => {
  const { id, type, payload } = e.data;

  try {
    switch (type) {
      case 'init':
        await initWasm();
        postMessage({ id, type: 'init_complete', success: true } as WorkerResponse);
        break;

      case 'edit_apk':
        if (!isInitialized) {
          await initWasm();
        }
        if (payload) {
          await processApk(id, payload);
        }
        break;

      default:
        postMessage({ id, type: 'error', error: 'Unknown message type' } as WorkerResponse);
    }
  } catch (err) {
    postMessage({
      id,
      type: 'error',
      error: err instanceof Error ? err.message : 'Unknown error',
    } as WorkerResponse);
  }
};

async function initWasm() {
  if (isInitialized) return;

  // Dynamic import of WASM module
  const wasm = await import('../wasm/rsapkeditor.js');
  wasmInit = wasm.default;
  edit_apk = wasm.edit_apk;
  edit_apk_with_keystore = wasm.edit_apk_with_keystore;
  init_panic_hook = wasm.init_panic_hook;

  // Get WASM binary URL
  const wasmBinary = await import('../wasm/rsapkeditor_bg.wasm?url');
  await wasmInit(wasmBinary.default);
  init_panic_hook();
  isInitialized = true;
}

async function processApk(id: string, payload: EditApkPayload) {
  const {
    apkData,
    packageName,
    appName,
    versionCode,
    versionName,
    useCustomKey,
    keystoreData,
    keystorePassword,
    keyAlias,
    keyPassword,
  } = payload;

  // Send progress update - starting
  postMessage({ id, type: 'progress', progress: 10 } as WorkerResponse);

  // Small delay to allow UI to update
  await new Promise(resolve => setTimeout(resolve, 0));

  // Send progress update - processing
  postMessage({ id, type: 'progress', progress: 30 } as WorkerResponse);

  let result;
  if (useCustomKey && keystoreData) {
    result = edit_apk_with_keystore(
      apkData,
      packageName,
      appName,
      versionCode,
      versionName,
      keystoreData,
      keystorePassword || '',
      keyAlias || null,
      keyPassword || null
    );
  } else {
    result = edit_apk(apkData, packageName, appName, versionCode, versionName);
  }

  // Send progress update - almost done
  postMessage({ id, type: 'progress', progress: 80 } as WorkerResponse);

  if (result.success) {
    const data = result.get_data();
    // Send progress update - complete
    postMessage({ id, type: 'progress', progress: 100 } as WorkerResponse);

    // Send the result with transferable ArrayBuffer for better performance
    postMessage(
      {
        id,
        type: 'edit_complete',
        success: true,
        data,
      } as WorkerResponse,
      { transfer: [data.buffer] }
    );
  } else {
    postMessage({
      id,
      type: 'edit_complete',
      success: false,
      error: result.error_message || 'Unknown error',
    } as WorkerResponse);
  }
}
