import init, {fetchAndProcess, ProcessOptions} from './pkg/rust_wasm_image_processor.js';

let initialized = false;
self.onmessage = async (e) => {
    if (e.data.type === 'process_image') {
        try {
            if (!initialized) {
                await init();
                initialized = true;
            }

            const options = new ProcessOptions()

            options.grayscale = e.data.options.grayscale?1:0    ;
            options.invert = e.data.options.invert?1:0;
            options.sepia = e.data.options.sepia?1:0;
            const result = await fetchAndProcess(e.data.url,  options);
            self.postMessage({
                type: 'processed_image',
                buffer: result.buffer,
                mime: 'image/png'
            }, [result.buffer]);

        } catch (err) {
            self.postMessage({
                type: 'error',
                message: err.toString()
            });
        }
    }
};

// 支持 top-level await 的浏览器环境
if (typeof self !== 'undefined' && self.document === undefined) {
    console.log('Web Worker environment detected');
}
