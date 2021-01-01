use age_core::format::{FileKey, Stanza};
use age_plugin::{
    identity::{self, Callbacks, IdentityPluginV1},
    print_new_identity,
    recipient::{self, RecipientPluginV1},
    run_state_machine,
};
use gumdrop::Options;
use secrecy::ExposeSecret;
use std::collections::HashMap;
use std::convert::TryInto;
use std::io;

const PLUGIN_NAME: &str = "trezor";
const RECIPIENT_TAG: &str = PLUGIN_NAME;

struct RecipientPlugin;

impl RecipientPluginV1 for RecipientPlugin {
    fn add_recipients<'a, I: Iterator<Item = &'a str>>(
        &mut self,
        recipients: I,
    ) -> Result<(), Vec<recipient::Error>> {
        let errors = recipients
            .enumerate()
            .filter_map(|(index, recipient)| {
                if recipient.contains(PLUGIN_NAME) {
                    // A real plugin would store the recipient here.
                    None
                } else {
                    Some(recipient::Error::Recipient {
                        index,
                        message: "invalid recipient".to_owned(),
                    })
                }
            })
            .collect::<Vec<_>>();
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn wrap_file_key(&mut self, file_key: &FileKey) -> Result<Vec<Stanza>, Vec<recipient::Error>> {
        // A real plugin would wrap the file key here.
        Ok(vec![Stanza {
            tag: RECIPIENT_TAG.to_owned(),
            args: vec!["does".to_owned(), "nothing".to_owned()],
            body: file_key.expose_secret().to_vec(),
        }])
    }
}

struct IdentityPlugin;

impl IdentityPluginV1 for IdentityPlugin {
    fn add_identities<'a, I: Iterator<Item = &'a str>>(
        &mut self,
        identities: I,
    ) -> Result<(), Vec<identity::Error>> {
        let errors = identities
            .enumerate()
            .filter_map(|(index, identity)| {
                if identity.contains(&PLUGIN_NAME.to_uppercase()) {
                    // A real plugin would store the identity.
                    None
                } else {
                    Some(identity::Error::Identity {
                        index,
                        message: "invalid identity".to_owned(),
                    })
                }
            })
            .collect::<Vec<_>>();
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn unwrap_file_keys(
        &mut self,
        files: Vec<Vec<Stanza>>,
        mut callbacks: impl Callbacks,
    ) -> io::Result<HashMap<usize, Result<FileKey, Vec<identity::Error>>>> {
        let mut file_keys = HashMap::with_capacity(files.len());
        for (file_index, stanzas) in files.into_iter().enumerate() {
            for stanza in stanzas {
                let _ = callbacks.message(&format!("s: {:?}", stanza));
                if stanza.tag == RECIPIENT_TAG {
                    // A real plugin would attempt to unwrap the file key with the stored
                    // identities.
                    // let _ = callbacks.message("This identity does nothing!")?;
                    file_keys.entry(file_index).or_insert(Ok(FileKey::from(
                        TryInto::<[u8; 16]>::try_into(&stanza.body[..]).unwrap(),
                    )));
                    break;
                }
            }
        }
        Ok(file_keys)
    }
}

#[derive(Debug, Options)]
struct PluginOptions {
    #[options(help = "print help message")]
    help: bool,

    #[options(help = "SLIP-0017 identity")]
    identity: Option<String>,

    #[options(help = "run the given age plugin state machine", no_short)]
    age_plugin: Option<String>,
}

fn main() -> io::Result<()> {
    let opts = PluginOptions::parse_args_default_or_exit();

    if let Some(state_machine) = opts.age_plugin {
        run_state_machine(&state_machine, || RecipientPlugin, || IdentityPlugin)
    } else {
        // A real plugin would generate a new identity here.
        print_new_identity(PLUGIN_NAME, &[], &[]);
        Ok(())
    }
}
