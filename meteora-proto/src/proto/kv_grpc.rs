// This file is generated. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_KV_SERVICE_GET: ::grpcio::Method<super::kv::GetReq, super::kv::GetReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/meteora.kv.KvService/Get",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KV_SERVICE_SET: ::grpcio::Method<super::kv::SetReq, super::kv::SetReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/meteora.kv.KvService/Set",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KV_SERVICE_DELETE: ::grpcio::Method<super::kv::DeleteReq, super::kv::DeleteReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/meteora.kv.KvService/Delete",
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

    pub fn get_opt(&self, req: &super::kv::GetReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kv::GetReply> {
        self.client.unary_call(&METHOD_KV_SERVICE_GET, req, opt)
    }

    pub fn get(&self, req: &super::kv::GetReq) -> ::grpcio::Result<super::kv::GetReply> {
        self.get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_async_opt(&self, req: &super::kv::GetReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kv::GetReply>> {
        self.client.unary_call_async(&METHOD_KV_SERVICE_GET, req, opt)
    }

    pub fn get_async(&self, req: &super::kv::GetReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kv::GetReply>> {
        self.get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_opt(&self, req: &super::kv::SetReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kv::SetReply> {
        self.client.unary_call(&METHOD_KV_SERVICE_SET, req, opt)
    }

    pub fn set(&self, req: &super::kv::SetReq) -> ::grpcio::Result<super::kv::SetReply> {
        self.set_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_async_opt(&self, req: &super::kv::SetReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kv::SetReply>> {
        self.client.unary_call_async(&METHOD_KV_SERVICE_SET, req, opt)
    }

    pub fn set_async(&self, req: &super::kv::SetReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kv::SetReply>> {
        self.set_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_opt(&self, req: &super::kv::DeleteReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kv::DeleteReply> {
        self.client.unary_call(&METHOD_KV_SERVICE_DELETE, req, opt)
    }

    pub fn delete(&self, req: &super::kv::DeleteReq) -> ::grpcio::Result<super::kv::DeleteReply> {
        self.delete_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_async_opt(&self, req: &super::kv::DeleteReq, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kv::DeleteReply>> {
        self.client.unary_call_async(&METHOD_KV_SERVICE_DELETE, req, opt)
    }

    pub fn delete_async(&self, req: &super::kv::DeleteReq) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kv::DeleteReply>> {
        self.delete_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait KvService {
    fn get(&mut self, ctx: ::grpcio::RpcContext, req: super::kv::GetReq, sink: ::grpcio::UnarySink<super::kv::GetReply>);
    fn set(&mut self, ctx: ::grpcio::RpcContext, req: super::kv::SetReq, sink: ::grpcio::UnarySink<super::kv::SetReply>);
    fn delete(&mut self, ctx: ::grpcio::RpcContext, req: super::kv::DeleteReq, sink: ::grpcio::UnarySink<super::kv::DeleteReply>);
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
