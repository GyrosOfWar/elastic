use std::marker::PhantomData;
use std::net::{ SocketAddr, SocketAddrV4, Ipv4Addr };

use crossbeam::sync::MsQueue;
use rotor::{ Notifier, Scope, GenericScope, Response, Void, Time, WakeupError };
use rotor::mio::tcp::TcpStream;
use rotor_http::client::{ Client, Request, Requester, Persistent, Connection, ResponseError, ProtocolError, Task, Head, RecvMode };

/// A request message.
pub struct Message;

/// A common message queue shared by multiple machines.
pub type Queue = MsQueue<Message>;

/// A client-side handle to sned request messages to a running loop.
pub struct Handle<'a> {
	queue: &'a Queue,
	notifiers: Vec<Notifier>
}

impl <'a> Handle<'a> {
	/// Create a new handle with no listeners.
	pub fn new(queue: &'a Queue) -> Self {
		Handle {
			queue: queue,
			notifiers: Vec::new()
		}
	}

	/// Add a machine as a listener on this handle's queue.
	pub fn add_listener(&mut self, notifier: Notifier) -> &'a Queue {
		self.notifiers.push(notifier);
		&self.queue
	}

	/// Push a message to the queue without blocking and notify listening machines.
	pub fn push(&self, msg: Message) {
		self.queue.push(msg);

		for notifier in &self.notifiers {
			notifier.wakeup();
		}
	}

	/// Try pop a message off the queue without blocking.
	pub fn pop(&self) -> Option<Message> {
		self.queue.try_pop()
	}
}

#[doc(hidden)]
pub struct Context;

/// A state machine for managing a persistent connection to an Elasticsearch node.
pub struct Fsm<'a, C> {
	queue: &'a Queue,
	_marker: PhantomData<C>
}

impl <'a, C> Client for Fsm<'a, C> {
	type Requester = RequestFsm<C>;
	type Seed = &'a Queue;

	fn create(seed: Self::Seed, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Self {
		Fsm {
			queue: seed,
			_marker: PhantomData
		}
	}

	fn connection_idle(self, _conn: &Connection, scope: &mut Scope<C>) -> Task<Self> {
		//Look for a message without blocking
		if let Some(msg) = self.queue.try_pop() {
			//Handle
		}
		else {
			//Snooze
		}

		unimplemented!()
	}

	fn wakeup(self, conn: &Connection, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Task<Self> {
		unimplemented!()
	}

	fn timeout(self, conn: &Connection, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Task<Self> {
		unimplemented!()
	}

	fn connection_error(self, err: &ProtocolError, _scope: &mut Scope<C>) {
		unimplemented!()
	}
}

/// A state machine for managing the HTTP component of an Elasticsearch connection.
pub struct RequestFsm<C> {
	_marker: PhantomData<C>
}

impl <C> Requester for RequestFsm<C> {
	type Context = C;

	fn prepare_request(self, req: &mut Request, _scope: &mut Scope<Self::Context>) -> Option<Self> {
		unimplemented!();
	}

	fn headers_received(self, head: Head, _request: &mut Request, scope: &mut Scope<Self::Context>) -> Option<(Self, RecvMode, Time)> {
		unimplemented!();
	}

	fn response_received(self, data: &[u8], _request: &mut Request, _scope: &mut Scope<Self::Context>) {
		unimplemented!();
	}

	fn bad_response(self, err: &ResponseError, _scope: &mut Scope<Self::Context>) {
		unimplemented!();
	}

	fn response_chunk(self, _chunk: &[u8], _request: &mut Request, _scope: &mut Scope<Self::Context>) -> Option<Self> {
		unimplemented!();
	}
	fn response_end(self, _request: &mut Request, _scope: &mut Scope<Self::Context>) {
		unimplemented!();
	}
	fn timeout(self, _request: &mut Request, _scope: &mut Scope<Self::Context>) -> Option<(Self, Time)> {
		unimplemented!();
	}
	fn wakeup(self, request: &mut Request, scope: &mut Scope<Self::Context>) -> Option<Self> {
		unimplemented!();
	}
}

/// Connect a persistent state machine to a node running on `localhost:9200`.
pub fn connect_localhost<S: GenericScope, C>(scope: &mut S, handle: &mut Handle<'static>) -> Response<Persistent<Fsm<'static, C>, TcpStream>, Void> {
	connect_addr(scope, SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9200)), handle)
}

/// Connect a persistent state machine to a node running at the given address.
pub fn connect_addr<S: GenericScope, C>(scope: &mut S, addr: SocketAddr, handle: &mut Handle<'static>) -> Response<Persistent<Fsm<'static, C>, TcpStream>, Void> {
	let queue = handle.add_listener(scope.notifier());

	Persistent::connect(scope, addr, queue)
}