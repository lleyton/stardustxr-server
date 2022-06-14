use super::scenegraph::Scenegraph;
use crate::nodes::field;
use crate::nodes::spatial;
use libstardustxr::messenger::Messenger;
use mio::net::UnixStream;
use std::rc::Rc;

pub struct Client {
	messenger: Messenger,
	scenegraph: Scenegraph,
}

impl Client {
	pub fn from_connection(connection: UnixStream) -> Rc<Self> {
		let client = Rc::new(Client {
			messenger: Messenger::new(connection),
			scenegraph: Default::default(),
		});
		client.scenegraph.set_client(&client);
		spatial::create_interface(client.clone());
		field::create_interface(client.clone());
		client
	}
	pub fn dispatch(&self) -> Result<(), std::io::Error> {
		self.messenger.dispatch(&self.scenegraph)
	}

	pub fn get_messenger(&self) -> &Messenger {
		&self.messenger
	}
	pub fn get_scenegraph(&self) -> &Scenegraph {
		&self.scenegraph
	}
}
