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

const METHOD_RAFT_SERVICE_CHANGE_CONFIG: ::grpcio::Method<super::eraftpb::ConfChange, super::raft::ChangeReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/meteora.raft.RaftService/ChangeConfig",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_RAFT_SERVICE_SEND_MSG: ::grpcio::Method<super::eraftpb::Message, super::common::Null> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/meteora.raft.RaftService/SendMsg",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_RAFT_SERVICE_SEND_ADDRESS: ::grpcio::Method<super::raft::AddressState, super::common::Null> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/meteora.raft.RaftService/SendAddress",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct RaftServiceClient {
    client: ::grpcio::Client,
}

impl RaftServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        RaftServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn change_config_opt(&self, req: &super::eraftpb::ConfChange, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::raft::ChangeReply> {
        self.client.unary_call(&METHOD_RAFT_SERVICE_CHANGE_CONFIG, req, opt)
    }

    pub fn change_config(&self, req: &super::eraftpb::ConfChange) -> ::grpcio::Result<super::raft::ChangeReply> {
        self.change_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn change_config_async_opt(&self, req: &super::eraftpb::ConfChange, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::raft::ChangeReply>> {
        self.client.unary_call_async(&METHOD_RAFT_SERVICE_CHANGE_CONFIG, req, opt)
    }

    pub fn change_config_async(&self, req: &super::eraftpb::ConfChange) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::raft::ChangeReply>> {
        self.change_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_msg_opt(&self, req: &super::eraftpb::Message, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::common::Null> {
        self.client.unary_call(&METHOD_RAFT_SERVICE_SEND_MSG, req, opt)
    }

    pub fn send_msg(&self, req: &super::eraftpb::Message) -> ::grpcio::Result<super::common::Null> {
        self.send_msg_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_msg_async_opt(&self, req: &super::eraftpb::Message, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::Null>> {
        self.client.unary_call_async(&METHOD_RAFT_SERVICE_SEND_MSG, req, opt)
    }

    pub fn send_msg_async(&self, req: &super::eraftpb::Message) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::Null>> {
        self.send_msg_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_address_opt(&self, req: &super::raft::AddressState, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::common::Null> {
        self.client.unary_call(&METHOD_RAFT_SERVICE_SEND_ADDRESS, req, opt)
    }

    pub fn send_address(&self, req: &super::raft::AddressState) -> ::grpcio::Result<super::common::Null> {
        self.send_address_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_address_async_opt(&self, req: &super::raft::AddressState, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::Null>> {
        self.client.unary_call_async(&METHOD_RAFT_SERVICE_SEND_ADDRESS, req, opt)
    }

    pub fn send_address_async(&self, req: &super::raft::AddressState) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::common::Null>> {
        self.send_address_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait RaftService {
    fn change_config(&mut self, ctx: ::grpcio::RpcContext, req: super::eraftpb::ConfChange, sink: ::grpcio::UnarySink<super::raft::ChangeReply>);
    fn send_msg(&mut self, ctx: ::grpcio::RpcContext, req: super::eraftpb::Message, sink: ::grpcio::UnarySink<super::common::Null>);
    fn send_address(&mut self, ctx: ::grpcio::RpcContext, req: super::raft::AddressState, sink: ::grpcio::UnarySink<super::common::Null>);
}

pub fn create_raft_service<S: RaftService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_RAFT_SERVICE_CHANGE_CONFIG, move |ctx, req, resp| {
        instance.change_config(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_RAFT_SERVICE_SEND_MSG, move |ctx, req, resp| {
        instance.send_msg(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_RAFT_SERVICE_SEND_ADDRESS, move |ctx, req, resp| {
        instance.send_address(ctx, req, resp)
    });
    builder.build()
}
