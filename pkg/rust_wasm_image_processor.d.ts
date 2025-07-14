/* tslint:disable */
/* eslint-disable */
export function init(): void;
/**
 * 核心图像处理函数（同步版）
 */
export function process_image(data: Uint8Array): Uint8Array;
/**
 * 高级图像处理（支持滤镜链）
 */
export function processWithOptions(data: Uint8Array, options: ProcessOptions): Uint8Array;
/**
 * 异步获取并处理图片（返回 Promise）
 */
export function fetchAndProcess(url: string, options: ProcessOptions): Promise<any>;
export function async_process(url: string, options: ProcessOptions): Promise<Promise<any>>;
/**
 * 面向对象接口
 */
export class ImageProcessorWrapper {
  free(): void;
  constructor(data: Uint8Array);
  applyGrayscale(): void;
  getImageData(): Uint8Array;
}
export class ProcessOptions {
  free(): void;
  constructor();
  grayscale: any;
  sepia: any;
  invert: any;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly init: () => void;
  readonly process_image: (a: number, b: number) => [number, number, number, number];
  readonly processWithOptions: (a: number, b: number, c: number) => [number, number, number, number];
  readonly fetchAndProcess: (a: number, b: number, c: number) => any;
  readonly __wbg_imageprocessorwrapper_free: (a: number, b: number) => void;
  readonly imageprocessorwrapper_new: (a: number, b: number) => [number, number, number];
  readonly imageprocessorwrapper_applyGrayscale: (a: number) => [number, number];
  readonly imageprocessorwrapper_getImageData: (a: number) => [number, number, number, number];
  readonly __wbg_processoptions_free: (a: number, b: number) => void;
  readonly processoptions_new: () => number;
  readonly processoptions_set_grayscale: (a: number, b: any) => void;
  readonly processoptions_set_sepia: (a: number, b: any) => void;
  readonly processoptions_set_invert: (a: number, b: any) => void;
  readonly processoptions_grayscale: (a: number) => any;
  readonly processoptions_sepia: (a: number) => any;
  readonly processoptions_invert: (a: number) => any;
  readonly async_process: (a: number, b: number, c: number) => any;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_6: WebAssembly.Table;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly closure193_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure215_externref_shim: (a: number, b: number, c: any, d: any) => void;
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
