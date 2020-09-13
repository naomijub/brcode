const ffi = require('ffi');

const parse = (str) => {
    const library_name =  process.platform === "darwin" 
    ? './libbrcode.dylib'
    : './libbrcode.so';

    const  api = ffi.Library(library_name, {
        'json_from_brcode': ['string', ['string']]
    });
    return JSON.parse(api.json_from_brcode(str));
};

const emit = (json) => {
    const library_name =  process.platform === "darwin" 
    ? './libbrcode.dylib'
    : './libbrcode.so';

    const  api = ffi.Library(library_name, {
        'json_to_brcode': ['string', ['string']]
    });
    return api.json_to_brcode(JSON.stringify(json));
}
module.exports =  {
    parse, emit
}