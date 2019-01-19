use futures::{Async, Stream};
use reqwest::header::HeaderMap;
use samp_sdk::amx::{AmxResult, AMX};
use samp_sdk::consts::*;
use samp_sdk::types::Cell;

use method::Method;
use pool::Pool;
use request_client::{Request, RequestClient, Response};

pub struct Plugin {
    request_clients: Pool<RequestClient>,
}

define_native!(new_requests_client, endpoint: String, headers: i32);
define_native!(
    do_request,
    request_client_id: Cell,
    path: String,
    method: Method,
    callback: String,
    body: String,
    headers: Cell
);

impl Plugin {
    pub fn load(&self) -> bool {
        return true;
    }

    pub fn unload(&self) {
        return;
    }

    pub fn amx_load(&self, amx: &AMX) -> Cell {
        let natives = natives! {
            "RequestsClient" => new_requests_client,
            "Request" => do_request
        };
        // "RequestHeaders" => request_headers,
        // "RequestJSON" => request_json,
        // "WebSocketClient" => web_socket_client,
        // "WebSocketSend" => web_socket_send,
        // "JsonWebSocketClient" => json_web_socket_client,
        // "JsonWebSocketSend" => json_web_socket_send,
        // "JsonParse" => json_parse,
        // "JsonStringify" => json_stringify,
        // "JsonNodeType" => json_node_type,
        // "JsonObject" => json_object,
        // "JsonInt" => json_int,
        // "JsonBool" => json_bool,
        // "JsonFloat" => json_float,
        // "JsonString" => json_string,
        // "JsonArray" => json_array,
        // "JsonAppend" => json_append,
        // "JsonSetObject" => json_set_object,
        // "JsonSetInt" => json_set_int,
        // "JsonSetFloat" => json_set_float,
        // "JsonSetBool" => json_set_bool,
        // "JsonSetString" => json_set_string,
        // "JsonGetObject" => json_get_object,
        // "JsonGetInt" => json_get_int,
        // "JsonGetFloat" => json_get_float,
        // "JsonGetBool" => json_get_bool,
        // "JsonGetString" => json_get_string,
        // "JsonGetArray" => json_get_array,
        // "JsonArrayLength" => json_array_length,
        // "JsonArrayObject" => json_array_object,
        // "JsonGetNodeInt" => json_get_node_int,
        // "JsonGetNodeFloat" => json_get_node_float,
        // "JsonGetNodeBool" => json_get_node_bool,
        // "JsonGetNodeString" => json_get_node_string,
        // "JsonToggleGC" => json_toggle_gc,
        // "JsonCleanup" => json_cleanup,

        match amx.register(&natives) {
            Ok(_) => AMX_ERR_NONE,
            Err(err) => {
                log!("failed to register natives: {:?}", err);
                AMX_ERR_INIT
            }
        }
    }

    pub fn amx_unload(&self, _: &AMX) -> Cell {
        return AMX_ERR_NONE;
    }

    pub fn process_tick(&mut self) {
        for (_, rc) in self.request_clients.active.iter_mut() {
            let r: Async<Option<Response>> = match rc.poll() {
                Ok(v) => v,
                Err(e) => {
                    log!("{:?}", e);
                    return;
                }
            };

            if r.is_not_ready() {
                continue;
            }

            // let amx = rc.amx;

            r.map(|o| {
                let response: Response = match o {
                    Some(v) => v,
                    None => return,
                };

                println!("{}: {}", response.id, response.request.callback)

                // TODO:
                // exec_public!(
                //     amx,
                //     &response.request.callback;
                //     response.id,
                //     response.status);
            });
        }
    }

    // Natives

    pub fn new_requests_client(
        &mut self,
        amx: &AMX,
        endpoint: String,
        _headers: i32, // TODO
    ) -> AmxResult<Cell> {
        let header_map = HeaderMap::new();
        let rqc = RequestClient::new(amx, endpoint, header_map);
        Ok(self.request_clients.alloc(rqc))
    }

    pub fn do_request(
        &mut self,
        _: &AMX,
        request_client_id: Cell,
        path: String,
        method: Method,
        callback: String,
        _body: String,  // TODO
        _headers: Cell, // TODO
    ) -> AmxResult<Cell> {
        let client = match self.request_clients.get(request_client_id) {
            Some(v) => v,
            None => {
                return Ok(1);
            }
        };

        let header_map = HeaderMap::new();

        let id = match client.request(Request {
            callback: callback,
            path: path,
            method: Method::from(method),
            headers: header_map,
            request_type: 0,
        }) {
            Ok(v) => v,
            Err(e) => {
                log!("{}", e);
                return Ok(1);
            }
        };

        Ok(id)
    }
}

impl Default for Plugin {
    fn default() -> Self {
        {
            Plugin {
                request_clients: Pool::default(),
            }
        }
    }
}

// fn get_arg_ref<T: Clone>(amx: &AMX, parser: &mut args::Parser, out_ref: &mut T) -> i32 {
//     expand_args!(@amx, parser, tmp_ref: ref T);
//     *out_ref = tmp_ref.clone();
//     return 1;
// }

// fn get_arg_string(amx: &AMX, parser: &mut args::Parser, out_str: &mut String) -> i32 {
//     expand_args!(@amx, parser, tmp_str: String);
//     match samp_sdk::cp1251::decode_to(&tmp_str.into_bytes(), out_str) {
//         Ok(_) => {
//             return 1;
//         }
//         Err(e) => {
//             log!("{}", e);
//             return 0;
//         }
//     }
// }