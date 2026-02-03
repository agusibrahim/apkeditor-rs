/* tslint:disable */
/* eslint-disable */

/**
 * Result of APK editing operation
 */
export class ApkEditResult {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    get_data(): Uint8Array;
    readonly error_message: string;
    readonly success: boolean;
}

/**
 * APK information extracted from manifest
 */
export class ApkInfo {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    readonly app_name: string;
    readonly error_message: string;
    readonly package_name: string;
    readonly success: boolean;
    readonly version_code: number;
    readonly version_name: string;
}

/**
 * Debug: Dump manifest structure
 */
export function dump_manifest(apk_data: Uint8Array): string;

/**
 * Edit and sign an APK file from JavaScript with default debug key
 */
export function edit_apk(apk_data: Uint8Array, package_name?: string | null, app_name?: string | null, version_code?: number | null, version_name?: string | null, icon_data?: Uint8Array | null): ApkEditResult;

/**
 * Edit and sign an APK file from JavaScript with CUSTOM keystore (JKS or P12)
 */
export function edit_apk_with_keystore(apk_data: Uint8Array, package_name: string | null | undefined, app_name: string | null | undefined, version_code: number | null | undefined, version_name: string | null | undefined, keystore_data: Uint8Array, store_password: string, key_alias?: string | null, key_password?: string | null, icon_data?: Uint8Array | null): ApkEditResult;

/**
 * Extract icon from APK - returns PNG bytes of the highest resolution icon
 */
export function get_apk_icon(apk_data: Uint8Array): Uint8Array;

/**
 * Extract APK info (package name, app name, version) from APK bytes
 */
export function get_apk_info(apk_data: Uint8Array): ApkInfo;

/**
 * Get list of private key aliases from keystore
 */
export function get_keystore_aliases(keystore_data: Uint8Array, password: string): Array<any>;

/**
 * Get version info
 */
export function get_version(): string;

export function init_panic_hook(): void;

/**
 * Debug: List all files in APK (returns newline-separated list)
 */
export function list_apk_files(apk_data: Uint8Array): string;

/**
 * Validate a package name
 */
export function validate_package_name(name: string): boolean;

/**
 * Verify if key password is correct for a specific alias
 */
export function verify_key_password(keystore_data: Uint8Array, store_password: string, alias: string, key_password: string): boolean;

/**
 * Verify if the password is correct for the given keystore data (JKS or P12)
 */
export function verify_keystore_password(keystore_data: Uint8Array, password: string): boolean;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_apkeditresult_free: (a: number, b: number) => void;
    readonly apkeditresult_success: (a: number) => number;
    readonly apkeditresult_error_message: (a: number) => [number, number];
    readonly apkeditresult_get_data: (a: number) => any;
    readonly get_keystore_aliases: (a: number, b: number, c: number, d: number) => any;
    readonly init_panic_hook: () => void;
    readonly edit_apk: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => number;
    readonly edit_apk_with_keystore: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number) => number;
    readonly verify_key_password: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => number;
    readonly verify_keystore_password: (a: number, b: number, c: number, d: number) => number;
    readonly validate_package_name: (a: number, b: number) => number;
    readonly get_version: () => [number, number];
    readonly __wbg_apkinfo_free: (a: number, b: number) => void;
    readonly apkinfo_package_name: (a: number) => [number, number];
    readonly apkinfo_app_name: (a: number) => [number, number];
    readonly apkinfo_version_code: (a: number) => number;
    readonly apkinfo_version_name: (a: number) => [number, number];
    readonly apkinfo_success: (a: number) => number;
    readonly apkinfo_error_message: (a: number) => [number, number];
    readonly get_apk_info: (a: number, b: number) => number;
    readonly get_apk_icon: (a: number, b: number) => any;
    readonly list_apk_files: (a: number, b: number) => [number, number];
    readonly dump_manifest: (a: number, b: number) => [number, number];
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
