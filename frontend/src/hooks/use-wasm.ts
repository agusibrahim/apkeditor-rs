import { useState, useCallback, useRef, useEffect } from 'react';
import init, {
  edit_apk,
  edit_apk_with_keystore,
  verify_keystore_password,
  verify_key_password,
  get_keystore_aliases,
  validate_package_name,
  get_version,
  init_panic_hook,
  get_apk_info as wasmGetApkInfo,
  get_apk_icon as wasmGetApkIcon,
  type ApkInfo as WasmApkInfo,
  type ApkEditResult,
} from '@/wasm/rsapkeditor.js';
import wasmUrl from '@/wasm/rsapkeditor_bg.wasm?url';

export interface ApkInfo {
  success: boolean;
  package_name: string;
  app_name: string;
  version_code: number;
  version_name: string;
  error_message?: string;
}

export interface EditResult {
  success: boolean;
  error_message?: string;
  get_data: () => Uint8Array;
}

export interface WasmModule {
  edit_apk: (data: Uint8Array, packageName: string | null, appName: string | null, versionCode: number | null, versionName: string | null) => EditResult;
  edit_apk_with_keystore: (data: Uint8Array, packageName: string | null, appName: string | null, versionCode: number | null, versionName: string | null, keystoreData: Uint8Array, storePassword: string, keyAlias: string | null, keyPassword: string | null) => EditResult;
  verify_keystore_password: (data: Uint8Array, password: string) => boolean;
  verify_key_password: (data: Uint8Array, storePassword: string, alias: string, keyPassword: string) => boolean;
  get_keystore_aliases: (data: Uint8Array, password: string) => string[];
  validate_package_name: (name: string) => boolean;
  get_version: () => string;
  init_panic_hook: () => void;
  get_apk_info: (data: Uint8Array) => ApkInfo;
  get_apk_icon: (data: Uint8Array) => Uint8Array;
}

interface UseWasmReturn {
  isLoading: boolean;
  isReady: boolean;
  error: string | null;
  version: string | null;
  module: WasmModule | null;
}

let globalModule: WasmModule | null = null;
let initPromise: Promise<WasmModule> | null = null;

function wrapApkInfo(wasmInfo: WasmApkInfo): ApkInfo {
  return {
    success: wasmInfo.success,
    package_name: wasmInfo.package_name,
    app_name: wasmInfo.app_name,
    version_code: wasmInfo.version_code,
    version_name: wasmInfo.version_name,
    error_message: wasmInfo.error_message || undefined,
  };
}

function wrapEditResult(result: ApkEditResult): EditResult {
  return {
    success: result.success,
    error_message: result.error_message || undefined,
    get_data: () => result.get_data(),
  };
}

async function loadWasmModule(): Promise<WasmModule> {
  // Initialize the WASM module
  await init(wasmUrl);

  // Initialize panic hook for better error messages
  init_panic_hook();

  // Create a wrapper module that matches the expected interface
  const module: WasmModule = {
    edit_apk: (data, packageName, appName, versionCode, versionName) => {
      const result = edit_apk(data, packageName, appName, versionCode, versionName);
      return wrapEditResult(result);
    },
    edit_apk_with_keystore: (data, packageName, appName, versionCode, versionName, keystoreData, storePassword, keyAlias, keyPassword) => {
      const result = edit_apk_with_keystore(data, packageName, appName, versionCode, versionName, keystoreData, storePassword, keyAlias, keyPassword);
      return wrapEditResult(result);
    },
    verify_keystore_password,
    verify_key_password,
    get_keystore_aliases: (data, password) => Array.from(get_keystore_aliases(data, password)),
    validate_package_name,
    get_version,
    init_panic_hook,
    get_apk_info: (data) => wrapApkInfo(wasmGetApkInfo(data)),
    get_apk_icon: wasmGetApkIcon,
  };

  return module;
}

export function useWasm(): UseWasmReturn {
  const [isLoading, setIsLoading] = useState(!globalModule);
  const [isReady, setIsReady] = useState(!!globalModule);
  const [error, setError] = useState<string | null>(null);
  const [version, setVersion] = useState<string | null>(null);
  const [module, setModule] = useState<WasmModule | null>(globalModule);

  useEffect(() => {
    if (globalModule) {
      setModule(globalModule);
      setIsReady(true);
      setIsLoading(false);
      try {
        setVersion(globalModule.get_version());
      } catch {}
      return;
    }

    if (!initPromise) {
      initPromise = loadWasmModule();
    }

    initPromise
      .then((wasmModule) => {
        globalModule = wasmModule;
        setModule(wasmModule);
        setVersion(wasmModule.get_version());
        setIsReady(true);
        setIsLoading(false);
      })
      .catch((err) => {
        console.error('WASM init error:', err);
        setError(err.message || 'Failed to load WASM module');
        setIsLoading(false);
      });
  }, []);

  return { isLoading, isReady, error, version, module };
}

export interface ApkFile {
  file: File;
  data: Uint8Array;
  info: ApkInfo | null;
  iconUrl: string | null;
}

interface UseApkReturn {
  apk: ApkFile | null;
  isLoading: boolean;
  error: string | null;
  loadApk: (file: File) => Promise<void>;
  clearApk: () => void;
}

export function useApk(wasmModule: WasmModule | null): UseApkReturn {
  const [apk, setApk] = useState<ApkFile | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const iconUrlRef = useRef<string | null>(null);

  const clearApk = useCallback(() => {
    if (iconUrlRef.current) {
      URL.revokeObjectURL(iconUrlRef.current);
      iconUrlRef.current = null;
    }
    setApk(null);
    setError(null);
  }, []);

  const loadApk = useCallback(async (file: File) => {
    if (!wasmModule) {
      setError('WASM module not loaded');
      return;
    }

    if (!file.name.toLowerCase().endsWith('.apk')) {
      setError('Please select a valid APK file');
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      const arrayBuffer = await file.arrayBuffer();
      const data = new Uint8Array(arrayBuffer);

      let info: ApkInfo | null = null;
      let iconUrl: string | null = null;

      try {
        info = wasmModule.get_apk_info(data);
      } catch (e) {
        console.error('Error reading APK info:', e);
      }

      try {
        const iconData = wasmModule.get_apk_icon(data);
        if (iconData.length > 0) {
          if (iconUrlRef.current) {
            URL.revokeObjectURL(iconUrlRef.current);
          }
          const blob = new Blob([iconData], { type: 'image/png' });
          iconUrl = URL.createObjectURL(blob);
          iconUrlRef.current = iconUrl;
        }
      } catch (e) {
        console.error('Error reading APK icon:', e);
      }

      setApk({ file, data, info, iconUrl });
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to load APK');
    } finally {
      setIsLoading(false);
    }
  }, [wasmModule]);

  return { apk, isLoading, error, loadApk, clearApk };
}
