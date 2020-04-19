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

const METHOD_RAFT_SERVICE_SNAPSHOT: ::grpcio::Method<super::eraftpb::Snapshot, super::raftpb::Null> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/kelpie.raft.RaftService/Snapshot",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_RAFT_SERVICE_CHANGE_CONFIG: ::grpcio::Method<super::eraftpb::ConfChange, super::raftpb::ChangeReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/kelpie.raft.RaftService/ChangeConfig",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_RAFT_SERVICE_SEND_MSG: ::grpcio::Method<super::eraftpb::Message, super::raftpb::Null> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/kelpie.raft.RaftService/SendMsg",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_RAFT_SERVICE_SEND_ADDRESS: ::grpcio::Method<super::raftpb::AddressState, super::raftpb::Null> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/kelpie.raft.RaftService/SendAddress",
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

    pub fn snapshot_opt(&self, req: &super::eraftpb::Snapshot, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::raftpb::Null> {
        self.client.unary_call(&METHOD_RAFT_SERVICE_SNAPSHOT, req, opt)
    }

    pub fn snapshot(&self, req: &super::eraftpb::Snapshot) -> ::grpcio::Result<super::raftpb::Null> {
        self.snapshot_opt(req, ::grpcio::CallOption::default())
    }

    pub fn snapshot_async_opt(&self, req: &super::eraftpb::Snapshot, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::raftpb::Null>> {
        self.client.unary_call_async(&METHOD_RAFT_SERVICE_SNAPSHOT, req, opt)
    }

    pub fn snapshot_async(&self, req: &super::eraftpb::Snapshot) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::raftpb::Null>> {
        self.snapshot_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn change_config_opt(&self, req: &super::eraftpb::ConfChange, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::raftpb::ChangeReply> {
        self.client.unary_call(&METHOD_RAFT_SERVICE_CHANGE_CONFIG, req, opt)
    }

    pub fn change_config(&self, req: &super::eraftpb::ConfChange) -> ::grpcio::Result<super::raftpb::ChangeReply> {
        self.change_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn change_config_async_opt(&self, req: &super::eraftpb::ConfChange, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::raftpb::ChangeReply>> {
        self.client.unary_call_async(&METHOD_RAFT_SERVICE_CHANGE_CONFIG, req, opt)
    }

    pub fn change_config_async(&self, req: &super::eraftpb::ConfChange) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::raftpb::ChangeReply>> {
        self.change_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_msg_opt(&self, req: &super::eraftpb::Message, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::raftpb::Null> {
        self.client.unary_call(&METHOD_RAFT_SERVICE_SEND_MSG, req, opt)
    }

    pub fn send_msg(&self, req: &super::eraftpb::Message) -> ::grpcio::Result<super::raftpb::Null> {
        self.send_msg_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_msg_async_opt(&self, req: &super::eraftpb::Message, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::raftpb::Null>> {
        self.client.unary_call_async(&METHOD_RAFT_SERVICE_SEND_MSG, req, opt)
    }

    pub fn send_msg_async(&self, req: &super::eraftpb::Message) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::raftpb::Null>> {
        self.send_msg_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_address_opt(&self, req: &super::raftpb::AddressState, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::raftpb::Null> {
        self.client.unary_call(&METHOD_RAFT_SERVICE_SEND_ADDRESS, req, opt)
    }

    pub fn send_address(&self, req: &super::raftpb::AddressState) -> ::grpcio::Result<super::raftpb::Null> {
        self.send_address_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_address_async_opt(&self, req: &super::raftpb::AddressState, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::raftpb::Null>> {
        self.client.unary_call_async(&METHOD_RAFT_SERVICE_SEND_ADDRESS, req, opt)
    }

    pub fn send_address_async(&self, req: &super::raftpb::AddressState) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::raftpb::Null>> {
        self.send_address_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait RaftService {
    fn snapshot(&mut self, ctx: ::grpcio::RpcContext, req: super::eraftpb::Snapshot, sink: ::grpcio::UnarySink<super::raftpb::Null>);
    fn change_config(&mut self, ctx: ::grpcio::RpcContext, req: super::eraftpb::ConfChange, sink: ::grpcio::UnarySink<super::raftpb::ChangeReply>);
    fn send_msg(&mut self, ctx: ::grpcio::RpcContext, req: super::eraftpb::Message, sink: ::grpcio::UnarySink<super::raftpb::Null>);
    fn send_address(&mut self, ctx: ::grpcio::RpcContext, req: super::raftpb::AddressState, sink: ::grpcio::UnarySink<super::raftpb::Null>);
}

pub fn create_raft_service<S: RaftService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_RAFT_SERVICE_SNAPSHOT, move |ctx, req, resp| {
        instance.snapshot(ctx, req, resp)
    });
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
