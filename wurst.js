/* tslint:disable */
import * as wasm from './wurst_bg';

/**
* @returns {void}
*/
export function make() {
    return wasm.make();
}

const __widl_f_create_element_Document_target = Document.prototype.createElement || function() {
    throw new Error(`wasm-bindgen: Document.prototype.createElement does not exist`);
};

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

const slab = [{ obj: undefined }, { obj: null }, { obj: true }, { obj: false }];

let slab_next = slab.length;

function addHeapObject(obj) {
    if (slab_next === slab.length) slab.push(slab.length + 1);
    const idx = slab_next;
    const next = slab[idx];
    
    slab_next = next;
    
    slab[idx] = { obj, cnt: 1 };
    return idx << 1;
}

const stack = [];

function getObject(idx) {
    if ((idx & 1) === 1) {
        return stack[idx >> 1];
    } else {
        const val = slab[idx >> 1];
        
        return val.obj;
        
    }
}

const TextDecoder = typeof self === 'object' && self.TextDecoder
    ? self.TextDecoder
    : require('util').TextDecoder;

let cachedDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

export function __widl_f_create_element_Document(arg0, arg1, arg2, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {
        return addHeapObject(__widl_f_create_element_Document_target.call(getObject(arg0), varg1));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);
        
    }
}

function GetOwnOrInheritedPropertyDescriptor(obj, id) {
    while (obj) {
        let desc = Object.getOwnPropertyDescriptor(obj, id);
        if (desc) return desc;
        obj = Object.getPrototypeOf(obj);
    }
    throw new Error(`descriptor for id='${id}' not found`);
}

const __widl_f_body_Document_target = GetOwnOrInheritedPropertyDescriptor(Document.prototype, 'body').get || function() {
    throw new Error(`wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(Document.prototype, 'body').get does not exist`);
};

function isLikeNone(x) {
    return x === undefined || x === null;
}

export function __widl_f_body_Document(arg0) {
    
    const val = __widl_f_body_Document_target.call(getObject(arg0));
    return isLikeNone(val) ? 0 : addHeapObject(val);
    
}

const __widl_f_set_id_Element_target = GetOwnOrInheritedPropertyDescriptor(Element.prototype, 'id').set || function() {
    throw new Error(`wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(Element.prototype, 'id').set does not exist`);
};

export function __widl_f_set_id_Element(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    __widl_f_set_id_Element_target.call(getObject(arg0), varg1);
}

export function __widl_instanceof_HTMLElement(idx) {
    return getObject(idx) instanceof HTMLElement ? 1 : 0;
}

const __widl_f_set_title_HTMLElement_target = GetOwnOrInheritedPropertyDescriptor(HTMLElement.prototype, 'title').set || function() {
    throw new Error(`wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(HTMLElement.prototype, 'title').set does not exist`);
};

export function __widl_f_set_title_HTMLElement(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    __widl_f_set_title_HTMLElement_target.call(getObject(arg0), varg1);
}

const __widl_f_set_lang_HTMLElement_target = GetOwnOrInheritedPropertyDescriptor(HTMLElement.prototype, 'lang').set || function() {
    throw new Error(`wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(HTMLElement.prototype, 'lang').set does not exist`);
};

export function __widl_f_set_lang_HTMLElement(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    __widl_f_set_lang_HTMLElement_target.call(getObject(arg0), varg1);
}

const __widl_f_append_child_Node_target = Node.prototype.appendChild || function() {
    throw new Error(`wasm-bindgen: Node.prototype.appendChild does not exist`);
};

export function __widl_f_append_child_Node(arg0, arg1, exnptr) {
    try {
        return addHeapObject(__widl_f_append_child_Node_target.call(getObject(arg0), getObject(arg1)));
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);
        
    }
}

export function __widl_f_document_() {
    
    const val = (() => document)();
    return isLikeNone(val) ? 0 : addHeapObject(val);
    
}

function dropRef(idx) {
    
    idx = idx >> 1;
    if (idx < 4) return;
    let obj = slab[idx];
    
    obj.cnt -= 1;
    if (obj.cnt > 0) return;
    
    // If we hit 0 then free up our space in the slab
    slab[idx] = slab_next;
    slab_next = idx;
}

export function __wbindgen_object_drop_ref(i) {
    dropRef(i);
}

export function __wbindgen_number_get(n, invalid) {
    let obj = getObject(n);
    if (typeof(obj) === 'number') return obj;
    getUint8Memory()[invalid] = 1;
    return 0;
}

export function __wbindgen_is_null(idx) {
    return getObject(idx) === null ? 1 : 0;
}

export function __wbindgen_is_undefined(idx) {
    return getObject(idx) === undefined ? 1 : 0;
}

export function __wbindgen_boolean_get(i) {
    let v = getObject(i);
    if (typeof(v) === 'boolean') {
        return v ? 1 : 0;
    } else {
        return 2;
    }
}

export function __wbindgen_is_symbol(i) {
    return typeof(getObject(i)) === 'symbol' ? 1 : 0;
}

const TextEncoder = typeof self === 'object' && self.TextEncoder
    ? self.TextEncoder
    : require('util').TextEncoder;

let cachedEncoder = new TextEncoder('utf-8');

function passStringToWasm(arg) {
    
    const buf = cachedEncoder.encode(arg);
    const ptr = wasm.__wbindgen_malloc(buf.length);
    getUint8Memory().set(buf, ptr);
    return [ptr, buf.length];
}

export function __wbindgen_string_get(i, len_ptr) {
    let obj = getObject(i);
    if (typeof(obj) !== 'string') return 0;
    const [ptr, len] = passStringToWasm(obj);
    getUint32Memory()[len_ptr / 4] = len;
    return ptr;
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

