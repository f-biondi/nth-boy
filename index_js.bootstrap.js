"use strict";
/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
(self["webpackChunkcreate_wasm_app"] = self["webpackChunkcreate_wasm_app"] || []).push([["index_js"],{

/***/ "../pkg/nth_boy_wasm.js":
/*!******************************!*\
  !*** ../pkg/nth_boy_wasm.js ***!
  \******************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   \"Emulator\": () => (/* reexport safe */ _nth_boy_wasm_bg_js__WEBPACK_IMPORTED_MODULE_0__.Emulator),\n/* harmony export */   \"__wbg_error_f851667af71bcfc6\": () => (/* reexport safe */ _nth_boy_wasm_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_error_f851667af71bcfc6),\n/* harmony export */   \"__wbg_new_abda76e883ba8a5f\": () => (/* reexport safe */ _nth_boy_wasm_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_new_abda76e883ba8a5f),\n/* harmony export */   \"__wbg_now_931686b195a14f9d\": () => (/* reexport safe */ _nth_boy_wasm_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_now_931686b195a14f9d),\n/* harmony export */   \"__wbg_set_wasm\": () => (/* reexport safe */ _nth_boy_wasm_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm),\n/* harmony export */   \"__wbg_stack_658279fe44541cf6\": () => (/* reexport safe */ _nth_boy_wasm_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_stack_658279fe44541cf6),\n/* harmony export */   \"__wbindgen_object_drop_ref\": () => (/* reexport safe */ _nth_boy_wasm_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_object_drop_ref),\n/* harmony export */   \"__wbindgen_throw\": () => (/* reexport safe */ _nth_boy_wasm_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_throw)\n/* harmony export */ });\n/* harmony import */ var _nth_boy_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./nth_boy_wasm_bg.wasm */ \"../pkg/nth_boy_wasm_bg.wasm\");\n/* harmony import */ var _nth_boy_wasm_bg_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./nth_boy_wasm_bg.js */ \"../pkg/nth_boy_wasm_bg.js\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([_nth_boy_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_1__]);\n_nth_boy_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = (__webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__)[0];\n\n\n(0,_nth_boy_wasm_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm)(_nth_boy_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_1__);\n\n\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });\n\n//# sourceURL=webpack://create-wasm-app/../pkg/nth_boy_wasm.js?");

/***/ }),

/***/ "../pkg/nth_boy_wasm_bg.js":
/*!*********************************!*\
  !*** ../pkg/nth_boy_wasm_bg.js ***!
  \*********************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   \"Emulator\": () => (/* binding */ Emulator),\n/* harmony export */   \"__wbg_error_f851667af71bcfc6\": () => (/* binding */ __wbg_error_f851667af71bcfc6),\n/* harmony export */   \"__wbg_new_abda76e883ba8a5f\": () => (/* binding */ __wbg_new_abda76e883ba8a5f),\n/* harmony export */   \"__wbg_now_931686b195a14f9d\": () => (/* binding */ __wbg_now_931686b195a14f9d),\n/* harmony export */   \"__wbg_set_wasm\": () => (/* binding */ __wbg_set_wasm),\n/* harmony export */   \"__wbg_stack_658279fe44541cf6\": () => (/* binding */ __wbg_stack_658279fe44541cf6),\n/* harmony export */   \"__wbindgen_object_drop_ref\": () => (/* binding */ __wbindgen_object_drop_ref),\n/* harmony export */   \"__wbindgen_throw\": () => (/* binding */ __wbindgen_throw)\n/* harmony export */ });\n/* module decorator */ module = __webpack_require__.hmd(module);\nlet wasm;\nfunction __wbg_set_wasm(val) {\n    wasm = val;\n}\n\n\nconst heap = new Array(128).fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nfunction getObject(idx) { return heap[idx]; }\n\nlet heap_next = heap.length;\n\nfunction dropObject(idx) {\n    if (idx < 132) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachedUint8Memory0 = null;\n\nfunction getUint8Memory0() {\n    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {\n        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);\n    }\n    return cachedUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nfunction passArray8ToWasm0(arg, malloc) {\n    const ptr = malloc(arg.length * 1);\n    getUint8Memory0().set(arg, ptr / 1);\n    WASM_VECTOR_LEN = arg.length;\n    return ptr;\n}\n\nlet cachedInt32Memory0 = null;\n\nfunction getInt32Memory0() {\n    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {\n        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);\n    }\n    return cachedInt32Memory0;\n}\n\nfunction getArrayU8FromWasm0(ptr, len) {\n    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);\n}\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n/**\n*/\nclass Emulator {\n\n    static __wrap(ptr) {\n        const obj = Object.create(Emulator.prototype);\n        obj.ptr = ptr;\n\n        return obj;\n    }\n\n    __destroy_into_raw() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n\n        return ptr;\n    }\n\n    free() {\n        const ptr = this.__destroy_into_raw();\n        wasm.__wbg_emulator_free(ptr);\n    }\n    /**\n    * @param {Uint8Array} rom\n    * @param {Uint8Array} ram\n    * @param {Uint8Array} rtc\n    * @returns {Emulator}\n    */\n    static new(rom, ram, rtc) {\n        const ptr0 = passArray8ToWasm0(rom, wasm.__wbindgen_malloc);\n        const len0 = WASM_VECTOR_LEN;\n        const ptr1 = passArray8ToWasm0(ram, wasm.__wbindgen_malloc);\n        const len1 = WASM_VECTOR_LEN;\n        const ptr2 = passArray8ToWasm0(rtc, wasm.__wbindgen_malloc);\n        const len2 = WASM_VECTOR_LEN;\n        const ret = wasm.emulator_new(ptr0, len0, ptr1, len1, ptr2, len2);\n        return Emulator.__wrap(ret);\n    }\n    /**\n    */\n    next_frame() {\n        wasm.emulator_next_frame(this.ptr);\n    }\n    /**\n    * @returns {number}\n    */\n    buffer() {\n        const ret = wasm.emulator_buffer(this.ptr);\n        return ret;\n    }\n    /**\n    * @returns {Uint8Array}\n    */\n    dump_ram() {\n        try {\n            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);\n            wasm.emulator_dump_ram(retptr, this.ptr);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            var v0 = getArrayU8FromWasm0(r0, r1).slice();\n            wasm.__wbindgen_free(r0, r1 * 1);\n            return v0;\n        } finally {\n            wasm.__wbindgen_add_to_stack_pointer(16);\n        }\n    }\n    /**\n    * @returns {Uint8Array}\n    */\n    dump_rtc() {\n        try {\n            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);\n            wasm.emulator_dump_rtc(retptr, this.ptr);\n            var r0 = getInt32Memory0()[retptr / 4 + 0];\n            var r1 = getInt32Memory0()[retptr / 4 + 1];\n            var v0 = getArrayU8FromWasm0(r0, r1).slice();\n            wasm.__wbindgen_free(r0, r1 * 1);\n            return v0;\n        } finally {\n            wasm.__wbindgen_add_to_stack_pointer(16);\n        }\n    }\n    /**\n    */\n    set_up() {\n        wasm.emulator_set_up(this.ptr);\n    }\n    /**\n    */\n    unset_up() {\n        wasm.emulator_unset_up(this.ptr);\n    }\n    /**\n    */\n    set_down() {\n        wasm.emulator_set_down(this.ptr);\n    }\n    /**\n    */\n    unset_down() {\n        wasm.emulator_unset_down(this.ptr);\n    }\n    /**\n    */\n    set_left() {\n        wasm.emulator_set_left(this.ptr);\n    }\n    /**\n    */\n    unset_left() {\n        wasm.emulator_unset_left(this.ptr);\n    }\n    /**\n    */\n    set_right() {\n        wasm.emulator_set_right(this.ptr);\n    }\n    /**\n    */\n    unset_right() {\n        wasm.emulator_unset_right(this.ptr);\n    }\n    /**\n    */\n    set_a() {\n        wasm.emulator_set_a(this.ptr);\n    }\n    /**\n    */\n    unset_a() {\n        wasm.emulator_unset_a(this.ptr);\n    }\n    /**\n    */\n    set_b() {\n        wasm.emulator_set_b(this.ptr);\n    }\n    /**\n    */\n    unset_b() {\n        wasm.emulator_unset_b(this.ptr);\n    }\n    /**\n    */\n    set_start() {\n        wasm.emulator_set_start(this.ptr);\n    }\n    /**\n    */\n    unset_start() {\n        wasm.emulator_unset_start(this.ptr);\n    }\n    /**\n    */\n    set_select() {\n        wasm.emulator_set_select(this.ptr);\n    }\n    /**\n    */\n    unset_select() {\n        wasm.emulator_unset_select(this.ptr);\n    }\n}\n\nfunction __wbg_new_abda76e883ba8a5f() {\n    const ret = new Error();\n    return addHeapObject(ret);\n};\n\nfunction __wbg_stack_658279fe44541cf6(arg0, arg1) {\n    const ret = getObject(arg1).stack;\n    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);\n    const len0 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len0;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\n\nfunction __wbg_error_f851667af71bcfc6(arg0, arg1) {\n    try {\n        console.error(getStringFromWasm0(arg0, arg1));\n    } finally {\n        wasm.__wbindgen_free(arg0, arg1);\n    }\n};\n\nfunction __wbindgen_object_drop_ref(arg0) {\n    takeObject(arg0);\n};\n\nfunction __wbg_now_931686b195a14f9d() {\n    const ret = Date.now();\n    return ret;\n};\n\nfunction __wbindgen_throw(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\n\n\n//# sourceURL=webpack://create-wasm-app/../pkg/nth_boy_wasm_bg.js?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var nth_boy_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! nth-boy-wasm */ \"../pkg/nth_boy_wasm.js\");\n/* harmony import */ var nth_boy_wasm_nth_boy_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! nth-boy-wasm/nth_boy_wasm_bg.wasm */ \"../pkg/nth_boy_wasm_bg.wasm\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([nth_boy_wasm__WEBPACK_IMPORTED_MODULE_0__, nth_boy_wasm_nth_boy_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_1__]);\n([nth_boy_wasm__WEBPACK_IMPORTED_MODULE_0__, nth_boy_wasm_nth_boy_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_1__] = __webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__);\n\n\n\nvar emulator = null;\nvar rom_name = null;\nvar startTime = performance.now();\nvar frames = 0;\n\nconst width = 160;\nconst height = 144;\nconst SCALE = 4;\nconst canvas = document.getElementById(\"lcd-canvas\");\nconst romSelect = document.getElementById(\"rom-select\");\nconst rom = document.getElementById(\"rom\");\nconst fps = document.getElementById(\"fps\");\nconst ctx = canvas.getContext('2d');\nconst palette = {\n    0x00: 0xFF000000,\n    0x55: 0xFF555555,\n    0xAA: 0xFFAAAAAA,\n    0xFF: 0xFFFFFFFF,\n};\ncanvas.height = SCALE * height;\ncanvas.width = SCALE * width;\n\nromSelect.addEventListener(\"click\", (e) => {\n    romSelect.blur();\n    rom.click();\n});\n\nrom.addEventListener(\"change\", (e) => {\n    if (rom.files.length > 0) {\n        let reader = new FileReader();\n        reader.readAsArrayBuffer(rom.files[0]);\n        reader.onload = function(){\n            let arrayBuffer = reader.result\n            let bytes = new Uint8Array(arrayBuffer);\n            saveData();\n            emulator = null;\n            setTimeout(() => {\n                rom_name = rom.files[0].name;\n                let ram = localStorage.getItem(rom_name + \".sav\");\n                let rtc = localStorage.getItem(rom_name + \".rtc\");\n                emulator = nth_boy_wasm__WEBPACK_IMPORTED_MODULE_0__.Emulator[\"new\"](\n                    bytes,\n                    ram != null ? new Uint8Array(JSON.parse(ram)) : new Uint8Array(),\n                    rtc != null ? new Uint8Array(JSON.parse(rtc)) : new Uint8Array(),\n                );\n                requestAnimationFrame(renderLoop);\n            }, 100);\n        }\n    }\n\n});\n\nwindow.addEventListener(\"beforeunload\", (e) => {\n    saveData();\n});\n\ndocument.addEventListener(\"keydown\", (e) => {\n  switch(e.key) {\n      case \"w\":\n        emulator.set_up();\n        break;\n      case \"s\":\n        emulator.set_down();\n        break;\n      case \"a\":\n        emulator.set_left();\n        break;\n      case \"d\":\n        emulator.set_right();\n        break;\n      case \"j\":\n        emulator.set_a();\n        break;\n      case \"k\":\n        emulator.set_b();\n        break;\n      case \"Enter\":\n        emulator.set_start();\n        break;\n      case \"Backspace\":\n        emulator.set_select();\n        break;\n  }\n});\n\ndocument.addEventListener(\"keyup\", (e) => {\n  switch(e.key) {\n      case \"w\":\n        emulator.unset_up();\n        break;\n      case \"s\":\n        emulator.unset_down();\n        break;\n      case \"a\":\n        emulator.unset_left();\n        break;\n      case \"d\":\n        emulator.unset_right();\n        break;\n      case \"j\":\n        emulator.unset_a();\n        break;\n      case \"k\":\n        emulator.unset_b();\n        break;\n      case \"Enter\":\n        emulator.unset_start();\n        break;\n      case \"Backspace\":\n        emulator.unset_select();\n        break;\n  }\n});\n\nconst saveData = () => {\n    if(emulator != null) {\n        let ram = emulator.dump_ram();\n        let rtc = emulator.dump_rtc();\n        if(ram.length > 0) { \n            localStorage.setItem(rom_name + \".sav\", JSON.stringify(Array.from(ram)));\n        }\n        if(rtc.length > 0) { \n            localStorage.setItem(rom_name + \".rtc\", JSON.stringify(Array.from(rtc)));\n        }\n    }\n};\n\nconst renderLoop = () => {\n  if (emulator != null) {\n      let startFrame = performance.now();\n      emulator.next_frame();\n\n      drawFrame();\n      frames++;\n\n      let now = performance.now();\n\n      if ((now-startTime) >= 1000) {\n          startTime = performance.now();\n          fps.innerHTML = frames + \" FPS\";\n          frames=0;\n      }\n\n      requestAnimationFrame(renderLoop);\n  }\n};\n\nconst drawFrame = () => {\n  const framePtr = emulator.buffer();\n  const pixels = new Uint8Array(nth_boy_wasm_nth_boy_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_1__.memory.buffer);\n  const imageData = ctx.createImageData(width*SCALE, height*SCALE);\n  const data = new Uint32Array(imageData.data.buffer);\n\n  for(let r=0; r<(height*SCALE); ++r) {\n    for(let c=0; c<(width*SCALE); ++c) {\n        let i = Math.floor(c/SCALE) + (Math.floor(r/SCALE) * width);\n        let color = palette[pixels[framePtr + (i * 4)]];\n        data[(r*width*SCALE) + c] = color;\n    }\n  }\n  \n  ctx.putImageData(imageData, 0, 0);\n};\n\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });\n\n//# sourceURL=webpack://create-wasm-app/./index.js?");

/***/ }),

/***/ "../pkg/nth_boy_wasm_bg.wasm":
/*!***********************************!*\
  !*** ../pkg/nth_boy_wasm_bg.wasm ***!
  \***********************************/
/***/ ((module, exports, __webpack_require__) => {

eval("/* harmony import */ var WEBPACK_IMPORTED_MODULE_0 = __webpack_require__(/*! ./nth_boy_wasm_bg.js */ \"../pkg/nth_boy_wasm_bg.js\");\nmodule.exports = __webpack_require__.v(exports, module.id, \"131c1023716eb631a70c\", {\n\t\"./nth_boy_wasm_bg.js\": {\n\t\t\"__wbg_new_abda76e883ba8a5f\": WEBPACK_IMPORTED_MODULE_0.__wbg_new_abda76e883ba8a5f,\n\t\t\"__wbg_stack_658279fe44541cf6\": WEBPACK_IMPORTED_MODULE_0.__wbg_stack_658279fe44541cf6,\n\t\t\"__wbg_error_f851667af71bcfc6\": WEBPACK_IMPORTED_MODULE_0.__wbg_error_f851667af71bcfc6,\n\t\t\"__wbindgen_object_drop_ref\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_object_drop_ref,\n\t\t\"__wbg_now_931686b195a14f9d\": WEBPACK_IMPORTED_MODULE_0.__wbg_now_931686b195a14f9d,\n\t\t\"__wbindgen_throw\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_throw\n\t}\n});\n\n//# sourceURL=webpack://create-wasm-app/../pkg/nth_boy_wasm_bg.wasm?");

/***/ })

}]);