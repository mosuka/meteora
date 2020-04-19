// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_KV_SERVICE_GET: ::grpcio::Method<super::kvpb::GetReq, super::kvpb::GetReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/kelpie.kv.KvService/Get",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KV_SERVICE_SET: ::grpcio::Method<super::kvpb::SetReq, super::kvpb::SetReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/kelpie.kv.KvService/Set",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KV_SERVICE_DELETE: ::grpcio::Method<super::kvpb::DeleteReq, super::kvpb::DeleteReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/kelpie.kv.KvService/Delete",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct KvServiceClient {
    client: ::grpcio::Client,
}

impl KvServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        KvServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_opt(&self, req: &super::kvpb::GetReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvpb::GetReply> {
        self.client.unary_call(&METHOD_KV_SERVICE_GET, req, opt)
    }

    pub fn get(&self, req: &super::kvpb::GetReq) -> ::grpcio::Result<super::kvpb::GetReply> {
        self.get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_async_opt(&self, req: &super::kvpb::GetReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvpb::GetReply>> {
        self.client.unary_call_async(&METHOD_KV_SERVICE_GET, req, opt)
    }

    pub fn get_async(&self, req: &super::kvpb::GetReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvpb::GetReply>> {
        self.get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_opt(&self, req: &super::kvpb::SetReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvpb::SetReply> {
        self.client.unary_call(&METHOD_KV_SERVICE_SET, req, opt)
    }

    pub fn set(&self, req: &super::kvpb::SetReq) -> ::grpcio::Result<super::kvpb::SetReply> {
        self.set_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_async_opt(&self, req: &super::kvpb::SetReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvpb::SetReply>> {
        self.client.unary_call_async(&METHOD_KV_SERVICE_SET, req, opt)
    }

    pub fn set_async(&self, req: &super::kvpb::SetReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvpb::SetReply>> {
        self.set_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_opt(&self, req: &super::kvpb::DeleteReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvpb::DeleteReply> {
        self.client.unary_call(&METHOD_KV_SERVICE_DELETE, req, opt)
    }

    pub fn delete(&self, req: &super::kvpb::DeleteReq) -> ::grpcio::Result<super::kvpb::DeleteReply> {
        self.delete_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_async_opt(&self, req: &super::kvpb::DeleteReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvpb::DeleteReply>> {
        self.client.unary_call_async(&METHOD_KV_SERVICE_DELETE, req, opt)
    }

    pub fn delete_async(&self, req: &super::kvpb::DeleteReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvpb::DeleteReply>> {
        self.delete_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait KvService {
    fn get(&mut self, ctx: ::grpcio::RpcContext, req: super::kvpb::GetReq, sink: ::grpcio::UnarySink<super::kvpb::GetReply>);
    fn set(&mut self, ctx: ::grpcio::RpcContext, req: super::kvpb::SetReq, sink: ::grpcio::UnarySink<super::kvpb::SetReply>);
    fn delete(&mut self, ctx: ::grpcio::RpcContext, req: super::kvpb::DeleteReq, sink: ::grpcio::UnarySink<super::kvpb::DeleteReply>);
}

pub fn create_kv_service<S: KvService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KV_SERVICE_GET, move |ctx, req, resp| {
        instance.get(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KV_SERVICE_SET, move |ctx, req, resp| {
        instance.set(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KV_SERVICE_DELETE, move |ctx, req, resp| {
        instance.delete(ctx, req, resp)
    });
    builder.build()
}
