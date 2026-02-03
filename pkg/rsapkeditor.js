/* @ts-self-types="./rsapkeditor.d.ts" */

/**
 * Result of APK editing operation
 */
export class ApkEditResult {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ApkEditResult.prototype);
        obj.__wbg_ptr = ptr;
        ApkEditResultFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ApkEditResultFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_apkeditresult_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get error_message() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.apkeditresult_error_message(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {Uint8Array}
     */
    get_data() {
        const ret = wasm.apkeditresult_get_data(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {boolean}
     */
    get success() {
        const ret = wasm.apkeditresult_success(this.__wbg_ptr);
        return ret !== 0;
    }
}
if (Symbol.dispose) ApkEditResult.prototype[Symbol.dispose] = ApkEditResult.prototype.free;

/**
 * APK information extracted from manifest
 */
export class ApkInfo {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ApkInfo.prototype);
        obj.__wbg_ptr = ptr;
        ApkInfoFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ApkInfoFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_apkinfo_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get app_name() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.apkinfo_app_name(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    get error_message() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.apkinfo_error_message(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    get package_name() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.apkinfo_package_name(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {boolean}
     */
    get success() {
        const ret = wasm.apkinfo_success(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {number}
     */
    get version_code() {
        const ret = wasm.apkinfo_version_code(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {string}
     */
    get version_name() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.apkinfo_version_name(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) ApkInfo.prototype[Symbol.dispose] = ApkInfo.prototype.free;

/**
 * Debug: Dump manifest structure
 * @param {Uint8Array} apk_data
 * @returns {string}
 */
export function dump_manifest(apk_data) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passArray8ToWasm0(apk_data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.dump_manifest(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
    }
}

/**
 * Edit and sign an APK file from JavaScript with default debug key
 * @param {Uint8Array} apk_data
 * @param {string | null} [package_name]
 * @param {string | null} [app_name]
 * @param {number | null} [version_code]
 * @param {string | null} [version_name]
 * @param {Uint8Array | null} [icon_data]
 * @returns {ApkEditResult}
 */
export function edit_apk(apk_data, package_name, app_name, version_code, version_name, icon_data) {
    const ptr0 = passArray8ToWasm0(apk_data, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    var ptr1 = isLikeNone(package_name) ? 0 : passStringToWasm0(package_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    var ptr2 = isLikeNone(app_name) ? 0 : passStringToWasm0(app_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len2 = WASM_VECTOR_LEN;
    var ptr3 = isLikeNone(version_name) ? 0 : passStringToWasm0(version_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len3 = WASM_VECTOR_LEN;
    var ptr4 = isLikeNone(icon_data) ? 0 : passArray8ToWasm0(icon_data, wasm.__wbindgen_malloc);
    var len4 = WASM_VECTOR_LEN;
    const ret = wasm.edit_apk(ptr0, len0, ptr1, len1, ptr2, len2, isLikeNone(version_code) ? 0x100000001 : (version_code) >>> 0, ptr3, len3, ptr4, len4);
    return ApkEditResult.__wrap(ret);
}

/**
 * Edit and sign an APK file from JavaScript with CUSTOM keystore (JKS or P12)
 * @param {Uint8Array} apk_data
 * @param {string | null | undefined} package_name
 * @param {string | null | undefined} app_name
 * @param {number | null | undefined} version_code
 * @param {string | null | undefined} version_name
 * @param {Uint8Array} keystore_data
 * @param {string} store_password
 * @param {string | null} [key_alias]
 * @param {string | null} [key_password]
 * @param {Uint8Array | null} [icon_data]
 * @returns {ApkEditResult}
 */
export function edit_apk_with_keystore(apk_data, package_name, app_name, version_code, version_name, keystore_data, store_password, key_alias, key_password, icon_data) {
    const ptr0 = passArray8ToWasm0(apk_data, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    var ptr1 = isLikeNone(package_name) ? 0 : passStringToWasm0(package_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    var ptr2 = isLikeNone(app_name) ? 0 : passStringToWasm0(app_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len2 = WASM_VECTOR_LEN;
    var ptr3 = isLikeNone(version_name) ? 0 : passStringToWasm0(version_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len3 = WASM_VECTOR_LEN;
    const ptr4 = passArray8ToWasm0(keystore_data, wasm.__wbindgen_malloc);
    const len4 = WASM_VECTOR_LEN;
    const ptr5 = passStringToWasm0(store_password, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len5 = WASM_VECTOR_LEN;
    var ptr6 = isLikeNone(key_alias) ? 0 : passStringToWasm0(key_alias, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len6 = WASM_VECTOR_LEN;
    var ptr7 = isLikeNone(key_password) ? 0 : passStringToWasm0(key_password, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len7 = WASM_VECTOR_LEN;
    var ptr8 = isLikeNone(icon_data) ? 0 : passArray8ToWasm0(icon_data, wasm.__wbindgen_malloc);
    var len8 = WASM_VECTOR_LEN;
    const ret = wasm.edit_apk_with_keystore(ptr0, len0, ptr1, len1, ptr2, len2, isLikeNone(version_code) ? 0x100000001 : (version_code) >>> 0, ptr3, len3, ptr4, len4, ptr5, len5, ptr6, len6, ptr7, len7, ptr8, len8);
    return ApkEditResult.__wrap(ret);
}

/**
 * Extract icon from APK - returns PNG bytes of the highest resolution icon
 * @param {Uint8Array} apk_data
 * @returns {Uint8Array}
 */
export function get_apk_icon(apk_data) {
    const ptr0 = passArray8ToWasm0(apk_data, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.get_apk_icon(ptr0, len0);
    return ret;
}

/**
 * Extract APK info (package name, app name, version) from APK bytes
 * @param {Uint8Array} apk_data
 * @returns {ApkInfo}
 */
export function get_apk_info(apk_data) {
    const ptr0 = passArray8ToWasm0(apk_data, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.get_apk_info(ptr0, len0);
    return ApkInfo.__wrap(ret);
}

/**
 * Get list of private key aliases from keystore
 * @param {Uint8Array} keystore_data
 * @param {string} password
 * @returns {Array<any>}
 */
export function get_keystore_aliases(keystore_data, password) {
    const ptr0 = passArray8ToWasm0(keystore_data, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(password, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.get_keystore_aliases(ptr0, len0, ptr1, len1);
    return ret;
}

/**
 * Get version info
 * @returns {string}
 */
export function get_version() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.get_version();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

export function init_panic_hook() {
    wasm.init_panic_hook();
}

/**
 * Debug: List all files in APK (returns newline-separated list)
 * @param {Uint8Array} apk_data
 * @returns {string}
 */
export function list_apk_files(apk_data) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passArray8ToWasm0(apk_data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.list_apk_files(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
    }
}

/**
 * Validate a package name
 * @param {string} name
 * @returns {boolean}
 */
export function validate_package_name(name) {
    const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.validate_package_name(ptr0, len0);
    return ret !== 0;
}

/**
 * Verify if key password is correct for a specific alias
 * @param {Uint8Array} keystore_data
 * @param {string} store_password
 * @param {string} alias
 * @param {string} key_password
 * @returns {boolean}
 */
export function verify_key_password(keystore_data, store_password, alias, key_password) {
    const ptr0 = passArray8ToWasm0(keystore_data, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(store_password, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(alias, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len2 = WASM_VECTOR_LEN;
    const ptr3 = passStringToWasm0(key_password, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len3 = WASM_VECTOR_LEN;
    const ret = wasm.verify_key_password(ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3);
    return ret !== 0;
}

/**
 * Verify if the password is correct for the given keystore data (JKS or P12)
 * @param {Uint8Array} keystore_data
 * @param {string} password
 * @returns {boolean}
 */
export function verify_keystore_password(keystore_data, password) {
    const ptr0 = passArray8ToWasm0(keystore_data, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(password, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.verify_keystore_password(ptr0, len0, ptr1, len1);
    return ret !== 0;
}

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_throw_be289d5034ed271b: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_error_7534b8e9a36f1ab4: function(arg0, arg1) {
            let deferred0_0;
            let deferred0_1;
            try {
                deferred0_0 = arg0;
                deferred0_1 = arg1;
                console.error(getStringFromWasm0(arg0, arg1));
            } finally {
                wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
            }
        },
        __wbg_new_3eb36ae241fe6f44: function() {
            const ret = new Array();
            return ret;
        },
        __wbg_new_8a6f238a6ece86ea: function() {
            const ret = new Error();
            return ret;
        },
        __wbg_new_from_slice_a3d2629dc1826784: function(arg0, arg1) {
            const ret = new Uint8Array(getArrayU8FromWasm0(arg0, arg1));
            return ret;
        },
        __wbg_new_with_length_a2c39cbe88fd8ff1: function(arg0) {
            const ret = new Uint8Array(arg0 >>> 0);
            return ret;
        },
        __wbg_push_8ffdcb2063340ba5: function(arg0, arg1) {
            const ret = arg0.push(arg1);
            return ret;
        },
        __wbg_stack_0ed75d68575b0f3c: function(arg0, arg1) {
            const ret = arg1.stack;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbindgen_cast_0000000000000001: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./rsapkeditor_bg.js": import0,
    };
}

const ApkEditResultFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_apkeditresult_free(ptr >>> 0, 1));
const ApkInfoFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_apkinfo_free(ptr >>> 0, 1));

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasm;
function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('rsapkeditor_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
