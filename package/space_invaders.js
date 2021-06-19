
let wasm;

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}
/**
*/
export class Invader {

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_invader_free(ptr);
    }
    /**
    * @returns {number}
    */
    get x() {
        var ret = wasm.__wbg_get_invader_x(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set x(arg0) {
        wasm.__wbg_set_invader_x(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get y() {
        var ret = wasm.__wbg_get_invader_y(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set y(arg0) {
        wasm.__wbg_set_invader_y(this.ptr, arg0);
    }
}
/**
*/
export class Invaders {

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_invaders_free(ptr);
    }
    /**
    * @returns {boolean}
    */
    all_killed() {
        var ret = wasm.invaders_all_killed(this.ptr);
        return ret !== 0;
    }
    /**
    * @returns {boolean}
    */
    reached_bottom() {
        var ret = wasm.invaders_reached_bottom(this.ptr);
        return ret !== 0;
    }
}
/**
*/
export class Player {

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_player_free(ptr);
    }
    /**
    */
    left() {
        wasm.player_left(this.ptr);
    }
    /**
    */
    right() {
        wasm.player_right(this.ptr);
    }
    /**
    * @returns {boolean}
    */
    shoot() {
        var ret = wasm.player_shoot(this.ptr);
        return ret !== 0;
    }
}
/**
*/
export class Render {

    static __wrap(ptr) {
        const obj = Object.create(Render.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_render_free(ptr);
    }
    /**
    * @returns {Render}
    */
    static new() {
        var ret = wasm.render_new();
        return Render.__wrap(ret);
    }
    /**
    * @returns {boolean}
    */
    detect_hits() {
        var ret = wasm.render_detect_hits(this.ptr);
        return ret !== 0;
    }
    /**
    */
    tick() {
        wasm.render_tick(this.ptr);
    }
}
/**
*/
export class Shot {

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_shot_free(ptr);
    }
    /**
    * @returns {number}
    */
    get x() {
        var ret = wasm.__wbg_get_shot_x(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set x(arg0) {
        wasm.__wbg_set_shot_x(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get y() {
        var ret = wasm.__wbg_get_shot_y(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set y(arg0) {
        wasm.__wbg_set_shot_y(this.ptr, arg0);
    }
    /**
    * @returns {boolean}
    */
    get exploding() {
        var ret = wasm.__wbg_get_shot_exploding(this.ptr);
        return ret !== 0;
    }
    /**
    * @param {boolean} arg0
    */
    set exploding(arg0) {
        wasm.__wbg_set_shot_exploding(this.ptr, arg0);
    }
    /**
    */
    explode() {
        wasm.shot_explode(this.ptr);
    }
    /**
    * @returns {boolean}
    */
    dead() {
        var ret = wasm.shot_dead(this.ptr);
        return ret !== 0;
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
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
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('space_invaders_bg.wasm', import.meta.url);
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }



    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

export default init;

