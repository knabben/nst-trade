use log::Level;

use crate::blockchain::utils;
use crate::validator::SawtoothConnection;

use protobuf::Message as M;
use protobuf::RepeatedField;
use sawtooth_sdk::messages::validator::Message_MessageType;
use serde::ser;
use uuid::Uuid;
use sawtooth_sdk::signing;
use sawtooth_sdk::signing::{Context, PrivateKey, PublicKey};
use sawtooth_sdk::messages::client_batch_submit::{ClientBatchSubmitRequest};
use crate::blockchain::{serialize_payload, serialize_tp_payload};

pub struct BCTransaction {
    pub family_name: String,
    pub family_version: String,
    pub agent_prefix: String,
    pub context: Box<dyn Context>,
}

impl BCTransaction {
    pub fn new (
        family_name: String, family_version: String, agent_prefix: String
    ) -> BCTransaction {
        let context = signing::create_context("secp256k1").unwrap();
        BCTransaction {
            context: context, family_name: family_name, family_version: family_version,
            agent_prefix: agent_prefix
        }
    }

    // Context and key pair
    pub fn generate_key_pair(&self, context: &dyn Context) -> (Box<dyn PrivateKey>, Box<dyn PublicKey>){
        let private_key = context.new_random_private_key().unwrap();
        let public_key = context.get_public_key(&*private_key).unwrap();
        debug!("private_key: {:?}\npublic_key: {:?}", &*private_key.as_hex(), &*public_key.as_hex());
        (private_key, public_key)
    }

    pub fn store(
        &self, signer: signing::Signer, public_key: String, username: &str, sender: SawtoothConnection,
    ) {
        // Calculate agent address
        let hashed_family = utils::hashed_value(&self.family_name);
        let _namespace = &hashed_family[0..6];
        let hashed_pk = utils::hashed_value(&*public_key);

        // Agent address
        let agent_address = &format!("{}{}{}", _namespace, self.agent_prefix, &hashed_pk[0..62]);
        let agent_payload = serialize_payload(username.to_string());
        let batch = serialize_tp_payload(
            agent_payload,
            &*public_key,
            self,
            agent_address.to_string(),
            signer,
        );
        
        // Create submit request
        let mut submit_request = ClientBatchSubmitRequest::new();
        submit_request.set_batches(RepeatedField::from_vec(vec![batch]));

        // Protobuf writing
        let correlation_id = Uuid::new_v4().to_string();
        let msg_bytes = match protobuf::Message::write_to_bytes(&submit_request) {
            Ok(b) => b,
            Err(error) => {
                println!("Error serializing request: {:?}", error);
                return;
            },
        };

        // Send to ZeroMQ
        let mut future = match sender.get_sender().send(Message_MessageType::CLIENT_BATCH_SUBMIT_REQUEST, &correlation_id, &msg_bytes) {
            Ok(f) => f,
            Err(error) => {
                error!("Error unwrapping future: {:?}", error);
                return;
            },
        };
        
        let response_msg = match future.get() {
            Ok(m) => m,
            Err(error) => {
                error!("Error getting future: {:?}", error);
                return;
            },
        };
        
        println!("Client batch submit: {:?}", response_msg);
    }
}